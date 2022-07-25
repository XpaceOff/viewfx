<script>
	import { onMount, onDestroy } from 'svelte';
    import { videoTotalFrameLength, videoCurrentFrame, isVideoPaused, videoStartFrame, canvasSize, mediaSlot, mediaToBeImported, imgDrawOnCanvasIsA, imgDrawOnCanvasIsB, imgDrawOnCanvasIsDiff, imgDrawOnCanvasIsAB, abHandlePos } from './stores'
    import { isCanvasAutoReload, internalViewwerSize, isLoadFullImg, addrAndPort } from "./stores";
    import { raw_images_a, raw_images_b } from "./MediaCache/mediaCache.js";
    import WorkerBuilder from "./Workers/worker-builder";
    import workerFile from "./Workers/cacheWorker";
    import axios from "axios";

    export let parentW;

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
        // TODO: all this function have to be re-written

        // When a plate is selected in the modal, and then clicked the button `import`
        // this will Update the store $mediaSlot. This store is an array of two values
        // [mediaA, mediaB]

        // When there is at least one media to be cached
        if ((value[0] || value[1]) && $mediaToBeImported != ""){

            try {
                let currentMedia = null;    // media to be cached.

                if ($mediaToBeImported == 'A') currentMedia = value[0];
                if ($mediaToBeImported == 'B') currentMedia = value[1];

                // Clear old Data
                if ($mediaToBeImported == 'A')
                    raw_images_a.clearAll();
                
                if ($mediaToBeImported == 'B'){
                    raw_images_b.clearAll();
                }
                
                $videoCurrentFrame = 0;
                rawImageFramesDiff = [];

                // Clear canvas 
                context.clearRect(0, 0, canvas.width, canvas.height);

                console.log(currentMedia.file_type);

                // If it's a Image/images
                if (currentMedia.file_type == "IMG"){

                    console.log("It's a image");

                    // If it's a seq
                    if (currentMedia.seqLength > 1){
                        console.log("It's a image sequence");

                        let splitedName = currentMedia.name.match(/^(.+?)([0-9]+)-([0-9]+)\.(.{3,4})$/);

                        if (splitedName.length == 5){ // Just making sure
                            console.log("HERE-03");

                            let imgFrom = parseInt(splitedName[2]);
                            let imgTo = parseInt(splitedName[3]);
                            let currentFrameLength = imgTo-imgFrom;

                            // Set the frame length and start frame using media A as the main media.
                            if ($mediaToBeImported == 'A'){
                                $videoTotalFrameLength = currentFrameLength;
                                $videoStartFrame = imgFrom;
                            }
                            if ($mediaToBeImported == 'B'){
                                if ($videoTotalFrameLength != currentFrameLength){
                                    console.log("Length are different");
                                    throw BreakError;
                                }
                            }

                            for (let x=0; x<=$videoTotalFrameLength; x++){
                                let preProName = currentMedia.path.match(/^(.+?)([0-9]+)\.(.{3,4})$/);
                                let tmpImageName = preProName[1] + (''+(x+imgFrom)).padStart(3, '0') + '.' + preProName[3] //currentMedia.path.replace(/\\/g, '/');
                                tmpImageName = tmpImageName.replace(/\\/g, '/');

                                if ($mediaToBeImported == 'A'){
                                    // Create the array of image paths
                                    raw_images_a.paths.push(tmpImageName);

                                    // Create the array that will contain the right order of frames already cached
                                    raw_images_a.order.push(0);

                                    // Update the bar cache status to 0 (non-cached)
                                    raw_images_a.pushToProgress(0);

                                    // 
                                    rawImageFramesDiff.push(null);
                                }

                                if ($mediaToBeImported == 'B'){
                                    // Create the array of image paths
                                    raw_images_b.paths.push(tmpImageName);

                                    // Create the array that will contain the right order of frames already cached
                                    raw_images_b.order.push(0);

                                    // Update the bar cache status to 0 (non-cached)
                                    raw_images_b.pushToProgress(0);
                                }

                            }
                        }
                    }
                    else{ // If not, it's a single image 
                        console.log("It's a single image");
                        console.log("CurrentMedia: ", currentMedia);

                        if ($mediaToBeImported == 'A'){
                            $videoTotalFrameLength = 0;
                            $videoStartFrame = 1;

                            // Create the array of image paths
                            raw_images_a.paths.push(currentMedia.path);

                            // Create the array that will contain the right order of frames already cached
                            raw_images_a.order.push(0);

                            // Update the bar cache status to 0 (non-cached)
                            raw_images_a.pushToProgress(0);

                            // 
                            rawImageFramesDiff.push(null);
                        }

                        if ($mediaToBeImported == 'B'){
                            if ($videoTotalFrameLength != 0){
                                console.log("Lengths are different");
                                throw BreakError;
                            }

                            // Create the array of image paths
                            raw_images_b.paths.push(currentMedia.path);

                            // Create the array that will contain the right order of frames already cached
                            raw_images_b.order.push(0);

                            // Update the bar cache status to 0 (non-cached)
                            raw_images_b.pushToProgress(0);
                        }

                    }

                    imgTypeToLoadFrom = "FROM_IMG";
                }
                else{ // If don't then it's a video file.
                    console.log("It's a video file!");
                    console.log(currentMedia.path);

                    try {

                        // Get the metadata of the video
                        const data_from_rust = await axios.get('http://'+$addrAndPort+'/video_metadata', {
                            params: {
                                video_full_path: currentMedia.path,
                            }
                        });

                        console.log("Metadata from video received!", data_from_rust);

                        if ($mediaToBeImported == 'A'){
                            console.log("Frame length: ", data_from_rust.data.frame_length);
                            $videoTotalFrameLength = data_from_rust.data.frame_length - 1;
                            //$videoTotalFrameLength = 3; // Just for now. Remember to remove this !!
                            $videoStartFrame = 0;
                        }
                        if ($mediaToBeImported == 'B'){
                            if ($videoTotalFrameLength != data_from_rust.data.frame_length - 1){
                                console.log("Lengths are different");
                                throw BreakError;
                            }
                        }

                        for (let x=0; x<=$videoTotalFrameLength; x++){

                            if ($mediaToBeImported == 'A'){
                                // Create the array of image paths
                                raw_images_a.paths.push(currentMedia.path);

                                // Create the array that will contain the right order of frames already cached
                                raw_images_a.order.push(0);

                                // Update the bar cache status to 0 (non-cached)
                                raw_images_a.pushToProgress(0);

                                // 
                                rawImageFramesDiff.push(null);
                            }

                            if ($mediaToBeImported == 'B'){
                                // Create the array of image paths
                                raw_images_b.paths.push(currentMedia.path);

                                // Create the array that will contain the right order of frames already cached
                                raw_images_b.order.push(0);

                                // Update the bar cache status to 0 (non-cached)
                                raw_images_b.pushToProgress(0);
                            }
                        }

                        imgTypeToLoadFrom = "FROM_VIDEO";

                    } catch (error) {

                        if (error == BreakError){
                            console.log("Frame mismatch error");
                        }
                        else{
                            console.log("Error reading the video. Maybe FFmpeg?");
                        }

                        // TODO: remember to handle this error and make sure that is bug free.
                        console.error(error);

                        throw error;
                    }

                    console.log("Data from rust loaded!");

                }

                // If it gets here it's because there was not any type of error or mismatch.
                console.log("Start Caching...");
                startCaching();
            } catch (err) {

                // Clear cache
                if ($mediaToBeImported == 'A'){
                    raw_images_a.clearAll();
                    $mediaSlot[0] = null;
                }
                
                if ($mediaToBeImported == 'B'){
                    raw_images_b.clearAll();
                    $mediaSlot[1] = null;
                    $imgDrawOnCanvasIsA = true;
                }
                
                rawImageFramesDiff = [];

                $mediaToBeImported = '';
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
            if ( (interSize[0] > cW) || (interSize[1] > cH) && cW != 0 && cH != 0 ){

                // Re-cache media A if it was loaded previously 
                if (raw_images_a.length > 0 ){
                    //$mediaToBeImported = 'A';

                    /*for (nFrame in $barFrameCacheStatusA){
                        $barFrameCacheStatusA[nFrame] = 0;
                    }*/

                    console.log("Cache A");
                    //startCaching();
                }
                
                // Re-cache media B if it was loaded previously 
                if (raw_images_b.imgs.length > 0){
                    //$mediaToBeImported = 'B';
                    console.log("Cache B");
                    //startCaching();
                }
            }
        }
    });

    onDestroy(() => {
        unsubscribe();
        unsubIntViewerSize();
    });

    async function startCaching(){
        let cMediaSlot = $mediaToBeImported;

        for (let nFrame=0; nFrame<=$videoTotalFrameLength; nFrame++){

            let frameNumber = $videoStartFrame + nFrame;
            let seqImgPaths = null;

            if (cMediaSlot == 'A'){
                // Update the bar cache status to 1 (caching)
                raw_images_a.setStatusAtFrame(1, nFrame);

                seqImgPaths = raw_images_a.paths;
            }

            if (cMediaSlot == 'B'){
                // Update the bar cache status to 1 (caching)
                raw_images_b.setStatusAtFrame(1, nFrame);

                seqImgPaths = raw_images_b.paths;
            }

            let queryParams = {
                src_img_type: imgTypeToLoadFrom,
                load_full_img: $isLoadFullImg,
                img_full_path: seqImgPaths[nFrame],
                frame_number: frameNumber,
                canvas_w: $canvasSize[0],
                canvas_h: $canvasSize[1]
            }

            //console.log(" # queryParams: ", queryParams);

            // Use WebWorkers to avoid lagging the window while gettin the image data 
            if (window.Worker){
                var cacheWorker = new WorkerBuilder(workerFile);

                // Send request to a web worker.
                cacheWorker.postMessage([queryParams, $addrAndPort]);
                //console.log('Message posted to worker');

                // Data received from the web worker.
                cacheWorker.onmessage = function(e) {
                    //console.log(e.data);
                    //console.log('Message received from worker');

                    if ( !e.data.error ){
                        // Push an array of the image's raw data into rawImageFrames
                        //let raw = e.data.image_raw_data;
                        let r_imgDimensions = e.data.img_dimensions;
                        let r_currentFrame = e.data.frame_number;
    
                        cW = r_imgDimensions[0];
                        cH = r_imgDimensions[1];
    
                        if (cMediaSlot == 'A'){
                            // Save the images paths into the array
                            raw_images_a.imgs.push([e.data.image_raw_data, r_imgDimensions]);
            
                            // Save the right order of frames
                            raw_images_a.order[r_currentFrame - $videoStartFrame] = raw_images_a.imgs.length - 1;
            
                            // Update the bar cache status to 2 (cached)
                            raw_images_a.setStatusAtFrame(2, (r_currentFrame - $videoStartFrame));
                        }
    
                        if (cMediaSlot == 'B'){
                            // Save the images paths into the array
                            raw_images_b.imgs.push([e.data.image_raw_data, r_imgDimensions]);
            
                            // Save the right order of frames
                            raw_images_b.order[r_currentFrame - $videoStartFrame] = raw_images_b.imgs.length - 1;
            
                            // Update the bar cache status to 2 (cached)
                            raw_images_b.setStatusAtFrame(2, (r_currentFrame - $videoStartFrame));
                        }
                    }
                    else{ // There was a problem while getting the image data.
                        let r_currentFrame = e.data.frame_number;

                        if (cMediaSlot == 'A'){
                            // Save the images paths into the array
                            raw_images_a.imgs.push([]);
            
                            // Save the right order of frames
                            raw_images_a.order[r_currentFrame - $videoStartFrame] = raw_images_a.imgs.length - 1;
            
                            // Update the bar cache status to 3 (error)
                            raw_images_a.setStatusAtFrame(3, (r_currentFrame - $videoStartFrame));
                        }
    
                        if (cMediaSlot == 'B'){
                            // Save the images paths into the array
                            raw_images_b.imgs.push([]);
            
                            // Save the right order of frames
                            raw_images_b.order[r_currentFrame - $videoStartFrame] = raw_images_b.imgs.length - 1;
            
                            // Update the bar cache status to 3 (error)
                            raw_images_b.setStatusAtFrame(3, (r_currentFrame - $videoStartFrame));
                        }

                    }
                }
            }
        }

        $mediaToBeImported = "";
    }

    async function updateCanvas(time) {

        if (raw_images_a.order.length > 0){
            // 24 frames per second (1000/24fps = 41.66):
            // Update the canvas every X frames per second 
            if (time > lastFrameTime + 41.66) {

                $canvasSize = [
                    Math.floor(canvas.getBoundingClientRect().width),
                    Math.floor(canvas.getBoundingClientRect().height)
                ];

                if ($imgDrawOnCanvasIsA){
                    // Update canvas if the image is cached
                    if ($progressA[$videoCurrentFrame] == 2){

                        // TODO: Get the image size and update the variables
                        imgW = raw_images_a.imgs[raw_images_a.order[$videoCurrentFrame]][1][0];
                        imgH = raw_images_a.imgs[raw_images_a.order[$videoCurrentFrame]][1][1];
        
                        let currentImageData = new ImageData(raw_images_a.imgs[raw_images_a.order[$videoCurrentFrame]][0], imgW, imgH);
            
                        if ($imgDrawOnCanvasIsAB){
                            // Update canvas if the image B is cached
                            if ($progressB[$videoCurrentFrame] == 2){
                                let scaleRatio = cW / $canvasSize[0];
    
                                let aW = $abHandlePos - Math.abs( (parentW - $canvasSize[0]) / 2 );
                                aW = parseInt(aW * scaleRatio);
    
                                if (aW < 0) aW = 0;
    
                                // Draw Image A
                                context.putImageData(currentImageData, 0, 0, 0, 0, aW, imgH);
    
                                currentImageData = new ImageData(raw_images_b.imgs[raw_images_b.order[$videoCurrentFrame]][0], imgW, imgH);
                                
                                let bW = 0;
                                if (cW > $canvasSize[0]) bW = parseInt(cW - aW);
                                else bW = parseInt($canvasSize[0] - aW) * scaleRatio;
    
                                //console.log($abHandlePos, $canvasSize[0], aW, bW);
                                
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
                                //if (rawImageFramesDiff[$videoCurrentFrame] == null){
                                    // 
                                    //var imgCopyA = new Image();
                                    //imgCopyA.src = canvas.toDataURL();
        
                                    let diffImageData = new ImageData(raw_images_b.imgs[raw_images_b.order[$videoCurrentFrame]][0], imgW, imgH);
                                    diffImageData = await createImageBitmap(diffImageData);
                                    //context.putImageData(diffImageData, 0, 0);
        
                                    context.globalAlpha = 1;   // amount of FX
                                    context.globalCompositeOperation = "difference";
                                    context.drawImage(diffImageData, 0, 0);
                                    //context.drawImage(frame, left, top, vidW * scale, vidH * scale);
    
                                    rawImageFramesDiff[$videoCurrentFrame] = new Image();
                                    rawImageFramesDiff[$videoCurrentFrame].src = canvas.toDataURL();
    
                                /*} else{ // If not then just draw it
                                    context.drawImage(rawImageFramesDiff[$videoCurrentFrame], 0, 0);
                                }*/
                            }
                        }

                        lastFrameTime = time;
                    }
                }

                if ($imgDrawOnCanvasIsB){
                    // Update canvas if the image is cached
                    if ($progressB[$videoCurrentFrame] == 2){
                        // TODO: Get the image size and update the variables
                        imgW = raw_images_b.imgs[raw_images_b.order[$videoCurrentFrame]][1][0];
                        imgH = raw_images_b.imgs[raw_images_b.order[$videoCurrentFrame]][1][1];
        
                        let currentImageData = new ImageData(raw_images_b.imgs[raw_images_b.order[$videoCurrentFrame]][0], imgW, imgH);
            
                        context.putImageData(currentImageData, 0, 0);
                        lastFrameTime = time;
                    }
                }
                
                // If player is not paused inc the frame number
                if (!($isVideoPaused)){
                    if ($videoCurrentFrame == $videoTotalFrameLength){
                        $videoCurrentFrame = 0;
                    } else {
                        $videoCurrentFrame = $videoCurrentFrame + 1;
                    }
                }
    
            }
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
