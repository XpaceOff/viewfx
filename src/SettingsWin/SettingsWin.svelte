<script>
    import { isSettingsWinOpen, limitCacheMb, isDevInfoOn } from '../stores.js'
    import { fly } from 'svelte/transition';
    import SettingNumInput from './SettingNumInput.svelte';
    import SettingBoolInput from './SettingBoolInput.svelte';
    import { writeTextFile, BaseDirectory } from '@tauri-apps/api/fs';
    import { notification_success, notification_error } from '../Notifications/theme01'

    // Update this list if you add more settings.
    function updateConf(){
        let confJson = {
            cache_limit: $limitCacheMb,
            dev_information: $isDevInfoOn,
        }

        return(JSON.stringify(confJson));
    }

    const writeConfFile = async () => {
        try {
            await writeTextFile({
                contents: updateConf(),
                path: `viewfx.conf`,
            },
            {
                dir: BaseDirectory.LocalData,
            });

            notification_success(`Changes saved!`)
        } catch (e) {
            //notification_error(`<strong>Error:</strong><br>`+e);
            notification_error(`<strong>Error writing on the conf file: </strong><br>`);
        }
    };

</script>

{#if $isSettingsWinOpen}
    <div 
        class="flex flex-row absolute top-0 right-0 w-full h-full z-40 bg-gradient-to-l from-zinc-800 via-zinc-800"
        transition:fly="{{ x: 200, duration: 300 }}"
    >
        <div class="flex w-1/2 h-full"/>
        <div class="flex flex-col w-1/2 h-full p-2">

            <div class="flex flex-col w-full h-full">
                <p class="text-2xl text-zinc-300 mb-4">Settings</p>
    
                <SettingNumInput
                    tittle="Cache limit (Mb)"
                    bind:value={$limitCacheMb}
                />
    
                <SettingBoolInput
                    tittle="Dev information"
                    bind:value={$isDevInfoOn}
                />

            </div>

            <div class="flex w-full h-10 justify-end">
                <div 
                    class="flex w-32 h-8 bg-zinc-700 hover:bg-sky-600 rounded-md items-center justify-center text-zinc-300 cursor-pointer scale-90"
                    on:click={writeConfFile}
                >
                    <p>Save Changes</p>
                </div>
            </div>

        </div>

    </div>
{/if}