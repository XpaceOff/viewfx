export default () => {
    var controller = new AbortController();

    onmessage = function(e) {
        //console.log('Message received from main script');

        if (e.data.abortSignal !== undefined) {
            console.log("Aborting Fetch...");
            if (e.data.abortSignal) {
                controller.abort();
            }
        }
        else{

            let bridge_url = '';
            if (e.data[0].src_img_type == "FROM_VIDEO")
                bridge_url = 'http://'+e.data[1]+'/video_frame?load_full_img='+e.data[0].load_full_img+'&img_full_path='+e.data[0].img_full_path+'&frame_number='+e.data[0].frame_number+'&canvas_w='+e.data[0].canvas_w+'&canvas_h='+e.data[0].canvas_h;
            else
                bridge_url = 'http://'+e.data[1]+'/image_raw_data?src_img_type='+e.data[0].src_img_type+'&load_full_img='+e.data[0].load_full_img+'&img_full_path='+e.data[0].img_full_path+'&frame_number='+e.data[0].frame_number+'&canvas_w='+e.data[0].canvas_w+'&canvas_h='+e.data[0].canvas_h;
    
            const options = {
                method: "GET",
                signal: controller.signal
            };

            //console.log("Requesting image data...");
    
            fetch(bridge_url, options)
            .then((response) => {
                if(!response.ok){
                    throw new Error("Couldn't get the frame data");
                }
    
                return response.json();
            })
            .then((data_from_rust) => {
                //console.log(data_from_rust);
    
                // Push an array of the image's raw data into rawImageFrames
                let raw = Uint8ClampedArray.from(data_from_rust.image_raw_data);
                let r_imgDimensions = data_from_rust.img_dimensions;
                let r_currentFrame = data_from_rust.frame_number;
    
                postMessage({
                    error: false,
                    image_raw_data: raw,
                    img_dimensions: {
                        width: r_imgDimensions[0],
                        height: r_imgDimensions[1]
                    },
                    frame_number: r_currentFrame
                });

                close();
            })
            .catch((error) => {
    
                postMessage({
                    error: true,
                    error_type: error.message,
                    frame_number: e.data[0].frame_number
                });

                close();
            });

        }

        //console.log('Posting message back to main script');
    }
}