<script>
	
	let text = '';

	let elapsed = 0;
	let frames = 0;
	let prevTime = performance.now();

    function fpsCalculation(){
        let time = performance.now();
		frames++;
		if ( time >= prevTime + 1000 ) {
			const fps = ((frames * 1000) / (time - prevTime));
			text = `${fps.toFixed(1)} FPS`;
			prevTime = time;
			frames = 0;
		}

        requestAnimationFrame(fpsCalculation);
    }

    requestAnimationFrame(fpsCalculation);
</script>

<div class="absolute top-0 left-0">
    <h3 class="h3">
        {text}
    </h3>
</div>


<!-- The following allows this component to nest children -->
<slot></slot>