<script>
    import { isVideoPaused, videoCurrentFrame, videoStartFrame, videoTotalFrameLength } from './stores';
    import StdButton01 from './Playback/Buttons/StdButton01.svelte';
    import PlayButton from './Playback/Buttons/PlayButton.svelte';

    function updatePlayStatus(){
        $isVideoPaused = !($isVideoPaused);
    }

    function nextFrame(){
        if ($isVideoPaused){
            if ( $videoCurrentFrame < ($videoStartFrame + $videoTotalFrameLength - 1) ){
                $videoCurrentFrame = $videoCurrentFrame + 1;
            }
        }
    }

    function prevFrame(){
        if ($isVideoPaused){
            if ( $videoCurrentFrame > $videoStartFrame ){
                $videoCurrentFrame = $videoCurrentFrame - 1;
            }
        }
    }
</script>

<div class="flex flex-col w-full h-full">
    <div class="flex flex-row w-full h-full items-start justify-center">

        <!-- Prev Button-->
        <StdButton01 on:click={ prevFrame }  cssIcon={"fa-backward-step"}></StdButton01>

        <!-- Play/Pause Button-->
        <PlayButton on:click={ updatePlayStatus } isPaused={ $isVideoPaused }></PlayButton>
    
        <!-- Next Button-->
        <StdButton01 on:click={ nextFrame } cssIcon={"fa-forward-step"}></StdButton01>

    </div>
</div>