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
                //handlePos = (($videoCurrentFrame+1) * parentW / $videoTotalFrameLength) + 16;
                //console.log($videoCurrentFrame, $videoTotalFrameLength, parentW);
                //console.log("HandlePos: ", handlePos);
                //console.log(e.movementX);
                handlePos += e.movementX;


                if (handlePos <= 1) handlePos = 1;
                if (handlePos >= parentW) handlePos = parentW-2;

                let tmpCal = handlePos / parentW;
                //handlePos = Math.round($videoTotalFrameLength * tmpCal)
                $videoCurrentFrame =  Math.round($videoTotalFrameLength * tmpCal);

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
    class="{$mediaSlot[0] ? '' : 'hidden'} absolute top-0 bg-sky-700 opacity-90 w-[2px] hover:w-1 hover:-m-[1px] cursor-col-resize z-40"
>
</div>