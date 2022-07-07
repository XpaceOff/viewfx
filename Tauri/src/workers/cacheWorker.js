

export default () => {
    onmessage = function(e) {
        console.log('Message received from main script');
        const workerResult = 'Result: ' + (e.data[1]);

        const url ='http://'+e.data[1]+'/image_raw_data?src_img_type='+e.data[0].src_img_type+'&load_full_img='+e.data[0].load_full_img+'&img_full_path='+e.data[0].img_full_path+'&frame_number=20&canvas_w=746&canvas_h=420';
        const options = {
            method: "GET"
        };

        fetch(url, options)
        .then((response) => {
            console.log("YES!");
        });
        /*.then((response) => response.json())
        .then((data) => {
            console.log(data);
        });*/

        console.log('Posting message back to main script');
        postMessage(workerResult);
    }
}