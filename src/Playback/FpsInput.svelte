<script>
    import { videoFps, videoCurrentFps, isVideoPaused } from '../stores'
    
    export let min = 1;
    export let max = 140;
   
    function validate(e){
        let userValue = e.target.value;
        userValue = parseFloat(userValue.replace(/[^\d.]/g, ''));

        if (isNaN(userValue) || userValue === null || userValue === undefined) userValue = min;
        if (userValue < min) userValue = min;
        if (userValue > max) userValue = max;

        $videoFps = userValue;
    }
</script>

<input 
    class=" {($videoCurrentFps < $videoFps) && !($isVideoPaused) ? "text-yellow-800" : "text-zinc-600"} w-12 h-8 bg-zinc-900 rounded-md border-0 px-0.5 ml-2 text-sm text-center"

    value={ ($videoCurrentFps < $videoFps) && !($isVideoPaused) ? $videoCurrentFps : $videoFps}
    on:change={validate}
    placeholder="FPS"
/>