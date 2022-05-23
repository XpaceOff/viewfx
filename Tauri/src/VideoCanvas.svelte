<script>
    import { invoke } from '@tauri-apps/api/tauri'
	import { onMount } from 'svelte';

    // Images variables
    let rawImagesFrames = [];
    let rawImagesFramesOrder = [];
	let rawImagesCounter = 0;

    let videoFrameLength = 9
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
    let currentFrame = 0;


    // Time variables
	let lastFrameTime = 0;

    onMount(() => {
		// prepare canvas stores
		context = canvas.getContext('2d');
        canvasW = Math.floor(canvas.getBoundingClientRect().width);
        canvasH = Math.floor(canvas.getBoundingClientRect().height);

        startCaching();
	});

    function startCaching(){

        
        for (let nFrame=0; nFrame<videoFrameLength; nFrame++){
            rawImagesFramesOrder.push(-1);
        }

        for (let nFrame=0; nFrame<videoFrameLength; nFrame++){

            frameNumber = frameStart + nFrame;
            console.log("# ----- > ", frameNumber);
            
            invoke('get_image_raw_data', { frameNumber, canvasW, canvasH }).then((data_from_rust) => {

                // Push an array of the image's raw data into rawImagesFrames
                rawImagesFrames.push([data_from_rust[0], data_from_rust[2]]);

                //console.log("Frame", data_from_rust[1] - frameStart, "in pos", rawImagesFrames.length-1, "will be saved in", data_from_rust[1] - frameStart);
                // Save the right order of frames
                rawImagesFramesOrder[data_from_rust[1] - frameStart] = rawImagesFrames.length - 1;

                framesCached += 1;

                // Once all frames are cached
                if (framesCached == videoFrameLength) updateCanvas();
            })
        }

    }

    function updateCanvas(time) {
		// TODO: Get the image size and update the variables
        
		imgW = rawImagesFrames[rawImagesFramesOrder[currentFrame]][1][0];
		imgH = rawImagesFrames[rawImagesFramesOrder[currentFrame]][1][1];
		
		// 24 frames per second (1000/24fps = 41.66):
		if (time > lastFrameTime + 41.66) {
			let raw = Uint8ClampedArray.from(rawImagesFrames[rawImagesFramesOrder[currentFrame]][0]);
			let currentImageData = new ImageData(raw, imgW, imgH);

			context.putImageData(currentImageData, 0, 0);
			lastFrameTime = time;
			
			if (currentFrame == videoFrameLength - 1){
				currentFrame = 0;
			} else {
				currentFrame = currentFrame + 1;
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
