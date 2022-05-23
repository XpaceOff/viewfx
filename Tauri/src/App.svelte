<script>
	import Canvas from './Canvas.svelte';
	import VideoCanvas from './VideoCanvas.svelte';
	import Fps from './FPS.svelte';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri'

	let frames = [];
	let nFrame = 0;
	let canvas;
	let tmpCanvas;
	let canvasW = 800;
	let canvasH = 400;
	let imgW = 0;
	let imgH = 0;
	let context;
	let lastFrameTime = 0;
	
	function imagedata_to_image(imagedata) {
		var ctx = tmpCanvas.getContext('2d');
		tmpCanvas.width = imagedata.width;
		tmpCanvas.height = imagedata.height;
		ctx.putImageData(imagedata, 0, 0);

		var image = new Image();
		image.src = tmpCanvas.toDataURL();
		return image;
	}

	function updateCanvas(time) {
		// TODO: Get the image size and update the variables
		imgW = 711;
		imgH = 400;
		
		// 24 frames per second (1000/24fps = 41.66):
		if (time > lastFrameTime + 41.66) {
			let raw = Uint8ClampedArray.from(frames[nFrame]);
			let currentImageData = new ImageData(raw, imgW, imgH);
			//let newImg = imagedata_to_image(currentImageData);
	
			//let imgRatioW = canvasW/imgW;
			//let imgRatioH = canvasH/imgH;
			//let ratio = Math.min ( imgRatioW, imgRatioH );

			//context.drawImage(newImg, 0, 0, imgW, imgH, (canvasW-imgW*ratio)/2, (canvasH-imgH*ratio)/2, imgW*ratio, imgH*ratio);//, 200, 200);
			context.putImageData(currentImageData, 100, 0);
			lastFrameTime = time;
			
			if (nFrame == frames.length - 1){
				nFrame = 0;
			} else {
				nFrame = nFrame + 1;
			}
		}
		requestAnimationFrame(updateCanvas);
	}

	/*onMount(() => {
		// prepare canvas stores
		context = canvas.getContext('2d');
	});*/

	/*invoke('get_image_raw_data').then((data_from_rust) => {
		//frames.push(data_from_rust);
		//console.log(data_from_rust);


		for (let nF=0; nF<data_from_rust.length; nF++){
			frames.push(data_from_rust[nF]);
		}

		console.log("# Raw Image received.");

		requestAnimationFrame(updateCanvas);
	});*/

	export let name;
	console.log(name);
</script>

<main class="h-full w-full overflow-hidden">
	<div class="flex flex-col w-full h-full overflow-hidden">
		<div class="flex w-full h-full basis-11/12 items-center justify-center bg-zinc-900">
			<VideoCanvas></VideoCanvas>
			<Fps></Fps>
		</div>
		<div class="flex w-full basis-1/12 bg-zinc-800"></div>
	</div>
	
	
	<!-- <Canvas></Canvas> -->
</main>

<style global lang="postcss">
	@tailwind base;
	@tailwind components;
	@tailwind utilities;

	:global(body) {
		/* this will apply to <body> */
		background: #27272A;
	}
</style>