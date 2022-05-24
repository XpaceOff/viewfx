<script>
	import VideoCanvas from './VideoCanvas.svelte';
	import Bar from './bar.svelte';
	import Fps from './FPS.svelte';
	
	//let newImg = imagedata_to_image(currentImageData);

	//let imgRatioW = canvasW/imgW;
	//let imgRatioH = canvasH/imgH;
	//let ratio = Math.min ( imgRatioW, imgRatioH );

	//context.drawImage(newImg, 0, 0, imgW, imgH, (canvasW-imgW*ratio)/2, (canvasH-imgH*ratio)/2, imgW*ratio, imgH*ratio);//, 200, 200);

	export let name;
	console.log(name);
</script>

<main class="h-full w-full overflow-hidden">
	<div class="flex flex-col w-full h-full overflow-hidden">
		<div class="flex w-full h-full basis-11/12 items-center justify-center bg-zinc-900">
			 <VideoCanvas></VideoCanvas> 
			<Fps></Fps>
		</div>
		<div class="flex w-full basis-1/12 bg-zinc-800">
			<Bar></Bar>
		</div>
	</div>
</main>

<style global lang="postcss">
	@tailwind base;
	@tailwind components;
	@tailwind utilities;

	:global(body) {
		/* this will apply to <body> */
		background: #27272A;
		padding: 3px;
	}
</style>