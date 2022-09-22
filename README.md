<p align="center">
  <img src="public/ViewFX_md_v04.png" alt="ViewFX Logo" width="400"/>
</p>

# What is ViewFX?

ViewFX is a multi-platform, lightweight video comparison tool, created to make checking post-production work easier. 

In post-production work, the client provides a *reference video* and a high resolution video. The artist then takes the high resolution video and apply effects to achieve the result requested. Once the job is done, the artist should always check their work before delivering it. 

I know from experience that a lot of artists in the field don't check their work before delivery because it can very time-consuming for After Effects or Nuke to finish caching every effect. Autodesk has an app called RV Viewer, but the problem with it is that it takes too long to open and too long to load. It also does not have the option to compare two videos side by side simultaneosly. This is where ViewFX comes in. 

# What ViewFX offers

- It is lightweight, with a size of under 20mb.
- It allows you to play the video while it is caching on the background.
- It allows you to compare two videos side by side in the following two modes:
    - The `A/B` mode:
    - The `Diff` mode:

# What ViewFX doesn't do

- It is *not* a video player to watch movies with.
- It is *not* a music player.
- It does *not* play audio.

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

Once you have Rust and all the dependencies installed, clone the repository and `cd` to the folder. Inside of it:

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
