<script>
    import VideoControlArea from './VideoControlArea.svelte';
    import CacheBarUnit from './Playback/Bar/CacheBarUnit.svelte';
    import BarHandle from './Playback/barHandle.svelte';
    import { barFrameCacheStatusA, barFrameCacheStatusB, videoTotalFrameLength, videoCurrentFrame, videoStartFrame } from './stores'
    

    let barWidth, barHeight;
</script>

<div class="flex flex-col w-full h-full">
    <div class="flex w-full h-2/6 py-0.5">
        <div class="flex w-12 h-full select-none justify-center items-center py-0.5 px-1">
            <div class="flex w-full h-full justify-center items-center bg-zinc-900/50 rounded-sm">
                {$videoStartFrame}
            </div>
        </div>

        <div class="flex w-full h-full bg-zinc-900 rounded-sm border border-black/10 drop-shadow-lg">
            <div class="flex-none flex-col w-full h-full overflow-hidden"
                bind:clientWidth={barWidth}
                bind:clientHeight={barHeight}
            >

                <BarHandle parentW={barWidth} parentH={barHeight}></BarHandle>

                <!-- Frame Numbers -->
                <div class="flex w-full h-4/6 px-2">
                    {#each $barFrameCacheStatusA as frameNumber, i}
                        <div class="flex w-full items-center justify-center text-sm {i == $videoCurrentFrame ? 'text-purple-400' : 'text-zinc-400'}">
                            <div class=" -ml-1 select-none scale-75">
                                {
                                $videoTotalFrameLength > 50 ? 
                                    i%5 == 0 ? 
                                        i+$videoStartFrame 
                                        : i == $videoCurrentFrame ?
                                        i+$videoStartFrame  : '' 
                                    : i+$videoStartFrame
                                }
                            </div>
                        </div>
                    {/each}
                </div>

                <!-- Frame lines and color bar cache Status-->
                <div class="flex flex-col w-full h-2/6">

                    <!-- Frame lines -->
                    <div class="flex w-full h-full px-2">
                        {#each $barFrameCacheStatusA as frameNumber, i}
                            <div class="flex w-full"></div>
                            <div class="flex w-full border-l {i == $videoCurrentFrame ? 'border-purple-400' : 'border-zinc-400'}"></div>
                        {/each}
                    </div>

                    <!-- Cache progress bar -->
                    <div class="flex flex-row w-full h-[1px] min-h-0 px-2 opacity-90">
                        {#each $barFrameCacheStatusA as cFrameCacheStatus, i}
                            {#if i==0}
                                <CacheBarUnit cacheStatus={cFrameCacheStatus}></CacheBarUnit>
                            {:else if i==$videoTotalFrameLength-1}
                                <CacheBarUnit cacheStatus={cFrameCacheStatus}></CacheBarUnit>
                            {:else}
                                <CacheBarUnit cacheStatus={cFrameCacheStatus}></CacheBarUnit>
                            {/if}
                        {/each}
                    </div>

                    <!-- Cache progress bar B -->
                    <div class="flex flex-row w-full h-[1px] min-h-0 px-2 opacity-90">
                        {#each $barFrameCacheStatusB as cFrameCacheStatus, i}
                            {#if i==0}
                                <CacheBarUnit cacheStatus={cFrameCacheStatus}></CacheBarUnit>
                            {:else if i==$videoTotalFrameLength-1}
                                <CacheBarUnit cacheStatus={cFrameCacheStatus}></CacheBarUnit>
                            {:else}
                                <CacheBarUnit cacheStatus={cFrameCacheStatus}></CacheBarUnit>
                            {/if}
                        {/each}
                    </div>
                </div>
            </div>
        </div>

        <div class="flex w-12 h-full select-none justify-center items-center py-0.5 px-1">
            <div class="flex w-full h-full justify-center items-center bg-zinc-900/50 rounded-sm">
                {$videoStartFrame+$videoTotalFrameLength}
            </div>
        </div>

    </div>
    <div class="flex w-full h-4/6 bg-zinc-800 p-1 rounded-b-lg">
        <VideoControlArea></VideoControlArea>
    </div>
</div>