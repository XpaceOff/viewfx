<script>
    import VideoControlArea from './VideoControlArea.svelte';
    import { barFrameCacheStatus, videoTotalFrameLength, videoCurrentFrame } from './stores'
</script>

<div class="flex flex-col w-full h-full">
    <div class="flex w-full h-2/6 py-0.5">
        <div class="flex w-full h-full bg-zinc-900  rounded-sm border border-black drop-shadow-lg">
            <div class="flex-none flex-col w-full h-full overflow-hidden">
                <div class="flex w-full h-4/6 px-2">
                    {#each $barFrameCacheStatus as frameNumber, i}
                        {#if i==$videoTotalFrameLength-1}
                            <div class="flex w-full items-center justify-between text-sm text-zinc-400">
                                <div class=" -ml-1">{i}</div>
                                <div class=" -mr-1">{i+1}</div>
                            </div>
                        {:else}
                            <div class="flex w-full items-center text-sm {i == $videoCurrentFrame ? 'text-purple-400' : 'text-zinc-400'}">
                                <div class=" -ml-1">{i}</div>
                            </div>
                        {/if}
                    {/each}
                </div>

                <!-- Frame lines and color bar cache Status-->
                <div class="flex flex-col w-full h-2/6">
                    <!-- Frame lines -->
                    <div class="flex w-full h-5/6 px-2">
                        {#each $barFrameCacheStatus as frameNumber, i}
                            {#if i==$videoTotalFrameLength-1}
                                <div class="flex w-full border-l border-x"></div>
                            {:else}
                                <div class="flex w-full border-l"></div>
                            {/if}
                        {/each}
                    </div>

                    <!-- Cache progress bar -->
                    <div class="flex flex-row w-full h-1/6 px-2 opacity-90">
                        {#each $barFrameCacheStatus as frameNumber, i}
                            {#if frameNumber==2}
                                <div class="flex w-full bg-green-400"></div>
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

    </div>
    <div class="flex w-full h-4/6 bg-zinc-800 p-1 rounded-b-lg">
        <VideoControlArea></VideoControlArea>
    </div>
</div>