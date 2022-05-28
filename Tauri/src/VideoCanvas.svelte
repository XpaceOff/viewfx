<script>
    import { invoke } from '@tauri-apps/api/tauri'
	import { onMount } from 'svelte';
    import { barFrameCacheStatus, videoTotalFrameLength, videoCurrentFrame } from './stores'
    import axios from "axios";

    // Images variables
    let rawImageFrames = [];
    let rawImageFramesOrder = [];
	let rawImagesCounter = 0;

    $videoTotalFrameLength = 24;
    let framesCached = 0;

    // DOM obj variables
	let canvas;
	let context;

    // Sizes 
	let imgW = 0;
	let imgH = 0;
    let canvasW = 0;
    let canvasH = 0;

    // 
    let frameNumber = 0;
    let frameStart = 1;
    $videoCurrentFrame = 0;


    // Time variables
	let lastFrameTime = 0;

    onMount(() => {
		// prepare canvas stores
		context = canvas.getContext('2d');
        canvasW = Math.floor(canvas.getBoundingClientRect().width);
        canvasH = Math.floor(canvas.getBoundingClientRect().height);

        startCaching();
        updateCanvas();
	});

    async function startCaching(){
        
        for (let nFrame=0; nFrame<$videoTotalFrameLength; nFrame++){
            // Create the array that will contain the right order of frames already cached
            rawImageFramesOrder.push(0);

            // Update the bar cache status to 0 (non-cached)
            $barFrameCacheStatus.push(0);
        }

        for (let nFrame=0; nFrame<$videoTotalFrameLength; nFrame++){

            frameNumber = frameStart + nFrame;

            // Update the bar cache status to 1 (caching)
            $barFrameCacheStatus[nFrame] = 1;

            axios.get('http://localhost:3000/image_raw_data', {
                //headers: {
                //    "Origin": [""],
			    //    "Access-Control-Allow-Origin": "*",
               // },
                params: {
                    frame_number: frameNumber,
                    canvas_w: canvasW,
                    canvas_h: canvasH
                }
            })
            .then(function (data_from_rust) {
                console.log(data_from_rust);
                // Push an array of the image's raw data into rawImageFrames
                let raw = Uint8ClampedArray.from(data_from_rust.data.image_raw_data);
                let r_imgDimensions = data_from_rust.data.img_dimensions;
                let r_currentFrame = data_from_rust.data.frame_number;

                rawImageFrames.push([raw, r_imgDimensions]);

                //console.log("Frame", r_currentFrame - frameStart, "in pos", rawImageFrames.length-1, "will be saved in", r_currentFrame - frameStart);
                // Save the right order of frames
                rawImageFramesOrder[r_currentFrame - frameStart] = rawImageFrames.length - 1;

                // Update the bar cache status to 1 (cached)
                $barFrameCacheStatus[r_currentFrame - frameStart] = 2;

                framesCached += 1;
            })
            .catch(function (error) {
                console.log(error);
            });
            
            /*invoke('get_image_raw_data', { frameNumber, canvasW, canvasH }).then((data_from_rust) => {
                // Push an array of the image's raw data into rawImageFrames
                let raw = Uint8ClampedArray.from(data_from_rust[0]);
                rawImageFrames.push([raw, data_from_rust[2]]);
                //rawImageFrames.push([data_from_rust[0], data_from_rust[2]]);

                //console.log("Frame", data_from_rust[1] - frameStart, "in pos", rawImageFrames.length-1, "will be saved in", data_from_rust[1] - frameStart);
                // Save the right order of frames
                rawImageFramesOrder[data_from_rust[1] - frameStart] = rawImageFrames.length - 1;

                // Update the bar cache status to 1 (cached)
                $barFrameCacheStatus[data_from_rust[1] - frameStart] = 2;

                framesCached += 1;

                // Once all frames are cached
                //if (framesCached == $videoTotalFrameLength) updateCanvas();
            })*/
        }
    }

    function updateCanvas(time) {
        
        // 24 frames per second (1000/24fps = 41.66):
        // Update the canvas every X frames per second 
		if (time > lastFrameTime + 41.66) {
            //console.log($barFrameCacheStatus[$videoCurrentFrame]);
            
            // Update canvas if the image is cached
            if ($barFrameCacheStatus[$videoCurrentFrame] == 2){
                //console.log($videoCurrentFrame, "printed ! ");
                // TODO: Get the image size and update the variables
                imgW = rawImageFrames[rawImageFramesOrder[$videoCurrentFrame]][1][0];
                imgH = rawImageFrames[rawImageFramesOrder[$videoCurrentFrame]][1][1];

                let currentImageData = new ImageData(rawImageFrames[rawImageFramesOrder[$videoCurrentFrame]][0], imgW, imgH);
    
                context.putImageData(currentImageData, 0, 0);
                lastFrameTime = time;
                
                if ($videoCurrentFrame == $videoTotalFrameLength - 1){
                    $videoCurrentFrame = 0;
                } else {
                    $videoCurrentFrame = $videoCurrentFrame + 1;
                }
            }

		}
		requestAnimationFrame(updateCanvas);
	}

</script>

<canvas
    bind:this={canvas}
    class="aspect-video h-full"
    width={imgW}
    height={imgH}
/>
