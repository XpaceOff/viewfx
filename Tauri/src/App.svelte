<script>
	import VideoCanvas from './VideoCanvas.svelte';
	import UpperSection from './UpperSection.svelte';
	import Bar from './bar.svelte';
	import Fps from './FPS.svelte';
	import AbSeparator from './abSeparator.svelte';
	import '@fortawesome/fontawesome-free/js/all';
	import { mediaSlot, internalViewwerSize, addrAndPort, osSepChar, bridgeHash } from './stores'
	import { invoke } from '@tauri-apps/api/tauri'
	import { path } from "@tauri-apps/api"
	import { SvelteToast } from '@zerodevx/svelte-toast'
	import SettingsWin from './SettingsWin/SettingsWin.svelte'
	import { isDevInfoOn, limitCacheMb } from './stores.js'
    import { readTextFile, BaseDirectory } from '@tauri-apps/api/fs';
    import { notification_success, notification_error } from './Notifications/theme01'

	const readConfFile = async () => {
        try {
            let confContent = await readTextFile(
				"viewfx.conf",{
               		dir: BaseDirectory.LocalData
				}
			);

			confContent = JSON.parse(confContent);
			console.log(confContent);

			// Update this list if you add more settings.
			if (confContent.cache_limit) $limitCacheMb = confContent.cache_limit;
			if (confContent.dev_information) $isDevInfoOn = confContent.dev_information;

            //notification_success(`Configuration file loaded!`)
        } catch (e) {
			console.log(e);
            //notification_error(`<strong>Error:</strong><br>`+e);
        }
    };

	// Get the backend bridge's ip and port.
	invoke('get_bg_addr').then((addr_and_port) => {
		$addrAndPort = addr_and_port;
		console.log(addr_and_port)
	});

	// Get the backend bridge's ip and port.
	invoke('get_bg_hash').then((bridge_hash) => {
		$bridgeHash = bridge_hash;
		//console.log(bridge_hash)
	});

	// Sets the OS separator character.
	$osSepChar = path.sep;

	export let name;
	console.log(name);
	readConfFile();
</script>

<main class=" h-full w-full overflow-hidden">
	
	<div class="flex flex-col w-full h-full overflow-hidden">
		<div class="flex w-full h-6">
			<UpperSection></UpperSection>
		</div>
		<div
			bind:clientWidth={$internalViewwerSize[0]}
			bind:clientHeight={$internalViewwerSize[1]}
			class="flex w-full h-full items-center justify-center bg-zinc-900 my-1 overflow-hidden"
		>
			<VideoCanvas
				parentW={$internalViewwerSize[0]}
			/>

			{#if $isDevInfoOn}
				<Fps></Fps>
			{/if}

			<AbSeparator
				parentH={$internalViewwerSize[1]}
				parentW={$internalViewwerSize[0]}
			/>
			<SvelteToast />
			<SettingsWin/>

			<!-- Only show ViewFX logo if no media has been loaded -->
			{#if !($mediaSlot[0] || $mediaSlot[1])}
				<div class=" absolute flex w-full h-full items-center justify-center overflow-hidden py-6 select-none opacity-5">
					<img class=" w-1/3  object-contain z-0" src="ViewFX_v04.png" alt="ViewFX Logo">
					<div class="absolute w-full h-full z-10"></div>
				</div>
			{/if}
		</div>
		<div class="flex w-full h-24 bg-zinc-800 pt-2">
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
		user-select: none;
	}
</style>