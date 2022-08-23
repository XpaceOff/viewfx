<script>
    export let text;
    export let isPress = false;
    export let isDisabled = false;
</script>

{#if isDisabled}
    <div
        disabled={isDisabled}
        class="flex w-7 h-6 p-1 mx-1 shadow-md rounded-md items-center text-zinc-600 opacity-30 cursor-default"
    >
        <p class=" select-none text-sm">{text}</p>
    </div>
{:else}
    <div
        on:click
        disabled={isDisabled}
        class="{isPress ? 'bg-zinc-900 scale-95' : 'bg-zinc-800 drop-shadow-xl'} flex w-7 h-6 p-1 mx-1 shadow-md rounded-md items-center text-zinc-600 hover:text-zinc-400 hover:scale-95 active:bg-zinc-900 active:text-zinc-300 active:scale-75 cursor-pointer "
    >
        <p class=" select-none text-sm">{text}</p>
    </div>
{/if}