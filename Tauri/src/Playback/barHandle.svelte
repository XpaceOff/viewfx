<script>
    import { mediaSlot, videoCurrentFrame, videoTotalFrameLength } from '../stores'

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
            if (moving) {
                let frame_width = (parentW) / ($videoTotalFrameLength+1);
                handlePos = (e.clientX-50);
                
                let tmpCal = handlePos / parentW;
                $videoCurrentFrame =  Math.round($videoTotalFrameLength * tmpCal);
                handlePos = (($videoCurrentFrame ) * frame_width) + frame_width/2;

                if (handlePos <= 1) handlePos = (frame_width/2) - 5;
                //if (handlePos >= parentW) handlePos = parentW-2;

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
    style="height: {parentH}px"
    class="{$mediaSlot[0] ? '' : ''} absolute top-0 bg-sky-700 opacity-90 w-[2px] hover:w-1 hover:-m-[1px] cursor-col-resize z-40"
>
</div>