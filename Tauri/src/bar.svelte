<script>
    import { barFrameCacheStatus } from './stores'
</script>

<div class="flex flex-col w-full h-full">
    <div class="flex w-full h-1/3 py-0.5">
        <div class="flex w-full h-full bg-zinc-900 opacity-50 rounded-sm border border-black drop-shadow-lg">
            <div class="flex flex-col w-full h-full">
                <div class="flex w-full h-4/5"></div>

                <!-- Cache progress bar -->
                <div class="flex flex-row w-full h-1/5">
                    {#each $barFrameCacheStatus as frameNumber, i}
                        {#if frameNumber==2}
                            <div class="flex w-full bg-green-400">{i}</div>
                        {:else if frameNumber==1}
                            <div class="flex w-full bg-yellow-400"></div>
                        {:else}
                            <div class="flex w-full bg-zinc-700"></div>
                        {/if}
                    {/each}
                </div>
            </div>
        </div>

    </div>
    <div class="flex w-full h-2/3 bg-zinc-900 rounded-b-lg text-sm">
        Buttons Area
    </div>
</div>