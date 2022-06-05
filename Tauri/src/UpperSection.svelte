<script>
    import { isModalActive, modalTittle, mediaToBeImported, imgDrawOnCanvasIsA, imgDrawOnCanvasIsB, imgDrawOnCanvasIsDiff, imgDrawOnCanvasIsAB, mediaSlot } from "./stores";
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
        <UpStdButtonTwoIco on:click={ openModalLoadFileA } cssIcon01={"fa-photo-film"} cssIcon02={"fa-a"}></UpStdButtonTwoIco>
        <UpStdButtonTwoIco on:click={ openModalLoadFileB } cssIcon01={"fa-photo-film"} cssIcon02={"fa-b"}></UpStdButtonTwoIco>
        <EmptySpace01></EmptySpace01>
        <UpButtonText
            isPress={$imgDrawOnCanvasIsDiff}
            text={"diff"}
            on:click={() => {
                if ($mediaSlot[0] && $mediaSlot[1]){
                    $imgDrawOnCanvasIsDiff = !($imgDrawOnCanvasIsDiff);
                }
            }}
        />
        <UpButtonText
            isPress={$imgDrawOnCanvasIsAB}
            text={"A/B"}
            on:click={() => {
                if ($mediaSlot[0] && $mediaSlot[1]){
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
            isPress={$imgDrawOnCanvasIsB}
            cssIcon={"fa-b"}
            on:click={() => {
                $imgDrawOnCanvasIsA = false;
                $imgDrawOnCanvasIsB = true;
            }}
        />
        <UpStdButton cssIcon={"fa-photo-film"}></UpStdButton>
        <UpStdButton cssIcon={"fa-object-intersect"}></UpStdButton>
    </div>
    <div class="flex flex-row h-full items-center">
        <UpStdButton cssIcon={"fa-gear"}></UpStdButton>
    </div>
</div>