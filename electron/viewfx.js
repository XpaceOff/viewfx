let video, canvas, canvasContext, tmpCanvas, tmpCanvasContext; 

function computeFrame() {
  var $this = this; //cache

  if (!$this.paused && !$this.ended) {
    tmpCanvasContext.drawImage(video, 0, 0, 400, 225);
    let frame = tmpCanvasContext.getImageData(0, 0, video.videoWidth, video.videoHeight);

    for (let i = 0; i < frame.data.length /4; i++) {
      let r = frame.data[i * 4 + 0];
      let g = frame.data[i * 4 + 1];
      let b = frame.data[i * 4 + 2];

      if (r > 70 && r < 160 && g > 95 && g < 220 && b > 25 && b < 150) 
        frame.data[i * 4 + 3] = 0;
    }

    canvasContext.putImageData(frame, 0, 0);
    setTimeout(computeFrame, 0);
  }
}

function init() {
  video = document.getElementById('video');

  canvas = document.getElementById('output-canvas');
  canvasContext = canvas.getContext('2d');

  tmpCanvas = document.createElement('canvas');
  tmpCanvas.setAttribute('width', 400);
  tmpCanvas.setAttribute('height', 225);
  tmpCanvasContext = tmpCanvas.getContext('2d');

  //video.addEventListener('play', computeFrame );

  /*video.addEventListener('play', function() {
    var $this = this; //cache
    (function loop() {
      if (!$this.paused && !$this.ended) {
        canvasContext.drawImage($this, 0, 0, 400, 225);
        setTimeout(loop, 1000 / 60); // drawing at 60fps
      }
    })();
  }, 0);*/

  video.addEventListener('play', () => {
    function step() {
      canvasContext.drawImage(video, 0, 0, canvas.width, canvas.height)
      requestAnimationFrame(step)
    }
    requestAnimationFrame(step);
  })

}


document.addEventListener("DOMContentLoaded", () => {
    //alert("HELLO");
    init();
});