<script>
    export let cssIcon;
    export let isPress = false;
</script>

<div
    on:click
    class="{isPress ? 'bg-zinc-900 scale-95' : 'bg-zinc-800 drop-shadow-xl'} flex w-7 h-6 p-1 mx-1 shadow-md rounded-md items-center text-zinc-600 hover:text-zinc-400 hover:scale-95 active:bg-zinc-900 active:text-zinc-300 active:scale-75"
>
    <i class="fa-solid {cssIcon}  w-full h-full "></i>
</div>