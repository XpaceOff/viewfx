# Research

## Things that might be useful for the future

### OpenGl related

If you have problem drawing images in cavas you might want to look at this options:

- Using THREE.js for drwing the image:
  - https://stackoverflow.com/questions/32924749/create-texture-from-array-three-js
  - https://threejs.org/docs/#api/en/textures/DataTexture

- You might wan to look for WebGL

### Web framefork 

As 05/26/22, I am having problems sending the raw data of images from rust to js.
Tauri developers are sying that the IPC will improve by Tauri v2. Until then I might 
use some we frameworks to send the data.

Check this [discussion](https://github.com/tauri-apps/tauri/discussions/4191)

- [Rocket](https://rocket.rs/)
- [Axum](https://github.com/tokio-rs/axum) (recommneded and use by the developer)

Aside

https://www.reddit.com/r/rust/comments/c313jf/why_people_use_both_yew_and_rocket_or_actix/
- [Yew](https://yew.rs/) (Frontend)
- [Actix](https://actix.rs/) (Backend)
- [Rocket](https://rocket.rs/) (Backend)