<p align="center">
  <img src="public/ViewFX_md_v04.png" alt="ViewFX Logo" width="400"/>
</p>

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
