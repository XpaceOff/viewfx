<script>
	import { onMount, onDestroy } from 'svelte';
    import { barFrameCacheStatus, videoTotalFrameLength, videoCurrentFrame, isVideoPaused, videoStartFrame, canvasSize, mediaA } from './stores'
    import axios from "axios";

    // Images variables
    let rawImageFrames = [];
    let rawImageFramesOrder = [];
    let seqImgPaths = [];

    $videoStartFrame = 0;

    // DOM obj variables
	let canvas;
	let context;

    // Sizes 
	let imgW = 0;
	let imgH = 0;

    // 
    let frameNumber = 0;
    let frameStart = 1;
    


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

        //startCaching();
        //updateCanvas();
	});

    // Cache Media A once the store `mediaA` is changed
    const unsubscribe = mediaA.subscribe(value => {
        // TODO: all this function have to be re-written
        if (value){
            let splitedName = value.name.split('.');

            // Clear old Data
            rawImageFramesOrder = [];
            $barFrameCacheStatus = [];
            $videoCurrentFrame = 0;

            // If it's a seq
            if (splitedName.length == 3){
                let splitedRange = splitedName[1].split('-');
                if (splitedRange.length == 2){
                    let imgFrom = parseInt(splitedRange[0]);
                    let imgTo = parseInt(splitedRange[1]);

                    $videoTotalFrameLength = imgTo-imgFrom;

                    for (let x=0; x<=imgTo-imgFrom; x++){
                        let preProName = value.path.match(/^(.+?)([0-9]+)\.(.{3,4})$/);
                        let tmpImageName = preProName[1] + (''+(x+imgFrom)).padStart(3, '0') + '.' + preProName[3] //value.path.replace(/\\/g, '/');
                        tmpImageName = tmpImageName.replace(/\\/g, '/');
                        console.log(tmpImageName);

                        // Create the array of image paths
                        seqImgPaths.push(tmpImageName);

                        // Create the array that will contain the right order of frames already cached
                        rawImageFramesOrder.push(0);

                        // Update the bar cache status to 0 (non-cached)
                        $barFrameCacheStatus.push(0);
                    }
                }
            }

            startCaching();
            updateCanvas();

        }
    });

    onDestroy(unsubscribe);

    async function startCaching(){

        for (let nFrame=0; nFrame<=$videoTotalFrameLength; nFrame++){

            frameNumber = frameStart + nFrame;

            // Update the bar cache status to 1 (caching)
            $barFrameCacheStatus[nFrame] = 1;

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

                // Save the images paths into the array
                rawImageFrames.push([raw, r_imgDimensions]);

                // Save the right order of frames
                rawImageFramesOrder[r_currentFrame - frameStart] = rawImageFrames.length - 1;

                // Update the bar cache status to 1 (cached)
                $barFrameCacheStatus[r_currentFrame - frameStart] = 2;
            })
            .catch(function (error) {
                // TODO: Change bar status to `red` / error
                // TBD
                console.log(error);
            });
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
		requestAnimationFrame(updateCanvas);
	}

</script>

<canvas
    bind:this={canvas}
    class="aspect-video h-full"
    width={imgW}
    height={imgH}
/>
