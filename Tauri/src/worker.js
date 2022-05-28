import axios from "axios";

export default () => {
    onmessage = ({data}) => {
        console.log('Message received from main script');
        //console.log(data);
        console.log('---------------------------------');

        let a = data[0];
        let b = data[1];
        let c = data[2];

        console.log(a, b, c);

        
        axios.get('http://localhost:3000/image_raw_data', {
            headers: {'Access-Control-Allow-Origin': '*'},
            params: {
                frame_number: frameNumber,
                canvas_w: canvasW,
                canvas_h: canvasH
            }
        })
        .then(function (data_from_rust) {
            // Push an array of the image's raw data into rawImageFrames
            let raw = Uint8ClampedArray.from(data_from_rust[0]);
            rawImageFrames.push([raw, data_from_rust[2]]);
            //rawImageFrames.push([data_from_rust[0], data_from_rust[2]]);

            //console.log("Frame", data_from_rust[1] - frameStart, "in pos", rawImageFrames.length-1, "will be saved in", data_from_rust[1] - frameStart);
            // Save the right order of frames
            rawImageFramesOrder[data_from_rust[1] - frameStart] = rawImageFrames.length - 1;

            // Update the bar cache status to 1 (cached)
            $barFrameCacheStatus[data_from_rust[1] - frameStart] = 2;

            framesCached += 1;
        })
        .catch(function (error) {
            console.log(error);
        });



        console.log('Posting message back to main script');
        postMessage(data[0]+1);
    }
}
  