<script>
    import VideoControlArea from './VideoControlArea.svelte';
    import CacheBarUnit from './Playback/Bar/CacheBarUnit.svelte';
    import BarHandle from './Playback/barHandle.svelte';
    import { videoTotalFrameLength, videoCurrentFrame, videoStartFrame, isVideoPaused } from './stores'
    import { raw_images_a, raw_images_b } from "./MediaCache/mediaCache.js";

    let progressA = raw_images_a.progress;
    let progressB = raw_images_b.progress;

    let barWidth, barHeight;
</script>

<div class="flex flex-col w-full h-full">
    <div class="flex w-full h-10 py-0.5">

        <!-- Start Frame Text-->
        <div class="flex shrink-0 w-12 h-full select-none justify-center items-center py-0.5 px-1">
            <div class="flex w-full h-full justify-center items-center bg-zinc-900/50 rounded-sm">
                {$videoStartFrame}
            </div>
        </div>

        <div class="flex w-full h-full bg-zinc-900 rounded-sm border border-black/10 drop-shadow-lg min-w-0">
            <div class="flex-none flex-col w-full h-full overflow-hidden"
                bind:clientWidth={barWidth}
                bind:clientHeight={barHeight}
            >

                <BarHandle parentW={barWidth} parentH={barHeight}></BarHandle>

                <!-- Frame Numbers -->
                <div class="relative flex w-full h-4/6">
                    {#each $progressA as frameNumber, i}
                        <div
                            class="flex items-center justify-center text-sm cursor-col-resize {i == $videoCurrentFrame ? 'text-purple-400' : 'text-zinc-400'}"
                            style="width: {(barWidth) / ($videoTotalFrameLength+1)}px"
                            on:click={() => {
                                if ($isVideoPaused){
                                    $videoCurrentFrame = i;
                                }
                            }}
                        >
                            <div class=" -ml-1 select-none cursor-col-resize scale-75">
                                {
                                    $videoTotalFrameLength > 50 ? 
                                        i%( Math.round(($videoTotalFrameLength+1)/10) ) == 0 ?
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
                    <div class="flex w-full h-full cursor-col-resize">
                        {#each $progressA as frameNumber, i}
                            <div class="flex w-full"></div>
                            <div 
                                class="flex w-full border-l {i == $videoCurrentFrame ? 'border-purple-400' : 'border-zinc-400'}"
                                on:click={() => {
                                    if ($isVideoPaused){
                                        $videoCurrentFrame = i;
                                    }
                                }}
                            ></div>
                        {/each}
                    </div>

                    <!-- Cache progress bar -->
                    <div class="flex flex-row w-full h-[1px] min-h-0 opacity-90">
                        {#each $progressA as cFrameCacheStatus, i}
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
                    <div class="flex flex-row w-full h-[1px] min-h-0 opacity-90">
                        {#each $progressB as cFrameCacheStatus, i}
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

        <!-- Last Frame Text-->
        <div class="flex shrink-0 w-12 h-full select-none justify-center items-center py-0.5 px-1">
            <div class="flex w-full h-full justify-center items-center bg-zinc-900/50 rounded-sm">
                {$videoStartFrame+$videoTotalFrameLength}
            </div>
        </div>

    </div>
    <div class="flex w-full h-full justify-center bg-zinc-800 p-1 rounded-b-lg">
        <VideoControlArea></VideoControlArea>
    </div>
</div>