<script>
    import CloseButton from "./Buttons/CloseButton.svelte";
    import { isModalActive } from "../stores";

    export let tittle="Tittle";

    function closeModal(){
        $isModalActive = false;
    }
</script>

<div class="{$isModalActive ? '' : 'hidden'} flex fixed w-full h-full top-0 left-0 items-center justify-center z-40  bg-gradient-to-tr from-sky-500/50 to-fuchsia-500/90 backdrop-blur-sm">
    
    <!-- Modal Window -->
    <div class="flex flex-col w-5/6 h-4/6 lg:w-4/6 bg-zinc-800 rounded-xl overflow-hidden shadow-md drop-shadow-2xl ">
        <div class="flex flex-row w-full h-6 bg-zinc-900/50">
            <!-- <div class="flex w-24 h-full pr-6 items-center justify-end order-last"> -->
            <div class="flex w-24 h-full pl-4 items-center">
                <CloseButton on:click={ closeModal }></CloseButton>
            </div>
            <div class="flex w-full h-full items-center justify-center text-zinc-300 select-none">
                <p>{ tittle }</p>
            </div>
        </div>

        <!-- Modal Body -->
        <div class="flex w-full h-full px-0.5 pb-0.5 min-h-0">
            <slot>
                <h3 class=" text-zinc-200">Empty Modal</h3>
            </slot>
        </div>

    </div>
</div>