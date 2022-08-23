<script>
    import { fly } from 'svelte/transition';

    export let tittle;
    export let value=false;
</script>

<div class="flex flex-row w-full hover:bg-zinc-600 px-2 py-1 rounded-lg text-zinc-500 hover:text-zinc-300 items-center">
    <div class="flex w-8/12">
        {tittle}
    </div>
    <!-- <div class="flex w-4/12 ">
        {value}
    </div> -->
    <div class="{value ? "bg-purple-900" : "bg-zinc-900"} relative w-10 h-5 rounded-xl items-center cursor-pointer" on:click={() => value = !value}>
        {#if value === true}
            <div 
                class="absolute flex right-0 w-5 h-5 rounded-full bg-purple-600"
                transition:fly="{{ x: 3, duration: 300 }}"
            >
            </div>
        {:else}
            <div
                class="absolute flex left-0 w-5 rounded-full bg-zinc-600 opacity-60"
                style="height: 20px; margin-top: -.5px; margin-left: -.5px;"
                transition:fly="{{ x: -1, duration: 300 }}"
            >
            </div>
        {/if}
    </div>
</div>