<script>
    import { fs } from "@tauri-apps/api"
    import { modalSelectedDirPath, modalSelectedMedia, mediaA, isModalActive, modalListOfFiles, modalListOfFilesError } from "../stores";
    import StdModalContainer from "./StdModalContainer.svelte";
    import StdSquareButton from "./Buttons/StdSquareButton.svelte";
    import { getQuickAccessDirs } from "../dirFunctions/quickAccess"

    let quickAccessDirectories = getQuickAccessDirs();
    let selectedFileIndex = -1;
    let selectedFileObj = null;

    // TODO: For now I am using the Tauri API to read directories.
    // This is quite limited so I might do this with invoke later :)
    async function cReloadFiles(){
        let preList = [];
        let rList = [];
        let seqList = [];
        let imgFilter = /((.png)|(.exr)|(.jpeg)|(.jpg))$/g;

        preList = await fs.readDir($modalSelectedDirPath);
        console.log(preList);

        for (const i in preList){

            // Is it's a folder
            if (preList[i].children){
                rList.push(preList[i])
            } else{
                if (preList[i].name.match(imgFilter)){
                    let tmpSplit = preList[i].name.split('.');

                    if (!isNaN(tmpSplit[1])){
                        seqList.push([0, preList[i]]);
                    }
                    else{
                        rList.push(preList[i]);
                    }

                    //console.log(preList[i].name.split('.'));
                }
            }
        }

        if (seqList.length > 0){

            /*for (const i in seqList){
                console.log(seqList[i][1]);
            }*/
    
            let fSplit = seqList[1][1].name.split('.');
            let imgFrom = seqList[0][1].name.split('.')[1];
            let imgTo = seqList[seqList.length-1][1].name.split('.')[1];
            let tmpNewName = [fSplit[0], imgFrom+"-"+imgTo, fSplit[2]].join('.');
            seqList[0][1].name = tmpNewName;
            rList.push(seqList[0][1]);
            console.log(seqList[0]);
            console.log(tmpNewName);
        }

        return(rList);
    }
    function reloadFiles(){
        currentDirList = cReloadFiles();//fs.readDir($modalSelectedDirPath);
    }
    let currentDirList = cReloadFiles();//fs.readDir($modalSelectedDirPath);
    
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
                    <p class=" text-red-700">{error}</p>
                {/await}
            </div>

            <!-- Files and folders in current directory -->
            <div class="flex flex-col w-full h-full px-2 pt-1 pb-3 overflow-x-hidden overflow-y-auto">
                {#await currentDirList}
                    <p>Loading Files...</p>
                {:then cAllFiles}
                    {#each cAllFiles as nFile, i}
                            {#if nFile.children}
                                <!-- If it's a folder -->
                                <div
                                    on:click={() => { 
                                        $modalSelectedDirPath = nFile.path;
                                        selectedFileIndex = -1;
                                        selectedFileObj = null;
                                        reloadFiles();
                                        //currentDirList = dirPathChanged();
                                    }}
                                    class="flex w-full h-6 mt-1 px-2 items-center text-zinc-400 hover:bg-zinc-800 rounded-l-md hover:text-sky-500 select-none">
                                    <i class="fa-solid fa-folder mr-1"></i>
                                    <p>{nFile.name}</p>
                                </div>
                            {:else}
                                <!-- If it's a file -->
                                <div
                                    on:click={() =>{
                                        selectedFileIndex = i;
                                        selectedFileObj = nFile;
                                        $modalSelectedMedia = nFile.path;
                                        console.log("Selected: ", nFile);
                                        
                                    }}
                                    class="{selectedFileIndex == i ? 'bg-zinc-700' : ''} flex w-full h-6 mt-1 px-2 items-center text-zinc-400 rounded-l-md hover:text-sky-500 select-none">
                                    <i class="fa-solid fa-file mr-1"></i>
                                    <p>{nFile.name}</p>
                                </div>
                            {/if}
                    {/each}
                {:catch error}
                    <!-- TODO: If error. Show the right error -->
                    <p class=" text-red-700">{error}</p>
                {/await}
            </div>
        </div>

        <!-- Bottom Area -->
        <div class="flex flex-row w-full h-12 border-t-2 border-zinc-900/30">
            <div class="flex w-8/12 h-full">

            </div>
            <div class="flex w-4/12 h-full items-center justify-end px-2">
                <button
                    on:click={() => {
                        $mediaA = selectedFileObj;  // Cache new Media A
                        $isModalActive = false;     // Close Modal

                        // TODO: reset all modal variables to default.
                    }}
                    disabled={ selectedFileIndex == -1} 
                    class="flex w-2/4 h-7 items-center justify-center px-2 bg-zinc-700 rounded-md text-zinc-300 hover:bg-sky-600 select-none border-none disabled:opacity-50 disabled:bg-zinc-700">
                    <i class="fa-solid fa-upload"></i>
                    <p class="ml-1">Import</p>
                </button>
            </div>
        </div>
    </div>
</StdModalContainer>