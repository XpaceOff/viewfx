<p align="center">
  <img src="public/ViewFX_md_v04.png" alt="ViewFX Logo" width="400"/>
</p>

# What is ViewFX?

ViewFX was created to make checking post-production work easier. 

In post-production work, the client provides a *reference video* and a high resolution video. The artist then takes the high resolution video and apply effects to achieve the result requested. Once the job is done, the artist should always check their work before delivering it. 

I know from experience that a lot of artists in the field don't check their work before delivery because they have to wait for After Effects or Nuke to finish caching every effect and ending wasting a lor of time. Autodesk created an app called RV viewer. but the problem with it is that it takes too long to open and too long to load. it also does not give an option to compare two videos at the same time. Which is the main reason to cheese ViewFX.

# What ViewFX is not and won't do

- Is not a video player to watch movies on.
- Is not a music player.
- It won't play audio.

# What ViewFX offers

- It comes in a very light weigh executable, with a size of under 20mb.
- it will lwt you play the video while is caching on the background.
- it will let you load a second video to be compared in two differenct modes:
    - The `A/B` mode:
    - The `Diff` mode:

---

# Dev setup

* ## MacOS

    Install the dependencies 
    ```bash
    xcode-select --install
    ```

    Install rust
    ```bash
    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
    ```

* ## Linux (Ubuntu)

    Install the dependencies
    ```bash
    sudo apt update
    sudo apt install libwebkit2gtk-4.0-dev \
        build-essential \
        curl \
        wget \
        libssl-dev \
        libgtk-3-dev \
        libayatana-appindicator3-dev \
        librsvg2-dev \
        cmake
    ```

    Install rust
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

Once you have rust and all the dependencies installed, clone the repository and `cd` to the folder. Inside of it:

To install the ViewFX npm dependencies:
```bash
npm install
```

Once that is done, you can run:

```bash
# Start ViewFX on development mode:
$ npm run tauri dev

# Or, if you want to build ViewFX
$ npm run tauri build
```

## Keep Tauri dependencies up to date

```bash
# If you use yarn
$ yarn upgrade @tauri-apps/cli @tauri-apps/api --latest

# If you use npm
$ npm install @tauri-apps/cli@latest @tauri-apps/api@latest

# Detect versions
$ npm outdated @tauri-apps/cli
$ yarn outdated @tauri-apps/cli
```

---

# How to contribute
