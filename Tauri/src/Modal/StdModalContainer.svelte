<script>
    import CloseButton from "./Buttons/CloseButton.svelte";
    import { isModalActive } from "../stores";

    export let name="Tittle";

    function closeModal(){
        $isModalActive = false;
    }
</script>

<div class="{$isModalActive ? '' : 'hidden'} flex fixed w-full h-full top-0 left-0 items-center justify-center z-40  bg-gradient-to-tr from-violet-500/50 to-fuchsia-500/90 backdrop-blur-sm">
    <div class="flex flex-col w-5/6 h-4/6 lg:w-4/6 bg-zinc-900 rounded-xl overflow-hidden shadow-md drop-shadow-2xl ">

        <div class="flex flex-row w-full h-6 bg-zinc-800/50">
            <!-- <div class="flex w-24 h-full pr-6 items-center justify-end order-last"> -->
            <div class="flex w-24 h-full pl-5 items-center">
                <CloseButton on:click={ closeModal }></CloseButton>
            </div>
            <div class="flex w-full h-full items-center justify-center text-zinc-300 select-none">
                <p>{ name }</p>
            </div>
        </div>

        <div class="flex w-full h-full p-2 ">
        </div>

    </div>
</div>