<script>
    import { isModalActive, modalTittle, mediaToBeImported, imgDrawOnCanvasIsA, imgDrawOnCanvasIsB, imgDrawOnCanvasIsDiff, imgDrawOnCanvasIsAB, mediaSlot } from "./stores";
    import { isLoadFullImg, isSettingsWinOpen } from "./stores";
    import UpStdButton from "./UpperSide/Buttons/UpStdButton.svelte";
    import UpStdButtonTwoIco from "./UpperSide/Buttons/UpStdButtonTwoIco.svelte";
    import UpButtonText from "./UpperSide/Buttons/UpButtonText.svelte";
    import EmptySpace01 from "./UpperSide/EmptySpace01.svelte";
    import LoadFileModal from "./Modal/LoadFileModal.svelte";

    function openModalLoadFileA(){
        $isModalActive = true;
        $modalTittle = "Load File A";
        $mediaToBeImported = 'A';
    }

    function openModalLoadFileB(){
        $isModalActive = true;
        $modalTittle = "Load File B";
        $mediaToBeImported = 'B';
    }
</script>

<div class="flex flex-row w-full h-full px-2 py-1 items-center justify-between">
    <LoadFileModal></LoadFileModal>
    <div class="flex flex-row h-full items-center">

        <!-- Only show options when the settings win is close-->
        {#if !($isSettingsWinOpen)}
            <UpStdButtonTwoIco
                on:click={ openModalLoadFileA }
                cssIcon01={"fa-photo-film"}
                cssIcon02={"fa-a"}
            />
            <UpStdButtonTwoIco
                on:click={ openModalLoadFileB }
                cssIcon01={"fa-photo-film"}
                cssIcon02={"fa-b"}
                isDisabled={!($mediaSlot[0])}
            />
            <EmptySpace01></EmptySpace01>
            <UpButtonText
                isDisabled={!($mediaSlot[0] && $mediaSlot[1])}
                isPress={$imgDrawOnCanvasIsDiff}
                text={"diff"}
                on:click={() => {
                    if ($mediaSlot[0] && $mediaSlot[1]){
                        if ($imgDrawOnCanvasIsDiff == false){
                            $imgDrawOnCanvasIsAB = false;
                            $imgDrawOnCanvasIsA = true;
                            $imgDrawOnCanvasIsB = false;
                        }

                        $imgDrawOnCanvasIsDiff = !($imgDrawOnCanvasIsDiff);
                    }
                }}
            />
            <UpButtonText
            isDisabled={!($mediaSlot[0] && $mediaSlot[1])}
                isPress={$imgDrawOnCanvasIsAB}
                text={"A/B"}
                on:click={() => {
                    if ($mediaSlot[0] && $mediaSlot[1]){
                        if ($imgDrawOnCanvasIsAB == false){
                            $imgDrawOnCanvasIsDiff = false;
                            $imgDrawOnCanvasIsA = true;
                            $imgDrawOnCanvasIsB = false;
                        }

                        $imgDrawOnCanvasIsAB = !($imgDrawOnCanvasIsAB);
                    }
                }}
            />
            <UpStdButton
                isPress={$imgDrawOnCanvasIsA}
                cssIcon={"fa-a"}
                on:click={() => {
                    $imgDrawOnCanvasIsA = true;
                    $imgDrawOnCanvasIsB = false;
                }}
            />
            <UpStdButton
                isDisabled={!($mediaSlot[1])}
                isPress={$imgDrawOnCanvasIsB}
                cssIcon={"fa-b"}
                on:click={() => {
                    if ($imgDrawOnCanvasIsAB == false && $imgDrawOnCanvasIsDiff == false) {
                        $imgDrawOnCanvasIsA = false;
                        $imgDrawOnCanvasIsB = true;
                    }
                    if ($imgDrawOnCanvasIsAB == true || $imgDrawOnCanvasIsDiff == true) {
                        $imgDrawOnCanvasIsAB = false;
                        $imgDrawOnCanvasIsDiff = false;
                        $imgDrawOnCanvasIsA = false;
                        $imgDrawOnCanvasIsB = true;
                    }
                }}
            />
            <EmptySpace01></EmptySpace01>
            <!-- <UpStdButton
                isPress={$isLoadFullImg}
                cssIcon={"fa-square-full"}
                on:click={() => {
                    $isLoadFullImg = !($isLoadFullImg);
                }}
            /> -->
        {/if}
    </div>
    <div class="flex flex-row h-full items-center">
        <UpStdButton 
            cssIcon={"fa-gear"}
            on:click={() => {
                $isSettingsWinOpen = !($isSettingsWinOpen);
            }}
        />
    </div>
</div>