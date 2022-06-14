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

### Video decoder

- FFmpeg
  - https://stackoverflow.com/questions/43859091/using-ffmpeg-shared-library-in-a-commercial-c-c-application
  - https://softwareengineering.stackexchange.com/questions/86142/what-exactly-do-i-need-to-do-if-i-use-a-lgpl-licensed-library
  - https://trac.ffmpeg.org/wiki/CompilationGuide
  - [ffmpeg doc](https://ffmpeg.org/ffmpeg.html#filter_005foption)
  - https://superuser.com/questions/1047660/ffmpeg-pipe-images-extracted-from-video

command to get an specific frame from video:

```bash
$ .\ffmpeg.exe -i .\SC_07.mov -vf "select=eq(n\,10)" -f image2pipe -vframes 1 -
```

`-i` input file.
`vf` filter specific frame (frame 10 in this case)
`-f` this allows me to send image through pipes instead of saving it into a file
`-` this is the output