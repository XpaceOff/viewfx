

export default () => {
    onmessage = function(e) {
        //console.log('Message received from main script');
        const workerResult = 'Result: ' + (e.data[1]);

        const url ='http://'+e.data[1]+'/image_raw_data?src_img_type='+e.data[0].src_img_type+'&load_full_img='+e.data[0].load_full_img+'&img_full_path='+e.data[0].img_full_path+'&frame_number='+e.data[0].frame_number+'&canvas_w='+e.data[0].canvas_w+'&canvas_h='+e.data[0].canvas_h;
        const options = {
            method: "GET"
        };

        fetch(url, options)
        /*.then((response) => {
            console.log(response);
        });*/
        .then((response) => response.json())
        .then((data_from_rust) => {
            //console.log(data_from_rust);

            // Push an array of the image's raw data into rawImageFrames
            let raw = Uint8ClampedArray.from(data_from_rust.image_raw_data);
            let r_imgDimensions = data_from_rust.img_dimensions;
            let r_currentFrame = data_from_rust.frame_number;

            postMessage({
                image_raw_data: raw,
                img_dimensions: r_imgDimensions,
                frame_number: r_currentFrame
            });
        });

        console.log('Posting message back to main script');
        //postMessage(workerResult);
    }
}