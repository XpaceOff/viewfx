<script>
	import VideoCanvas from './VideoCanvas.svelte';
	import UpperSection from './UpperSection.svelte';
	import Bar from './bar.svelte';
	import Fps from './FPS.svelte';
	import AbSeparator from './abSeparator.svelte';
	import '@fortawesome/fontawesome-free/js/all';
	import { mediaSlot } from './stores'
	
	//let newImg = imagedata_to_image(currentImageData);

	//let imgRatioW = canvasW/imgW;
	//let imgRatioH = canvasH/imgH;
	//let ratio = Math.min ( imgRatioW, imgRatioH );

	//context.drawImage(newImg, 0, 0, imgW, imgH, (canvasW-imgW*ratio)/2, (canvasH-imgH*ratio)/2, imgW*ratio, imgH*ratio);//, 200, 200);

	let intViewerW, intViewerH;

	export let name;
	console.log(name);
</script>

<main class=" h-full w-full overflow-hidden">
	
	<div class="flex flex-col w-full h-full overflow-hidden">
		<div class="flex w-full h-6">
			<UpperSection></UpperSection>
		</div>
		<div
			bind:clientWidth={intViewerW}
			bind:clientHeight={intViewerH}
			class="flex w-full h-full items-center justify-center bg-zinc-900 my-1"
		>
			<VideoCanvas
				parentH={intViewerH}
				parentW={intViewerW}
			/>
			<Fps></Fps>
			<AbSeparator
				parentH={intViewerH}
				parentW={intViewerW}
			/>

			{#if !($mediaSlot[0] || $mediaSlot[1])}
				<div class=" absolute flex w-full h-full items-center justify-center overflow-hidden py-6 select-none opacity-5">
					<img class=" w-1/3  object-contain z-0" src="ViewFX_v04.png" alt="ViewFX Logo">
					<div class="absolute w-full h-full z-10"></div>
				</div>
			{/if}
		</div>
		<div class="flex w-full h-32 bg-zinc-800 pt-2">
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