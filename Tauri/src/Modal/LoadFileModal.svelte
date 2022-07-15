<script>
    import { fs, path } from "@tauri-apps/api"
    import { modalSelectedDirPath, mediaSlot, isModalActive, modalListOfFiles, modalListOfFilesError, mediaToBeImported, videoTotalFrameLength, osSepChar } from "../stores";
    import StdModalContainer from "./StdModalContainer.svelte";
    import StdSquareButton from "./Buttons/StdSquareButton.svelte";
    import { getQuickAccessDirs } from "../dirFunctions/quickAccess";
    import { onMount } from 'svelte';
import { join } from "@tauri-apps/api/path";

    let quickAccessDirectories = getQuickAccessDirs();
    let selectedFileIndex = -1;
    let selectedFileObj = null;
    let pathHistory = [];
    let currentPathHisIndex = null;

    function popHistory(hisArray, currentIndex){
        if (currentIndex < hisArray.length-1){
            let nPops = hisArray.length - 1 - currentIndex;

            for (let i=0; i<nPops; i++){
                hisArray.pop();
            }
        }

        return( hisArray );
    }

    // TODO: For now I am using the Tauri API to read directories.
    // This is quite limited so I might do this with invoke later :)
    async function cReloadFiles(){
        let preList = [];   // List of files returned from Turi's API
        let rList = [];     // List to be returned
        let seqList = [];   // tmp list containing img seqs. Once I know the n seq these will be push to rList
        let imgFilter = "(.png)|(.exr)|(.jpeg)|(.gif)|(.bmp)|(.ico)|(.jpg)"; // File formats acepted by ViewFX
        let vidFilter = "(.mov)|(.mp4)|(.avi)|(.mvk)";
        let acceptedFormats = new RegExp("(" + imgFilter + "|" + vidFilter + ")$", "g");

        preList = await fs.readDir($modalSelectedDirPath);
        console.log("Current folder files", preList);

        // Personalise compare function to help sort the list of files
        function compare( a, b ) {
            if ( a.name < b.name ){
                return -1;
            }
            if ( a.name > b.name ){
                return 1;
            }
            return 0;
        }

        // Make sure to sort the files list before anithing so I can get the correct seq range.
        preList.sort(compare);

        for (const i in preList){

            // Is it's a folder
            if (preList[i].children){
                rList.push(preList[i])
            } else{ // If not, it's a file

                // Only show supported files
                if (preList[i].name.match(acceptedFormats)){

                    // Check if the image have a seq format
                    let imgSeqFormat = new RegExp("^(.+?)([0-9]+)("+imgFilter+")$");
                    let tmpSeqMatch = preList[i].name.match(imgSeqFormat); //(/^(.+?)([0-9]+)\.(.{3,4})$/);

                    // If so, push it to the seqList to be analized later.
                    if (tmpSeqMatch){
                        // Just make sure its a seq
                        if (!isNaN( parseInt(tmpSeqMatch[2]) ))
                            seqList.push([0, preList[i], ""]);
                    }
                    else{// Otherwise, just push it because it's either a video or a single image.
                        preList[i].seqLength = 0;

                        let rx_img_filter = new RegExp("^(.+?)("+imgFilter+")$");
                        if (preList[i].name.match(rx_img_filter)){
                            preList[i].file_type = "IMG";
                        }
                        else{
                            preList[i].file_type = "VID";
                        }

                        console.log(preList[i]);

                        rList.push(preList[i]);
                    }
                    //console.log(preList[i].name.split('.'));
                }
            }
        }

        if (seqList.length > 0){

            for (const nImg in seqList){
                let cImg = null;
                let nStart = 0;
                let nCurrent = 0;
                let nEnd = 0;

                if (seqList[nImg][0] == 0){
                    cImg = seqList[nImg][1].name.match(/^(.+?)([0-9]+)\.(.{3,4})$/);
                    console.log(cImg);

                    seqList[nImg][0] = 1; // Mark as read
                    nStart = parseInt(cImg[2]);
                    nCurrent = nStart;
                    console.log(nStart);

                    for (const nImg2 in seqList){

                        // If the image is not marked yet
                        if (seqList[nImg2][0] == 0){
                            let cImg2 = seqList[nImg2][1].name.match(/^(.+?)([0-9]+)\.(.{3,4})$/);

                            // Check that the name match
                            if (cImg2[1] == cImg[1]){

                                // Check that the ext match
                                if ( cImg2[3] == cImg[3] ){

                                    // Check that the nPads match
                                    if ( cImg2[2].length == cImg[2].length ){
    
                                        if ( parseInt(cImg2[2]) ==  nCurrent+1 ){
                                            seqList[nImg2][0] = 1;
                                            nCurrent = nCurrent + 1;
                                        }
                                    }
                                }
                            }
                        }
                    }

                    //
                    let padStart = nStart.toString().padStart(cImg[2].length, '0');
                    let padEnd = nCurrent.toString().padStart(cImg[2].length, '0');
                    seqList[nImg][2] = cImg[1] + padStart + '-' + padEnd + '.' + cImg[3];

                    seqList[nImg][1].name = seqList[nImg][2];
                    seqList[nImg][1].seqLength = nCurrent - nStart + 1;
                    seqList[nImg][1].file_type = "IMG";
                    rList.push(seqList[nImg][1]);
                }

                nStart = 0;
                nCurrent = 0;
                nEnd = 0;
            }

            //console.log("---", seqList);
        }

        return(rList);
    }

    function reloadFiles(){
        currentDirList = cReloadFiles();//fs.readDir($modalSelectedDirPath);
    }

    onMount(() => {
        // Get Home directory
        path.homeDir().then((tmpHomeDir) => {
            console.log("Home dir: ", tmpHomeDir);
            $modalSelectedDirPath = tmpHomeDir;

            pathHistory.push(tmpHomeDir);
            currentPathHisIndex = 0;

            reloadFiles();
        });
    })

    let currentDirList = cReloadFiles();//fs.readDir($modalSelectedDirPath);
    
</script>

<StdModalContainer tittle="Load File">
    <div class="flex flex-col w-full h-full">

        <!-- Top Area -->
        <div class="flex flex-row items-center w-full h-12 px-1">
            <StdSquareButton
                on:click={() => {
                    let tmpCurrentPath = $modalSelectedDirPath;

                    if (currentPathHisIndex > 0) currentPathHisIndex -= 1;
                    $modalSelectedDirPath = pathHistory[currentPathHisIndex];

                    // Only reload if the path actually changed
                    if ( tmpCurrentPath != $modalSelectedDirPath ) reloadFiles();
                }}
                cssIcon={"fa-arrow-left"}
            />
            <StdSquareButton
                on:click={() => {
                    let tmpCurrentPath = $modalSelectedDirPath;

                    if (currentPathHisIndex < pathHistory.length-1) currentPathHisIndex += 1;
                    $modalSelectedDirPath = pathHistory[currentPathHisIndex];

                    // Only reload if the path actually changed
                    if ( tmpCurrentPath != $modalSelectedDirPath ) reloadFiles();
                }}
                cssIcon={"fa-arrow-right"}
            />
            <StdSquareButton
                on:click={() => {
                    let tmpCurrentPath = $modalSelectedDirPath;

                    if (tmpCurrentPath[tmpCurrentPath.length-1] == $osSepChar)
                        tmpCurrentPath = tmpCurrentPath.slice(0,-1);

                    tmpCurrentPath = tmpCurrentPath.split($osSepChar).slice(0, tmpCurrentPath.split($osSepChar).length-1).join($osSepChar);

                    $modalSelectedDirPath = tmpCurrentPath;
                    reloadFiles();
                }}
                cssIcon={"fa-arrow-up"}
            />
            <input
                bind:value={$modalSelectedDirPath}
                on:change={(val) => {
                    pathHistory = popHistory(pathHistory, currentPathHisIndex);
                    pathHistory.push($modalSelectedDirPath);
                    currentPathHisIndex += 1;
                    
                    reloadFiles();
                }}
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
                        <div
                            on:click={() => {
                                $modalSelectedDirPath = qaDir[1];
                                pathHistory = popHistory(pathHistory, currentPathHisIndex);
                                pathHistory.push(qaDir[1]);
                                currentPathHisIndex += 1;
                                reloadFiles();
                            }}
                            class="{ qaDir[1] == $modalSelectedDirPath ? 'text-sky-500 bg-zinc-700' : 'text-zinc-400' } flex w-full h-6 mt-1 px-2 items-center hover:bg-zinc-800 rounded-l-md hover:text-sky-500 select-none cursor-pointer"
                        >
                            <i class="fa-solid fa-folder mr-1"></i>
                            <p>{qaDir[0]}</p>
                        </div>
                    {/each}
                {:catch error}
                <!-- TODO: If folder is not found then add the folder but with a "lost" icon -->
                    <p class=" text-red-700">{error}</p>
                {/await}
            </div>

            <!-- Files and folders in current directory -->
            <div class="flex flex-col w-full h-full px-2 pt-1 pb-3 overflow-x-hidden overflow-y-auto scrollbar-thin scrollbar-thumb-sky-700 scrollbar-track-zinc-600 hover:scrollbar-thumb-sky-600">
                {#await currentDirList}
                    <p>Loading Files...</p>
                {:then cAllFiles}
                    {#each cAllFiles as nFile, i}
                            {#if nFile.children}
                                <!-- If it's a folder -->
                                <div
                                    on:click={() => {
                                        $modalSelectedDirPath = nFile.path;
                                        pathHistory = popHistory(pathHistory, currentPathHisIndex);
                                        pathHistory.push(nFile.path);
                                        currentPathHisIndex += 1;
                                        selectedFileIndex = -1;
                                        selectedFileObj = null;
                                        reloadFiles();
                                        //currentDirList = dirPathChanged();
                                    }}
                                    class="flex w-full h-6 mt-1 px-2 items-center text-zinc-400 hover:bg-zinc-800 rounded-l-md hover:text-sky-500 select-none cursor-pointer">
                                    <i class="fa-solid fa-folder mr-1"></i>
                                    <p>{nFile.name}</p>
                                </div>
                            {:else}
                                <!-- If it's a file -->
                                <div
                                    on:click={() =>{
                                        selectedFileIndex = i;
                                        selectedFileObj = nFile;
                                        console.log("Selected: ", nFile);
                                        
                                    }}
                                    class="{selectedFileIndex == i ? 'bg-zinc-700' : ''} flex w-full h-6 mt-1 px-2 items-center text-zinc-400 rounded-l-md hover:text-sky-500 select-none cursor-pointer">
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

                        if ($mediaToBeImported == 'A'){
                            $mediaSlot[0] = selectedFileObj;  // Cache new media A
                            console.log($mediaSlot);
                            
                        }

                        if ($mediaToBeImported == 'B'){
                            // Only cache media B if the length is the same as media A
                            //if (selectedFileObj.seqLength == $videoTotalFrameLength){
                                $mediaSlot[1] = selectedFileObj;  // Cache new media B
                            //}

                            console.log(selectedFileObj.seqLength + "==" + $videoTotalFrameLength);
                        }
                        
                        // TODO: reset all modal variables to default.
                        $isModalActive = false;     // Close Modal
                        selectedFileObj = null;     // de-select file
                        selectedFileIndex = -1;     // de-select file
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