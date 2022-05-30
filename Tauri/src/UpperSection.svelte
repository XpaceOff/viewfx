<script>
    import UpStdButton from "./UpperSide/Buttons/UpStdButton.svelte";
    import UpStdButtonTwoIco from "./UpperSide/Buttons/UpStdButtonTwoIco.svelte";
    import EmptySpace01 from "./UpperSide/EmptySpace01.svelte";
</script>

<div class="flex flex-row w-full h-full px-2 py-1 items-center justify-between">
    <div class="flex flex-row h-full items-center">
        <UpStdButtonTwoIco cssIcon01={"fa-photo-film"} cssIcon02={"fa-a"}></UpStdButtonTwoIco>
        <UpStdButtonTwoIco cssIcon01={"fa-photo-film"} cssIcon02={"fa-b"}></UpStdButtonTwoIco>
        <EmptySpace01></EmptySpace01>
        <UpStdButton cssIcon={"fa-photo-film"}></UpStdButton>
        <UpStdButton cssIcon={"fa-a"}></UpStdButton>
        <UpStdButton cssIcon={"fa-object-intersect"}></UpStdButton>
    </div>
    <div class="flex flex-row h-full items-center">
        <UpStdButton cssIcon={"fa-gear"}></UpStdButton>
    </div>
</div>