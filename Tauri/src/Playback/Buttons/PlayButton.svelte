<script>
    export let isPaused;
</script>

<div
    on:click
    class="{isPaused ? 'shadow-inner drop-shadow-sm bg-zinc-800 opacity-90' : 'shadow-md drop-shadow-lg bg-zinc-800 hover:bg-zinc-700'} flex w-8 h-8 rounded-lg items-center justify-center mx-1 hover:scale-90 duration-100 active:bg-zinc-900"> 
    
    <!-- I have to make both icons because font awesome is injecting the svg when DOM is loaded -->
    
    <div class="{isPaused ? '' : 'hidden'} flex w-full h-full items-center justify-center">
        <i class="fa-solid fa-play text-purple-500"></i>
    </div>

    <div class="{isPaused ? 'hidden' : ''} flex w-full h-full items-center justify-center">
        <i class="fa-solid fa-pause text-purple-500"></i>
    </div>
</div>