<script>
    import { isSettingsWinOpen, limitCacheMb } from '../stores.js'
    import { fly } from 'svelte/transition';
    import SettingNumInput from './SettingNumInput.svelte';
</script>

{#if $isSettingsWinOpen}
    <div 
        class="flex flex-row absolute top-0 right-0 w-full h-full z-40 bg-gradient-to-l from-zinc-800 via-zinc-800"
        transition:fly="{{ x: 200, duration: 300 }}"
    >
        <div class="flex w-1/2 h-full"/>
        <div class="flex flex-col w-1/2 h-full p-2">
            <p class="text-2xl text-zinc-300 mb-4">Settings</p>

            <SettingNumInput
                tittle="Cache limit"
                bind:value={$limitCacheMb}
            />

        </div>

    </div>
{/if}