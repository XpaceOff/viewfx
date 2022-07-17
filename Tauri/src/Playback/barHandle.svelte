<script>
    import { mediaSlot, videoCurrentFrame, videoTotalFrameLength, isVideoPaused } from '../stores'

    export let parentW;
    export let parentH;

    let handlePos = 0;

    function dragMe(node) {
        let moving = false;
        let left = 33;

        node.style.position = 'absolute';
        node.style.left = `${left}px`;

        node.addEventListener('mousedown', () => {
            moving = true;
        });
        
        window.addEventListener('mousemove', (e) => {
            // The Handle can only be updated when is grabbed and the video is paused.
            if (moving && $isVideoPaused) {

                // Calculate the width between each frame line.
                let frame_width = (parentW) / ($videoTotalFrameLength+1);

                // Get the handle's position minus an offset (whic is the left side)
                handlePos = (e.clientX-50);
                
                let tmpCal = handlePos / parentW;
                $videoCurrentFrame =  Math.round($videoTotalFrameLength * tmpCal);
                handlePos = (($videoCurrentFrame ) * frame_width) + frame_width/2;

                // Sets handles limits.
                if (handlePos < (frame_width/2)) handlePos = (frame_width/2) ;
                if (handlePos > (parentW - (frame_width/2))) handlePos =  parentW - (frame_width/2);

                // Update the handle's position.
                node.style.left = `${handlePos}px`;
            }
        });
    
        window.addEventListener('mouseup', () => {
            moving = false;
        });
	}
</script>

<div
    use:dragMe
    style="height: {parentH*.8}px; top: 18px; left: {($videoCurrentFrame * parentW / ($videoTotalFrameLength+1)) + (parentW / ($videoTotalFrameLength+1))/2 }px"
    class="{$mediaSlot[0] ? '' : ''} absolute top-0 bg-sky-700 opacity-90 w-[2px] hover:w-1 hover:-m-[1px] cursor-col-resize z-40"
>
</div>