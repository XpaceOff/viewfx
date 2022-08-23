<script>
    export let cssIcon01;
    export let cssIcon02;
    export let isDisabled = false;
</script>


{#if isDisabled}
    <div
        class="flex w-9 h-6 p-1 mx-1 shadow-md drop-shadow-xl bg-zinc-800 rounded-md items-center text-zinc-600 opacity-30"
    >
        <i class="fa-solid {cssIcon01} text-zinc-500 w-2/3"></i>
        <i class="fa-solid {cssIcon02} text-zinc-600 w-1/3 ml-0.5"></i>
    </div>
{:else}
    <div
        on:click
        class="flex w-9 h-6 p-1 mx-1 shadow-md drop-shadow-xl bg-zinc-800 rounded-md items-center text-zinc-600 hover:text-zinc-400 hover:scale-95 active:bg-zinc-900 active:text-zinc-300 active:scale-75"
    >
        <i class="fa-solid {cssIcon01} text-zinc-500 w-2/3"></i>
        <i class="fa-solid {cssIcon02} text-zinc-600 w-1/3 ml-0.5"></i>
    </div>
{/if}