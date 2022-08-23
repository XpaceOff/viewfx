<script>
	import { onMount } from 'svelte';

    let frames = [];
    let nFrame = 0;
    let videoReady = false;
	let canvas;
	let context;
    let video = document.createElement("video"); // create a video element
    let mediaSource01 = "http://upload.wikimedia.org/wikipedia/commons/7/79/Big_Buck_Bunny_small.ogv";
    mediaSource01 = "proA.mp4";
    
    video.src = mediaSource01;
    video.autoPlay = false; // ensure that the video does not auto play
    video.loop = false; // set the video to loop.


	onMount(() => {
		// prepare canvas stores
		context = canvas.getContext('2d');

        // start game loop
		return initVideo();
	});

    function initVideo(){
        video.oncanplay = cacheVideo(frames, video);
        console.log("Video Loaded!");

        requestAnimationFrame(updateCanvas);
    }

    async function getVideoTrack(videoObj) {
        await videoObj.play();
        const [track] = videoObj.captureStream().getVideoTracks();
        videoObj.onended = (evt) => track.stop();
        console.log("Track opened !");
        return track;
    }

    async function cacheVideo(frameList, videoObj){ // this is a referance to the video
        if (window.MediaStreamTrackProcessor) {
            const track = await getVideoTrack(videoObj);
            const processor = new MediaStreamTrackProcessor(track);
            const reader = processor.readable.getReader();
            readChunk();
        
            function readChunk() {
                reader.read().then(async({ done, value }) => {
                    console.log(done);
                    if (value) {
                        const bitmap = await createImageBitmap(value);
                        const index = frameList.length;
                        frameList.push(bitmap);
                        //select.append(new Option("Frame #" + (index + 1), index));
                        value.close();
                    }
                    if (!done) {
                        readChunk();
                    } else {
                        // Once image is loaded into buffer
                        console.log("Loaded!");
                        console.log(frameList.length, " frames.");

                        videoReady = true;
                        // the video can be played so hand it off to the display function
                        //requestAnimationFrame(updateCanvas);
                    }
                });
            }
        } else {
          console.error("your browser doesn't support this API yet");
        }
    }

    function updateCanvas(){
        context.clearRect(0,0,canvas.width,canvas.height);

        // only draw if loaded and ready
        if(video !== undefined && videoReady){

            const frame = frames[nFrame];

            // now just draw the video the correct size
            //context.drawImage(frame, left, top, vidW * scale, vidH * scale);
            context.drawImage(frame, 0, 0, 800, 400)//, vidW*scale, vidH);

   
            // Get Icon back after filter
            context.globalAlpha = 1;  // reset alpha
            context.globalCompositeOperation = "source-over";  // reset comp*/

            if (nFrame < frames.length-1){
                nFrame = nFrame + 1;
            }
        }
        // all done for display 
        // request the next frame in 1/60th of a second
        requestAnimationFrame(updateCanvas);
    }


</script>

<canvas
    bind:this={canvas}
    width="800"
    height="400"
/>
