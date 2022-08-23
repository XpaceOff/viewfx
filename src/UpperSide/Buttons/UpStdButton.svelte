<script>
    export let cssIcon;
    export let isPress = false;
    export let isDisabled = false;
</script>

{#if isDisabled}
    <div
        class="flex w-7 h-6 p-1 mx-1 shadow-md rounded-md items-center text-zinc-600 bg-zinc-800 drop-shadow-xl opacity-30"
    >
        <i class="fa-solid {cssIcon}  w-full h-full "></i>
    </div>
{:else}
    <div
        on:click
        disabled={isDisabled}
        class="{isPress ? 'bg-zinc-900 scale-95' : 'bg-zinc-800 drop-shadow-xl'} flex w-7 h-6 p-1 mx-1 shadow-md rounded-md items-center text-zinc-600 hover:text-zinc-400 hover:scale-95 active:bg-zinc-900 active:text-zinc-300 active:scale-75 cursor-pointer"
    >
        <i class="fa-solid {cssIcon}  w-full h-full "></i>
    </div>
{/if}