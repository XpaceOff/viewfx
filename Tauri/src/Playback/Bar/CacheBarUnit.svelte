<script>
    export let cacheStatus=0;
    export let corner=false;
</script>

{#if cacheStatus==2}
    <div class="flex {corner ? 'w-1/2' : 'w-full'} bg-green-400"></div>
{:else if cacheStatus==1}
    <div class="flex {corner ? 'w-1/2' : 'w-full'} bg-yellow-400"></div>
{:else}
    <div class="flex {corner ? 'w-1/2' : 'w-full'} bg-zinc-700"></div>
{/if}