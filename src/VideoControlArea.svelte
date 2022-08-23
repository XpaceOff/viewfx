<script>
    import { isVideoPaused, videoCurrentFrame, videoStartFrame, videoTotalFrameLength } from './stores';
    import StdButton01 from './Playback/Buttons/StdButton01.svelte';
    import PlayButton from './Playback/Buttons/PlayButton.svelte';
    import FpsInput from './Playback/FpsInput.svelte';

    function updatePlayStatus(){
        $isVideoPaused = !($isVideoPaused);
    }

    function nextFrame(){
        if ($isVideoPaused){
            if ( $videoCurrentFrame < $videoTotalFrameLength){
                $videoCurrentFrame = $videoCurrentFrame + 1;
            }
        }
    }

    function prevFrame(){
        if ($isVideoPaused){
            if ( ($videoCurrentFrame+$videoStartFrame) > $videoStartFrame ){
                $videoCurrentFrame = $videoCurrentFrame - 1;
            }
        }
    }
</script>

<div class="flex flex-row w-full h-full">
    <div class="flex flex-row w-2/12 h-full items-center">
        <FpsInput/>
    </div>

    <div class="flex flex-row w-8/12 h-full items-center justify-center">

        <!-- Prev Button-->
        <StdButton01 on:click={ prevFrame }  cssIcon={"fa-backward-step"}></StdButton01>

        <!-- Play/Pause Button-->
        <PlayButton on:click={ updatePlayStatus } isPaused={ $isVideoPaused }></PlayButton>
    
        <!-- Next Button-->
        <StdButton01 on:click={ nextFrame } cssIcon={"fa-forward-step"}></StdButton01>

    </div>

    <div class="flex flex-row w-2/12 h-full items-center justify-center">
    </div>
</div>