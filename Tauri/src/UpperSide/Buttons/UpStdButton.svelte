<script>
    export let cssIcon;
</script>

<div
    on:click
    class="flex w-7 h-6 p-1 mx-1 shadow-md drop-shadow-xl bg-zinc-800 rounded-md items-center text-zinc-600 hover:text-zinc-400 hover:scale-95 active:bg-zinc-900 active:text-zinc-300 active:scale-75"
>
    <i class="fa-solid {cssIcon}  w-full h-full "></i>
</div>