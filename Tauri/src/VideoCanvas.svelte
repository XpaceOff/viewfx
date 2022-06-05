<script>
	import { onMount, onDestroy } from 'svelte';
    import { barFrameCacheStatusA, barFrameCacheStatusB, videoTotalFrameLength, videoCurrentFrame, isVideoPaused, videoStartFrame, canvasSize, mediaSlot, mediaToBeImported, imgDrawOnCanvasIsA, imgDrawOnCanvasIsB, imgDrawOnCanvasIsDiff } from './stores'
    import axios from "axios";

    // Images variables
    let rawImageFramesA = [];
    let rawImageFramesOrderA = [];
    let seqImgPathsA = [];

    let rawImageFramesB = [];
    let rawImageFramesOrderB = [];
    let seqImgPathsB = [];

    let rawImageFramesDiff = [];

    // DOM obj variables
	let canvas;
	let context;

    // Sizes 
	let imgW = 0;
	let imgH = 0;

    // Time variables
	let lastFrameTime = 0;

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

    // Cache Media once `mediaSlot` is changed
    const unsubscribe = mediaSlot.subscribe(value => {
        // TODO: all this function have to be re-written
        if ((value[0] || value[1]) && $mediaToBeImported != ""){
            let currentMedia = null;

            if ($mediaToBeImported == 'A') currentMedia = value[0];
            if ($mediaToBeImported == 'B') currentMedia = value[1];

            let splitedName = currentMedia.name.split('.');

            // Clear old Data
            if ($mediaToBeImported == 'A'){
                rawImageFramesOrderA = [];
                $barFrameCacheStatusA = [];
                $videoCurrentFrame = 0;
                rawImageFramesDiff = [];
            }

            if ($mediaToBeImported == 'B'){
                rawImageFramesOrderB = [];
                $barFrameCacheStatusB = [];
                rawImageFramesDiff = [];
            }

            // If it's a seq
            if (splitedName.length == 3){
                let splitedRange = splitedName[1].split('-');
                if (splitedRange.length == 2){
                    let imgFrom = parseInt(splitedRange[0]);
                    let imgTo = parseInt(splitedRange[1]);

                    $videoTotalFrameLength = imgTo-imgFrom;
                    $videoStartFrame = imgFrom;

                    for (let x=0; x<=$videoTotalFrameLength; x++){
                        let preProName = currentMedia.path.match(/^(.+?)([0-9]+)\.(.{3,4})$/);
                        let tmpImageName = preProName[1] + (''+(x+imgFrom)).padStart(3, '0') + '.' + preProName[3] //currentMedia.path.replace(/\\/g, '/');
                        tmpImageName = tmpImageName.replace(/\\/g, '/');
                        console.log(tmpImageName);

                        if ($mediaToBeImported == 'A'){
                            // Create the array of image paths
                            seqImgPathsA.push(tmpImageName);
    
                            // Create the array that will contain the right order of frames already cached
                            rawImageFramesOrderA.push(0);
    
                            // Update the bar cache status to 0 (non-cached)
                            $barFrameCacheStatusA.push(0);

                            // 
                            rawImageFramesDiff.push(null);
                        }

                        if ($mediaToBeImported == 'B'){
                            // Create the array of image paths
                            seqImgPathsB.push(tmpImageName);
    
                            // Create the array that will contain the right order of frames already cached
                            rawImageFramesOrderB.push(0);
    
                            // Update the bar cache status to 0 (non-cached)
                            $barFrameCacheStatusB.push(0);
                        }

                    }
                }
            }

            startCaching();

        }
    });

    onDestroy(unsubscribe);

    async function startCaching(){
        let cMediaSlot = $mediaToBeImported;

        for (let nFrame=0; nFrame<=$videoTotalFrameLength; nFrame++){

            let frameNumber = $videoStartFrame + nFrame;
            let seqImgPaths = null;

            if (cMediaSlot == 'A'){
                // Update the bar cache status to 1 (caching)
                $barFrameCacheStatusA[nFrame] = 1;

                seqImgPaths = seqImgPathsA;
            }

            if (cMediaSlot == 'B'){
                // Update the bar cache status to 1 (caching)
                $barFrameCacheStatusB[nFrame] = 1;

                seqImgPaths = seqImgPathsB;
            }


            axios.get('http://localhost:3000/image_raw_data', {
                //headers: {
                //    "Origin": [""],
			    //    "Access-Control-Allow-Origin": "*",
                // },
                params: {
                    img_full_path: seqImgPaths[nFrame],
                    frame_number: frameNumber,
                    canvas_w: $canvasSize[0],
                    canvas_h: $canvasSize[1]
                }
            })
            .then(function (data_from_rust) {
                // Push an array of the image's raw data into rawImageFrames
                let raw = Uint8ClampedArray.from(data_from_rust.data.image_raw_data);
                let r_imgDimensions = data_from_rust.data.img_dimensions;
                let r_currentFrame = data_from_rust.data.frame_number;

                if (cMediaSlot == 'A'){
                    // Save the images paths into the array
                    rawImageFramesA.push([raw, r_imgDimensions]);
    
                    // Save the right order of frames
                    rawImageFramesOrderA[r_currentFrame - $videoStartFrame] = rawImageFramesA.length - 1;
    
                    // Update the bar cache status to 1 (cached)
                    $barFrameCacheStatusA[r_currentFrame - $videoStartFrame] = 2;
                }

                if (cMediaSlot == 'B'){
                    // Save the images paths into the array
                    rawImageFramesB.push([raw, r_imgDimensions]);
    
                    // Save the right order of frames
                    rawImageFramesOrderB[r_currentFrame - $videoStartFrame] = rawImageFramesB.length - 1;
    
                    // Update the bar cache status to 1 (cached)
                    $barFrameCacheStatusB[r_currentFrame - $videoStartFrame] = 2;
                }

            })
            .catch(function (error) {
                // TODO: Change bar status to `red` / error
                // TBD
                console.log(error);
            });
        }
    }

    function updateCanvas(time) {

        if (rawImageFramesOrderA != []){
            // 24 frames per second (1000/24fps = 41.66):
            // Update the canvas every X frames per second 
            if (time > lastFrameTime + 41.66) {
                //console.log($barFrameCacheStatusA[$videoCurrentFrame]);

                if ($imgDrawOnCanvasIsA){
                    // Update canvas if the image is cached
                    if ($barFrameCacheStatusA[$videoCurrentFrame] == 2){
                        //console.log($videoCurrentFrame, "printed ! ");
                        // TODO: Get the image size and update the variables
                        imgW = rawImageFramesA[rawImageFramesOrderA[$videoCurrentFrame]][1][0];
                        imgH = rawImageFramesA[rawImageFramesOrderA[$videoCurrentFrame]][1][1];
        
                        let currentImageData = new ImageData(rawImageFramesA[rawImageFramesOrderA[$videoCurrentFrame]][0], imgW, imgH);
            
                        context.putImageData(currentImageData, 0, 0);

                        // Diff Code
                        if ($imgDrawOnCanvasIsDiff){

                            // If the diff image is not cached yet then start the computation
                            if (rawImageFramesDiff[$videoCurrentFrame] == null){
                                // 
                                var imgCopyA = new Image();
                                imgCopyA.src = canvas.toDataURL();
    
                                let diffImageData = new ImageData(rawImageFramesB[rawImageFramesOrderB[$videoCurrentFrame]][0], imgW, imgH);
                                context.putImageData(diffImageData, 0, 0);
    
                                context.globalAlpha = 1;   // amount of FX
                                context.globalCompositeOperation = "difference";
                                context.drawImage(imgCopyA, 0, 0);
                                //context.drawImage(frame, left, top, vidW * scale, vidH * scale);

                                rawImageFramesDiff[$videoCurrentFrame] = new Image();
                                rawImageFramesDiff[$videoCurrentFrame].src = canvas.toDataURL();

                            } else{ // If not then just draw it
                                context.drawImage(rawImageFramesDiff[$videoCurrentFrame], 0, 0);
                            }

                        }

                        lastFrameTime = time;
                    }
                }

                if ($imgDrawOnCanvasIsB){
                    // Update canvas if the image is cached
                    if ($barFrameCacheStatusB[$videoCurrentFrame] == 2){
                        // TODO: Get the image size and update the variables
                        imgW = rawImageFramesB[rawImageFramesOrderB[$videoCurrentFrame]][1][0];
                        imgH = rawImageFramesB[rawImageFramesOrderB[$videoCurrentFrame]][1][1];
        
                        let currentImageData = new ImageData(rawImageFramesB[rawImageFramesOrderB[$videoCurrentFrame]][0], imgW, imgH);
            
                        context.putImageData(currentImageData, 0, 0);
                        lastFrameTime = time;
                    }
                }
                
                // If player is not paused inc the frame number
                if (!($isVideoPaused)){
                    if ($videoCurrentFrame == $videoTotalFrameLength - 1){
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
    width={imgW}
    height={imgH}
/>
