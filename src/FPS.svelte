<script>
	import { getVersion } from '@tauri-apps/api/app';
	import { onMount } from 'svelte';
	import { videoCurrentFps } from './stores'
	
	let appVersion = "";
	let text = '';

	let elapsed = 0;
	let frames = 0;
	let prevTime = performance.now();

	onMount(async () => {
		appVersion = await getVersion();
	});

    function fpsCalculation(){
        let time = performance.now();
		frames++;
		if ( time >= prevTime + 1000 ) {
			const fps = ((frames * 1000) / (time - prevTime));
			text = `${fps.toFixed(1)} FPS`;
			prevTime = time;
			frames = 0;
		}

        requestAnimationFrame(fpsCalculation);
    }

    requestAnimationFrame(fpsCalculation);
</script>

<div class="absolute top-9 left-2 opacity-50 select-none">
	<div class="flex flex-col">
		<h3 class="h3">
			Canvas refresh rate:
			{text}
		</h3>
		<h3 class="h3">
			App version: {appVersion}
		</h3>
		<h3 class="h3">
			Current FPS: {$videoCurrentFps}
		</h3>
	</div>
</div>


<!-- The following allows this component to nest children -->
<slot></slot>