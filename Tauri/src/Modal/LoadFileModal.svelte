<script>
    import { fs } from "@tauri-apps/api"
    import { onMount } from "svelte";
    import { modalSelectedDirPath, modalListOfFiles, modalListOfFilesError } from "../stores";
    import StdModalContainer from "./StdModalContainer.svelte";
    import StdSquareButton from "./Buttons/StdSquareButton.svelte";
    import { getQuickAccessDirs } from "../dirFunctions/quickAccess"
    import CloseButton from "./Buttons/CloseButton.svelte";
import { children } from "svelte/internal";

    let quickAccessDirectories = getQuickAccessDirs();

    // TODO: For now I am using the Tauri API to read directories.
    // This is quite limited so I might do this with invoke later :)
    async function dirPathChanged(){
        console.log("Changed");
        let rListOfFiles = [];

        try {
            rListOfFiles = await fs.readDir($modalSelectedDirPath);
        } catch (error) {
            rListOfFiles = "";
            //rListOfFilesError = error;
        }
        console.log(rListOfFiles);

        return(rListOfFiles)
    }
    function reloadFiles(){
        currentDirList = fs.readDir($modalSelectedDirPath);
    }
    let currentDirList = fs.readDir($modalSelectedDirPath);
    
</script>

<StdModalContainer tittle="Load File">
    <div class="flex flex-col w-full h-full">

        <!-- Top Area -->
        <div class="flex flex-row items-center w-full h-12 px-1">
            <StdSquareButton cssIcon={"fa-arrow-left"}></StdSquareButton>
            <StdSquareButton cssIcon={"fa-arrow-right"}></StdSquareButton>
            <StdSquareButton cssIcon={"fa-arrow-up"}></StdSquareButton>
            <input
                bind:value={$modalSelectedDirPath}
                on:change={reloadFiles}
                type="text"
                class="flex w-full h-6 mx-2 px-2 rounded-md bg-zinc-900 text-zinc-300 border-none ring-0 placeholder:text-zinc-600 focus:outline-none focus:ring-sky-500 focus:ring-1"
                placeholder="Directory..."
            />
        </div>

        <!-- Mid Area -->
        <div class="flex flex-row w-full h-full min-h-0 ">
            <!-- Quick Access Folders -->
            <div class="flex flex-col w-3/12 h-full bg-zinc-900/10">
                {#await quickAccessDirectories}
                    <p>...waiting</p>
                {:then qaDirs}
                    {#each qaDirs as qaDir}
                        <div class="{ qaDir[1] == $modalSelectedDirPath ? 'text-sky-500 bg-zinc-700' : 'text-zinc-400' } flex w-full h-6 mt-1 px-2 items-center hover:bg-zinc-800 rounded-l-md hover:text-sky-500 select-none">
                            <i class="fa-solid fa-folder mr-1"></i>
                            <p>{qaDir[0]}</p>
                        </div>
                    {/each}
                {:catch error}
                <!-- TODO: If folder is not found then add the folder but wirh a "lost" icon -->
                    <p class=" text-red-700">{error.message}</p>
                {/await}
            </div>

            <!-- Files and folders in current directory -->
            <div class="flex flex-col w-full h-full px-2 pt-1 pb-3 overflow-x-hidden overflow-y-auto">
                {#await currentDirList}
                    <p>...waiting</p>
                {:then cAllFiles}
                    {#each cAllFiles as nFile}
                            {#if nFile.children}
                                <div
                                    on:click={() => { 
                                        $modalSelectedDirPath = nFile.path;
                                        reloadFiles();
                                        //currentDirList = dirPathChanged();
                                    }}
                                    class="flex w-full h-6 mt-1 px-2 items-center text-zinc-400 hover:bg-zinc-800 rounded-l-md hover:text-sky-500 select-none">
                                    <i class="fa-solid fa-folder mr-1"></i>
                                    <p>{nFile.name}</p>
                                </div>
                            {:else}
                                <div class="flex w-full h-6 mt-1 px-2 items-center text-zinc-400 hover:bg-zinc-800 rounded-l-md hover:text-sky-500 select-none">
                                    <i class="fa-solid fa-file mr-1"></i>
                                    <p>{nFile.name}</p>
                                </div>
                            {/if}
                    {/each}
                {:catch error}
                    <!-- TODO: If error. Show the right error -->
                    <p class=" text-red-700">{error.message}</p>
                {/await}
            </div>
        </div>

        <!-- Bottom Area -->
        <div class="flex flex-row w-full h-12 border-t-2 border-zinc-900/30">
            <div class="flex w-8/12 h-full">

            </div>
            <div class="flex w-4/12 h-full items-center justify-end px-2">
                <div class="flex w-2/4 h-7 items-center justify-center px-2 bg-zinc-700 rounded-md text-zinc-300 hover:bg-sky-600 cursor-pointer select-none">
                    <i class="fa-solid fa-upload"></i>
                    <p class="ml-1">Import</p>
                </div>
            </div>
        </div>
    </div>
</StdModalContainer>