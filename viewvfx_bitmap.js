let frames = [];
let nFrame = 0;
let mediaSource01 = "http://upload.wikimedia.org/wikipedia/commons/7/79/Big_Buck_Bunny_small.ogv";
let mediaSource02 = "media/pro01Blur.mp4";
let muted = true;

document.addEventListener("DOMContentLoaded", () => {
    mediaSource01 = "media/pro01.mp4";
    mediaSource02 = "media/pro01.mp4";

    let canvas = document.getElementById('output-canvas');  // get the canvas from the page
    let canvasContext = canvas.getContext("2d");            // Create the canvas context to paint on
    let videoContainer, videoContainer02;                   // Object to hold video and associated info
    let videoA = document.createElement("video");          // create the video A element
    let videoB = document.createElement("video");            // create the video B element

    videoA.src = mediaSource01;
    videoB.src = mediaSource02;

    // the video will now begin to load.
    // As some additional info is needed we will place the video in a
    // containing object for convenience
    videoA.autoPlay = false; // ensure that the video does not auto play
    videoA.loop = false; // set the video to not loop.
    videoA.muted = muted;

    videoB.autoPlay = false;
    videoB.loop = false;
    videoB.muted = true;

    videoContainer = {  // we will add properties as needed
        video : videoA,
        ready : false,
        isPause : true 
    };
    videoContainer02 = {  // we will add properties as needed
        video : videoB,
        ready : false,   
    };

    //videoB.oncanplay = cacheVideo02;
    videoA.oncanplay = cacheVideo; // set the event to the play function that can be found below
    
    function cacheVideo02(event){
        videoContainer02.ready = true;
    }

    async function getVideoTrack() {
        await videoContainer.video.play();
        const [track] = videoA.captureStream().getVideoTracks();
        videoA.onended = (evt) => track.stop();
        return track;
    }

    async function cacheVideo(event){ // this is a referance to the video
        if (window.MediaStreamTrackProcessor) {
            const track = await getVideoTrack();
            const processor = new MediaStreamTrackProcessor(track);
            const reader = processor.readable.getReader();
            readChunk();
        
            function readChunk() {
                reader.read().then(async({ done, value }) => {
                    console.log(done);
                    if (value) {
                        const bitmap = await createImageBitmap(value);
                        const index = frames.length;
                        frames.push(bitmap);
                        //select.append(new Option("Frame #" + (index + 1), index));
                        value.close();
                    }
                    if (!done) {
                        readChunk();
                    } else {
                        // Once image is loaded into buffer
                        console.log("Loaded!");
                        console.log(frames.length, " frames.");

                        // the video may not match the canvas size so find a scale to fit
                        videoContainer.scale = Math.min(
                            canvas.width / this.videoWidth, 
                            canvas.height / this.videoHeight
                        );

                        videoContainer.ready = true;
                        // the video can be played so hand it off to the display function
                        requestAnimationFrame(updateCanvas);
                    }
                });
            }
        } else {
          console.error("your browser doesn't support this API yet");
        }
    }

    function updateCanvas(){
        canvasContext.clearRect(0,0,canvas.width,canvas.height);

        // only draw if loaded and ready
        if(videoContainer !== undefined && videoContainer.ready){ 
            // find the top left of the video on the canvas
            var scale = videoContainer.scale;
            var vidH = videoContainer.video.videoHeight;
            var vidW = videoContainer.video.videoWidth;
            var top = canvas.height / 2 - (vidH /2 ) * scale;
            var left = canvas.width / 2 - (vidW /2 ) * scale;

            const frame = frames[nFrame];

            // now just draw the video the correct size
            //canvasContext.drawImage(frame, left, top, vidW * scale, vidH * scale);
            canvasContext.drawImage(frame, 0, 0, 800, 400)//, vidW*scale, vidH);

            // Black and White Filter
            let mixAmount = 1; // filter mix
            /*canvasContext.fillStyle = "#888"; // gray colour
            canvasContext.globalAlpha = mixAmount;   // amount of FX
            canvasContext.globalCompositeOperation = "color";  // The comp setting to do BLACK/WHITE
            canvasContext.fillRect(0,0, vidW, vidH);*/

            // Diff Code
            //canvasContext.globalAlpha = mixAmount;   // amount of FX
            //canvasContext.globalCompositeOperation = "difference"; 
            //canvasContext.drawImage(videoContainer02.video, left, top, vidW * scale, vidH * scale);
            //canvasContext.drawImage(canvasContext.canvas, left, top, vidW * scale, vidH * scale);

            // Get Icon back after filter
            canvasContext.globalAlpha = 1;  // reset alpha
            canvasContext.globalCompositeOperation = "source-over";  // reset comp*/

            if (!videoContainer.isPause){
                if (nFrame < frames.length-1){
                    nFrame = nFrame + 1;
                }
                else{
                    nFrame = 1;
                    videoContainer.isPause = true;
                }
            }

            if(videoContainer.isPause){ // if not playing show the paused screen 
                drawPlayIcon();
            }
        }
        // all done for display 
        // request the next frame in 1/60th of a second
        requestAnimationFrame(updateCanvas);
    }

    function drawPlayIcon(){
        canvasContext.fillStyle = "black";  // darken display
        canvasContext.globalAlpha = 0.5;
        canvasContext.fillRect(0,0,canvas.width,canvas.height);
        canvasContext.fillStyle = "#DDD"; // colour of play icon
        canvasContext.globalAlpha = 0.75; // partly transparent
        canvasContext.beginPath(); // create the path for the icon
        var size = (canvas.height / 2) * 0.5;  // the size of the icon
        canvasContext.moveTo(canvas.width/2 + size/2, canvas.height / 2); // start at the pointy end
        canvasContext.lineTo(canvas.width/2 - size/2, canvas.height / 2 + size);
        canvasContext.lineTo(canvas.width/2 - size/2, canvas.height / 2 - size);
        canvasContext.closePath();
        canvasContext.fill();
        canvasContext.globalAlpha = 1; // restore alpha
    }    

    function playPauseClick(){
        if(videoContainer !== undefined && videoContainer.ready){
            if(videoContainer.isPause){                                 
                videoContainer.isPause = false;
            }else{
                videoContainer.isPause = true;
            }
        }
    }

    // register the event
    canvas.addEventListener("click",playPauseClick);
});
