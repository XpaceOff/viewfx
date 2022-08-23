<script>
    import { abHandlePos, imgDrawOnCanvasIsAB } from './stores'

    export let parentW;
    export let parentH;

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
                $abHandlePos = e.clientX;

                if ($abHandlePos <= 1) $abHandlePos = 1;
                if ($abHandlePos >= parentW) $abHandlePos = parentW-2;

                node.style.left = `${$abHandlePos}px`;
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
    class="{$imgDrawOnCanvasIsAB ? '' : ' hidden'} absolute top-0 bg-sky-700 opacity-90 w-[2px] hover:w-1 hover:-m-[1px] cursor-col-resize"
>
</div>