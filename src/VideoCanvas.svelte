<script>
	import { onMount, onDestroy } from 'svelte';
    import { videoTotalFrameLength, videoCurrentFrame, isVideoPaused, videoStartFrame, canvasSize, mediaSlot, mediaToBeImported, imgDrawOnCanvasIsA, imgDrawOnCanvasIsB, imgDrawOnCanvasIsDiff, imgDrawOnCanvasIsAB, abHandlePos } from './stores'
    import { isCanvasAutoReload, internalViewwerSize, isLoadFullImg, addrAndPort, limitCacheMb, usedCacheMb, videoFps, videoCurrentFps, bridgeHash} from "./stores";
    import { raw_images_a, raw_images_b } from "./MediaCache/mediaCache.js";
    import WorkerBuilder from "./Workers/worker-builder";
    import workerFile from "./Workers/cacheWorker";
    import axios from "axios";
    import { toast } from '@zerodevx/svelte-toast'
    import { notification_error, notification_success, notification_warning } from './Notifications/theme01'

    export let parentW;

    let frameCounter = 0;
    let rtimeElapsed02 = 0;
    let timeNow = Date.now();
    let timeThen = Date.now();
    let timeElapsed = Date.now();

    // Image variables
    let progressA = raw_images_a.progress;
    let progressB = raw_images_b.progress;

    let rawImageFramesDiff = [];

    // DOM obj variables
	let canvas;
	let context;

    // Sizes 
	let imgW = 0;
	let imgH = 0;

    let cW = 0;
    let cH = 0;

    // Time variables
	let lastFrameTime = 0;

    // Variables to set at pre-cached time.
    let imgTypeToLoadFrom = ""; //  [FROM_VIDEO | FROM_IMG]
    let vidImgResolution = [0, 0];

    onMount(() => {
		// prepare canvas stores
		context = canvas.getContext('2d');

        // Set Canva's size when mounted
        $canvasSize = [
            Math.floor(canvas.getBoundingClientRect().width),
            Math.floor(canvas.getBoundingClientRect().height)
        ];

        updateCanvas();
	});

    // Pre-cache Media once `mediaSlot` is changed
    const unsubscribe = mediaSlot.subscribe(async (value) => {

        // When a plate is selected in the modal, and then clicked the button `import`
        // this will Update the store $mediaSlot. This store is an array of two values
        // [mediaA, mediaB]

        // When there is at least one media to be cached
        if ((value[0] || value[1]) && $mediaToBeImported != ""){

            let currentMedia = null;    // media to be cached.

            let refObject = null;
            let tmpMediaToBeImported = $mediaToBeImported;

            if (tmpMediaToBeImported == 'A') {
                currentMedia = value[0];
                refObject = raw_images_a;
            }
            if (tmpMediaToBeImported == 'B') {
                currentMedia = value[1];
                refObject = raw_images_b;
            }

            $mediaToBeImported = "";
            
            try {

                // Clear old Data
                if (tmpMediaToBeImported == 'A'){
                    raw_images_a.clearAll();
                    $mediaSlot[1] = null;
                    $imgDrawOnCanvasIsA = true;
                    $imgDrawOnCanvasIsB = false;
                    $usedCacheMb = 0;
                }
                
                $imgDrawOnCanvasIsAB = false;
                $imgDrawOnCanvasIsDiff = false;
                raw_images_b.clearAll();
                
                $videoCurrentFrame = 0;
                rawImageFramesDiff = [];

                console.log("Data cleared!");

                // Clear canvas 
                context.clearRect(0, 0, canvas.width, canvas.height);

                // If it's a Image/images
                if (currentMedia.file_type == "IMG"){

                    console.log("It's a image");

                    // If it's a seq
                    if (currentMedia.seqLength > 1){
                        console.log("It's a image sequence");

                        let splitedName = currentMedia.name.match(/^(.+?)([0-9]+)-([0-9]+)\.(.{3,4})$/);

                        if (splitedName.length == 5){ // Just making sure

                            let imgFrom = parseInt(splitedName[2]);
                            let imgTo = parseInt(splitedName[3]);
                            let currentFrameLength = imgTo-imgFrom;

                            // Set the frame length and start frame using media A as the main media.
                            if (tmpMediaToBeImported == 'A'){
                                $videoTotalFrameLength = currentFrameLength;
                                $videoStartFrame = imgFrom;
                            }

                            // Make sure the length of Media B is the same as Media A.
                            if (tmpMediaToBeImported == 'B'){
                                if ($videoTotalFrameLength != currentFrameLength){
                                    console.log("Length are different");

                                    notification_error(`<strong>Error:</strong><br> You can only compare videos/images-seq with the same frame length.`)
                                    throw BreakError;
                                }
                            }

                            for (let x=0; x<=$videoTotalFrameLength; x++){
                                let preProName = currentMedia.path.match(/^(.+?)([0-9]+)\.(.{3,4})$/);
                                //let tmpImageName = preProName[1] + (''+(x+imgFrom)).padStart(3, '0') + '.' + preProName[3] //currentMedia.path.replace(/\\/g, '/');
                                let tmpImageName = preProName[1] + (''+(x+imgFrom)).padStart(preProName[2].length , '0') + '.' + preProName[3];
                                
                                tmpImageName = tmpImageName.replace(/\\/g, '/');

                                // Create the array of image paths
                                refObject.paths.push(tmpImageName);

                                // Create the array that will contain the right order of frames already cached
                                refObject.order.push(0);

                                // Update the bar cache status to 0 (non-cached)
                                refObject.pushToProgress(0);

                                if (tmpMediaToBeImported == 'B'){
                                    rawImageFramesDiff.push(null);
                                }
                            }
                        }
                    }
                    else{ // If not, it's a single image 
                        console.log("It's a single image");
                        console.log("CurrentMedia: ", currentMedia);

                        if (tmpMediaToBeImported == 'A'){
                            $videoTotalFrameLength = 0;
                            $videoStartFrame = 1;
                        }

                        // Create the array of image paths
                        refObject.paths.push(currentMedia.path);

                        // Create the array that will contain the right order of frames already cached
                        refObject.order.push(0);

                        // Update the bar cache status to 0 (non-cached)
                        refObject.pushToProgress(0);

                        if (tmpMediaToBeImported == 'B'){
                            if ($videoTotalFrameLength != 0){
                                console.log("Lengths are different");

                                notification_error(`<strong>Error:</strong><br> You can only compare videos/images-seq with the same frame length.`)
                                throw BreakError;
                            }

                            // 
                            rawImageFramesDiff.push(null);
                        }

                    }

                    imgTypeToLoadFrom = "FROM_IMG";
                }
                else{ // If don't then it's a video file.
                    console.log("It's a video file!");

                    // Get the metadata of the video
                    const data_from_rust = await axios.get('http://'+$addrAndPort+'/video_metadata', {
                        params: {
                            video_full_path: currentMedia.path,
                        },
                        headers: {
                            'Authorization': 'Bearer '+$bridgeHash,
                        }
                    }).catch(function (error) {
                        if (error.response) {
                            console.log(error.response);
                            notification_error(`<strong>Error:</strong><br>` + error.response.data);
                        }

                        throw error;
                    });

                    console.log("Metadata from video received!", data_from_rust);

                    if (tmpMediaToBeImported == 'A'){
                        console.log("Frame length: ", data_from_rust.data.frame_length);
                        $videoFps = !(isNaN(parseFloat(data_from_rust.data.fps))) ? data_from_rust.data.fps : $videoFps;
                        $videoTotalFrameLength = data_from_rust.data.frame_length - 1;
                        //$videoTotalFrameLength = 3; // Just for now. Remember to remove this !!
                        $videoStartFrame = 0;
                    }
                    if (tmpMediaToBeImported == 'B'){
                        if ($videoTotalFrameLength != data_from_rust.data.frame_length - 1){
                            console.log("Lengths are different");

                            notification_error(`<strong>Error:</strong><br> You can only compare videos/images-seq with the same frame length.`)
                            throw BreakError;
                        }
                    }

                    for (let x=0; x<=$videoTotalFrameLength; x++){

                        // Create the array of image paths
                        refObject.paths.push(currentMedia.path);

                        // Create the array that will contain the right order of frames already cached
                        refObject.order.push(0);

                        // Update the bar cache status to 0 (non-cached)
                        refObject.pushToProgress(0);

                        if (tmpMediaToBeImported == 'B'){
                            rawImageFramesDiff.push(null);
                        }
                    }

                    imgTypeToLoadFrom = "FROM_VIDEO";

                }

                // If it gets here it's because there was not any type of error or mismatch.
                console.log("Start Caching...");
            } catch (err) {

                //notification_warning("Cleaning up...");

                // Clear cache
                if (tmpMediaToBeImported == 'A'){
                    raw_images_a.clearAll();
                    $mediaSlot[0] = null;
                    $usedCacheMb = 0;
                }
                
                $mediaSlot[1] = null;
                raw_images_b.clearAll();
                rawImageFramesDiff = [];
                console.log(" # Error: ", err);
            }
        }
    });

    // TODO: code for auto recache
    // This function is executed everytime the window changes size
    const unsubIntViewerSize = internalViewwerSize.subscribe((interSize) => {

        // If auto-reload is ON
        if ($isCanvasAutoReload){

            console.log(interSize[0], ">", cW, " || ", interSize[1], '>', cH);

            // Only reload if the win was scale up
            // and it's no the first cache
            //if ( (interSize[0] > cW) || (interSize[1] > cH) && cW != 0 && cH != 0 ){}

        }
    });

    onDestroy(() => {
        unsubscribe();
        unsubIntViewerSize();
    });

    function cacheFrame(nFrame, cMediaSlot){

        let frameNumber = $videoStartFrame + nFrame;
        let seqImgPaths = null;
        let refObject = null;

        // number of images deleted. 
        // Useful to calculate the usedCacheMb.
        let cacheDelCount = 0;

        if (cMediaSlot == 'A'){
            refObject = raw_images_a;
        }

        if (cMediaSlot == 'B'){
            refObject = raw_images_b;
        }

        if ($usedCacheMb < $limitCacheMb){
    
            // Update the bar cache status to 1 (caching)
            refObject.setStatusAtFrame(1, nFrame);
            seqImgPaths = refObject.paths;

            // Keep count of the used cache.
            $usedCacheMb += (refObject.firstSize.width * refObject.firstSize.height) * 4 / 1000000;

            // Set image to the first image size.
            if (refObject.firstSize.width == 0 && refObject.firstSize.height == 0){
                refObject.firstSize.width = $canvasSize[0];
                refObject.firstSize.height = $canvasSize[1];
            }

            let queryParams = {
                src_img_type: imgTypeToLoadFrom,
                load_full_img: $isLoadFullImg,
                img_full_path: seqImgPaths[nFrame],
                frame_number: frameNumber,
                canvas_w: refObject.firstSize.width,
                canvas_h: refObject.firstSize.height
            }

            //console.log(" # queryParams: ", queryParams);

            // Use WebWorkers to avoid lagging the window while gettin the image data 
            if (window.Worker){
                let cacheWorker = new WorkerBuilder(workerFile);

                // Send request to a web worker.
                cacheWorker.postMessage([queryParams, $addrAndPort, $bridgeHash]);
                //console.log('Message posted to worker');

                // When data is received from the web worker.
                cacheWorker.onmessage = function(e) {
                    //console.log(e.data);
                    //console.log('Message received from worker');

                    if ( !e.data.error ){
                        // Push an array of the image's raw data into rawImageFrames
                        //let raw = e.data.image_raw_data;
                        let r_imgDimensions = e.data.img_dimensions;
                        let r_currentFrame = e.data.frame_number;

                        if (cMediaSlot == 'A'){
                            cW = r_imgDimensions.width;
                            cH = r_imgDimensions.height;
                        }

                        // Save the image's pixels and dimensions
                        refObject.imgs.push({
                            raw_data: e.data.image_raw_data,
                            dimensions: {...e.data.img_dimensions}
                        });

                        e.data.image_raw_data = null;
        
                        // Save the right order of frames
                        refObject.order[r_currentFrame - $videoStartFrame] = refObject.imgs.length - 1;
        
                        // Update the bar cache status to 2 (cached)
                        refObject.setStatusAtFrame(2, (r_currentFrame - $videoStartFrame));
                    }
                    else{ // There was a problem while getting the image data.

                        //notification_error(`<strong>Error:</strong><br> There was a problem while getting the image data.`)

                        let r_currentFrame = e.data.frame_number;

                        // Keep track of the used cache.
                        $usedCacheMb += (e.data.img_dimensions.width * e.data.img_dimensions.height) * 4 / 1000000;
        
                        // Update the bar cache status to 3 (error)
                        refObject.setStatusAtFrame(3, (r_currentFrame - $videoStartFrame));
                    }
                }

                return(cacheWorker);
            }
        }
        else{// If nor then clear some memory.

            if ($videoCurrentFrame >= $videoTotalFrameLength/2){
                for (let i=0; i<=$videoCurrentFrame-2; i++){
    
                    // If the frame has already been cached.
                    if ($progressA[i] == 2){
                        // Clear the cache of that frame.
                        raw_images_a.imgs[raw_images_a.order[i]] = null;
                        raw_images_a.setStatusAtFrame(0, i);
                        cacheDelCount++;
                    }

                    // If the frame in media B is cached then clear it as well.
                    if ($progressB[i] == 2){
                        // Clear the cache of that frame.
                        raw_images_b.imgs[raw_images_b.order[i]] = null;
                        raw_images_b.setStatusAtFrame(0, i);
                        cacheDelCount++;
                    }

                    if (cacheDelCount) break;
                }
            }
            else{
                for (let i=$videoTotalFrameLength; i>=0; i--){
                    // If the frame has already been cached.
                    if ($progressA[i] == 2){
                        // Clear the cache of that frame.
                        raw_images_a.imgs[raw_images_a.order[i]] = null;
                        raw_images_a.setStatusAtFrame(0, i);
                        cacheDelCount++;
                    }

                    // If the frame in media B is cached then clear it as well.
                    if ($progressB[i] == 2){
                        // Clear the cache of that frame.
                        raw_images_b.imgs[raw_images_b.order[i]] = null;
                        raw_images_b.setStatusAtFrame(0, i);
                        cacheDelCount++;
                    }

                    if (cacheDelCount) break;
                }
            }

            let tmpImgSize = ((raw_images_a.firstSize.width * raw_images_a.firstSize.height) * 4 / 1000000) * cacheDelCount;
            $usedCacheMb = $usedCacheMb - tmpImgSize;
            cacheDelCount = 0;
        }

        //console.log($usedCacheMb);

        return(null);
    }

    function bgCache(cMediaSlot){

        let refObject = null;
        let progressRef = null;

        if (cMediaSlot == 'A'){
            refObject = raw_images_a;
            progressRef = $progressA;
        }
        if (cMediaSlot == 'B'){
            refObject = raw_images_b;
            progressRef = $progressB;
        }

        // If there is any frame that has not been cached yet
        if (refObject.workers.length <= $videoTotalFrameLength){
            let cachedCounter = 0;

            // Go throw the progress bar of the current media.
            for (let i=$videoCurrentFrame; i<progressRef.length; i++) {

                if (progressRef[i] == 2) cachedCounter++;
                else{
                    if (progressRef[i] != 1){ //If its not caching yet.
                        if (cachedCounter > 0){
                            let nlCache = 5; // number of frames to cache on the background at one time.
                            if ($usedCacheMb >= $limitCacheMb) nlCache = 1;

                            for (let nc=1; nc<nlCache; nc++) {
                                if (progressRef[i-nc] == 2){
                                    refObject.workers[i] = cacheFrame(i, cMediaSlot);
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            if (cachedCounter == 0){
                if (progressRef[$videoCurrentFrame] != 1){
                    refObject.workers[$videoCurrentFrame] = cacheFrame($videoCurrentFrame, cMediaSlot);
                }
            }
        }
    }

    async function updateCanvas(time) {

        // If there is a Media A loaded.
        if (raw_images_a.order.length > 0){

            timeNow = Date.now();
            timeElapsed = timeNow - timeThen;
            let fpsInterval = 1000.0 / $videoFps;

            // 24 frames per second (1000/24fps = 41.66):
            // Update the canvas every X frames per second 
            if (timeElapsed > fpsInterval) {
                //console.log("tDiff: "+(time-tFps)+" "+$videoFps+" fps");

                $canvasSize = [
                    Math.floor(canvas.getBoundingClientRect().width),
                    Math.floor(canvas.getBoundingClientRect().height)
                ];

                if ($imgDrawOnCanvasIsA){

                    // Update canvas if the image is cached
                    if ($progressA[$videoCurrentFrame] == 2){

                        // TODO: Get the image size and update the variables
                        imgW = raw_images_a.imgs[raw_images_a.order[$videoCurrentFrame]].dimensions.width;
                        imgH = raw_images_a.imgs[raw_images_a.order[$videoCurrentFrame]].dimensions.height;
        
                        let currentImageData = new ImageData(raw_images_a.imgs[raw_images_a.order[$videoCurrentFrame]].raw_data, imgW, imgH);
            
                        if ($imgDrawOnCanvasIsAB){
                            // Update canvas if the image B is cached
                            if ($progressB[$videoCurrentFrame] == 2){
                                //context.clearRect(0, 0, canvas.width, canvas.height);

                                let scaleRatio = cW / $canvasSize[0];

                                let bW = 0;
                                let aW = $abHandlePos - ((parentW - $canvasSize[0]) / 2) ;
                                bW = aW;
                                aW = parseInt(aW * scaleRatio);
    
                                if (aW < 0) aW = 0;
                                //console.log("aW: " + aW);
    
                                // Draw Image A
                                context.putImageData(currentImageData, 0, 0, 0, 0, aW, imgH);
    
                                //currentImageData = new ImageData(raw_images_b.imgs[raw_images_b.order[$videoCurrentFrame]].raw_data, imgW, imgH);
                                currentImageData = new ImageData(
                                    raw_images_b.imgs[raw_images_b.order[$videoCurrentFrame]].raw_data,
                                    raw_images_b.imgs[raw_images_b.order[$videoCurrentFrame]].dimensions.width,
                                    raw_images_b.imgs[raw_images_b.order[$videoCurrentFrame]].dimensions.height
                                );
                                
                                bW = Math.abs(parseInt( ($canvasSize[0] - bW) * scaleRatio ));
                                
                                // Draw Image B
                                context.putImageData(currentImageData, 0, 0, aW, 0, bW, imgH);
                            }

                        } else{
                            context.putImageData(currentImageData, 0, 0);
                            //context.putImageData(currentImageData, 0, 0, 100, 0, imgW, imgH);
                        }

                        // Diff Code
                        if ($imgDrawOnCanvasIsDiff){
                            
                            // Update canvas if the image B is cached
                            if ($progressB[$videoCurrentFrame] == 2){
                                // If the diff image is not cached yet then start the computation
                                if (rawImageFramesDiff[$videoCurrentFrame] == null){
                                    // 
                                    //let imgCopyA = new Image();
                                    //imgCopyA.src = canvas.toDataURL();
        
                                    //let diffImageData = new ImageData(raw_images_b.imgs[raw_images_b.order[$videoCurrentFrame]].raw_data, imgW, imgH);
                                    let diffImageData = new ImageData(
                                        raw_images_b.imgs[raw_images_b.order[$videoCurrentFrame]].raw_data,
                                        raw_images_b.imgs[raw_images_b.order[$videoCurrentFrame]].dimensions.width,
                                        raw_images_b.imgs[raw_images_b.order[$videoCurrentFrame]].dimensions.height
                                    );

                                    diffImageData = await createImageBitmap(diffImageData);
                                    //context.putImageData(diffImageData, 0, 0);
        
                                    context.globalAlpha = 1;   // amount of FX
                                    context.globalCompositeOperation = "difference";
                                    context.drawImage(diffImageData, 0, 0);
                                    //context.drawImage(frame, left, top, vidW * scale, vidH * scale);
    
                                    //rawImageFramesDiff[$videoCurrentFrame] = new Image();
                                    //rawImageFramesDiff[$videoCurrentFrame].src = canvas.toDataURL();
    
                                } else{ // If not then just draw it
                                    context.drawImage(rawImageFramesDiff[$videoCurrentFrame], 0, 0);
                                }
                            }
                        }

                        bgCache('A');
                        if (raw_images_a.workers.length > $videoTotalFrameLength){
                            if (raw_images_b.isPreCached()) bgCache('B');
                        }

                    }
                    else{ // If the image is not cahed yet then cache it.
                        
                        // Cache the frame if it's not caching already
                        if ($progressA[$videoCurrentFrame] != 1){

                            // Get the worker in case we need to terminated later.
                            raw_images_a.workers[$videoCurrentFrame] = cacheFrame($videoCurrentFrame, 'A');
                        }
                    }
                }

                if ($imgDrawOnCanvasIsB){

                    // Update canvas if the image is cached
                    if ($progressB[$videoCurrentFrame] == 2){
                        // TODO: Get the image size and update the variables
                        imgW = raw_images_b.imgs[raw_images_b.order[$videoCurrentFrame]].dimensions.width;
                        imgH = raw_images_b.imgs[raw_images_b.order[$videoCurrentFrame]].dimensions.height;
        
                        let currentImageData = new ImageData(raw_images_b.imgs[raw_images_b.order[$videoCurrentFrame]].raw_data, imgW, imgH);
            
                        context.putImageData(currentImageData, 0, 0);

                        bgCache('B');
                    }
                    else{ // If the image is not cahed yet then cache it.
                        
                        // Cache the frame if it's not caching already
                        if ($progressB[$videoCurrentFrame] != 1){

                            // Get the worker in case we need to terminated later.
                            raw_images_b.workers[$videoCurrentFrame] = cacheFrame($videoCurrentFrame, 'B');
                        }
                    }
                }
                
                // If player is not paused then increment the frame number
                if (!($isVideoPaused)){

                    let tmpProgress = null;
                    if ($imgDrawOnCanvasIsA) tmpProgress = $progressA;
                    if ($imgDrawOnCanvasIsB) tmpProgress = $progressB;

                    // If the current frame is the last one then
                    if ($videoCurrentFrame == $videoTotalFrameLength){
                        // Select the FIRST CACHED frame.
                        for (let i=0; i<$videoTotalFrameLength; i++){

                            // If the Ab or the Diff mode is ON then make sure that both frames are cached.
                            if ($imgDrawOnCanvasIsAB || $imgDrawOnCanvasIsDiff){
                                if ($progressA[i] == 2 && $progressB[i]== 2){
                                    $videoCurrentFrame = i;
                                    break;
                                }
                            }
                            else if (tmpProgress[i] == 2){
                                $videoCurrentFrame = i;
                                break;
                            }
                        }
                    } else {

                        // If the next frame is already cached then select it.
                        let gotoNextFrame = false;

                        // If the AB or the Diff mode is ON then make sure that both frames are cached.
                        if ($imgDrawOnCanvasIsAB || $imgDrawOnCanvasIsDiff){
                            if ($progressA[$videoCurrentFrame + 1] == 2 && $progressB[$videoCurrentFrame + 1]== 2){
                                $videoCurrentFrame = $videoCurrentFrame + 1;
                                gotoNextFrame = true;
                            }
                        }
                        else if (tmpProgress[$videoCurrentFrame + 1] == 2){
                            $videoCurrentFrame = $videoCurrentFrame + 1;
                            gotoNextFrame = true;
                        }

                        // If the next frame is NOT cached yet then.
                        if (!gotoNextFrame){
                            // Look for the next cached frame.
                            let cachedFrameFound = false;

                            for (let i=$videoCurrentFrame+1; i<$videoTotalFrameLength; i++){
                                // If the AB or the Diff mode is ON then make sure that both frames are cached.
                                if ($imgDrawOnCanvasIsAB || $imgDrawOnCanvasIsDiff){
                                    if ($progressA[i] == 2 && $progressB[i]== 2){
                                        $videoCurrentFrame = i;
                                        cachedFrameFound = true;
                                    }
                                }
                                else if (tmpProgress[i] == 2){
                                    $videoCurrentFrame = i;
                                    cachedFrameFound = true;
                                }
                            }

                            if (!cachedFrameFound){
                                for (let i=0; i<$videoTotalFrameLength; i++){

                                    // If the AB or the Diff mode is ON then make sure that both frames are cached.
                                    if ($imgDrawOnCanvasIsAB || $imgDrawOnCanvasIsDiff){
                                        if ($progressA[i] == 2 && $progressB[i]== 2){
                                            $videoCurrentFrame = i;
                                            break;
                                        }
                                    }
                                    else if (tmpProgress[i] == 2){
                                        $videoCurrentFrame = i;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }

                frameCounter += 1;
                timeThen = timeNow - (timeElapsed % fpsInterval);
            }
        }

        // Calculate nad update the current fps.
        if (timeNow - rtimeElapsed02 >= 1000.0){
            $videoCurrentFps = Math.round(1000 / ((timeNow - rtimeElapsed02) / frameCounter) * 100) / 100;

            rtimeElapsed02 = timeNow;
            frameCounter = 0;
        }

        // Refresh canvas
        requestAnimationFrame(updateCanvas);
	}

</script>

<canvas
    bind:this={canvas}
    class="aspect-video h-full"
    width={cW}
    height={cH}
/>
