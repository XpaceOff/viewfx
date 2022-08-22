
# Dev setup

## MacOS

Install the dependencies 
```bash
xcode-select --install
```

Install rust
```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

## Linux (Ubuntu)

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


# Install the app npm dependencies 

cd into the project and type:
```bash
npm install
```

## Start a new Tauri project

```bash
# Create a new tauri app
$ npx create-tauri-app

# Start tauri development window
$ npm run tauri dev

# Build an Tauri app
$ npm run tauri build
```

## Keep Tauri dependencies up to date

```bash
# If you use yarn
$ yarn upgrade @tauri-apps/cli @tauri-apps/api --latest

# if you use npm
$ npm install @tauri-apps/cli@latest @tauri-apps/api@latest

# Detect versions
$ npm outdated @tauri-apps/cli
$ yarn outdated @tauri-apps/cli
```

## Install Tailwind on my Svelte project

https://css-tricks.com/how-to-use-tailwind-on-a-svelte-site/

```bash
$ npm install -D tailwindcss postcss autoprefixer svelte-preprocess
$ npx tailwindcss init tailwind.config.js 
```

tailwind.config.js
```css
module.exports = {
    //The content section is where you configure the paths to all 
    // files that contain Tailwind class names.
    content: ['./src/**/*.{html,js,svelte,ts}'],
    theme: {
        extend: {},
    },
    plugins: [],
}
```

rollup.config.js
```js
import sveltePreprocess from "svelte-preprocess";

plugins: [
		svelte({
			preprocess: sveltePreprocess({
				sourceMap: !production,
				postcss: {
				  plugins: [
				   require("tailwindcss"), 
				   require("autoprefixer"),
				  ],
				},
			}),
```

---

## How I compiled ffmpeg

### On MacOS

```bash
$ git clone https://git.ffmpeg.org/ffmpeg.git ffmpeg
$ brew install fdk-aac

$ ./configure  --prefix=/usr/local --disable-gpl --disable-nonfree --enable-libass --enable-libfdk-aac --enable-libfreetype --enable-libmp3lame --enable-libtheora --enable-libvorbis --enable-libvpx --disable-libx264 --disable-libx265 --enable-libopus --disable-libxvid --disable-chromaprint --enable-libopenjpeg --enable-libaom --extra-ldflags=-L/usr/local/lib --samples=fate-suite/

# MacOS - v02
./configure --cc=/usr/bin/clang --prefix=/opt/ffmpeg --extra-version=viewfx --enable-static --disable-shared --arch=x86_64 --disable-debug --disable-doc --disable-x86asm --disable-gpl --enable-version3 --disable-libx264 --pkg-config-flags=--static --disable-ffplay --disable-libxcb --disable-sdl2 --disable-xlib

# Check more options here -> https://github.com/FFmpeg/FFmpeg/blob/master/configure

$ make
```
### On Win10 

For now lets just use https://github.com/BtbN/FFmpeg-Builds/releases
until I figure out how to build it on windows
Use the one called `ffmpeg-master-[VER]-win64-lgpl`

```bash
$ pacman -S make
$ pacman -S diffutils
$ pacman -S yasm


$ git clone https://git.ffmpeg.org/ffmpeg.git ffmpeg

C:\msys64\usr\bin\bash -lc "pacman -S nasm"

./configure --prefix=/usr/local --extra-version=viewfx --arch=x86_64 --target-os=mingw32 --cross-prefix=x86_64-w64-mingw32- --enable-static --disable-shared --disable-debug --disable-doc --disable-x86asm --disable-gpl --enable-version3 --disable-libx264 --pkg-config-flags=--static --disable-ffplay --disable-libxcb --disable-sdl2 --disable-xlib
```
pacman -S pacman -S mingw-w64-x86_64-toolchain
pacman -S autoconf automake binutils cmake doxygen git libtool make mercurial nasm pkg-config subversion texinfo yasm


CFLAGS=-I/mingw64/include &&
LDFLAGS=-L/mingw64/lib &&
export PKG_CONFIG_PATH= &&
export PKG_CONFIG_LIBDIR=/mingw64/lib/pkgconfig &&
./configure \
--arch=x86_64 \
--target-os=mingw64 \
--cross-prefix=x86_64-w64-mingw64- \
--prefix=/usr/local \
--pkg-config=pkg-config \
--pkg-config-flags=--static \
--extra-cflags=-static \
--extra-ldflags=-static \
--enable-static --disable-shared --disable-debug --disable-doc --disable-x86asm --disable-gpl --enable-version3 --disable-libx264 --disable-ffplay --disable-libxcb --disable-sdl2 --disable-xlib

--extra-libs="-lm -lz -fopenmp" \
--enable-static \
--disable-shared \
--enable-nonfree \
--enable-gpl \
--enable-avisynth \
--enable-libaom \
--enable-libfdk-aac \
--enable-libfribidi \
--enable-libmp3lame \
--enable-libopus \
--enable-libsoxr \
--enable-libvorbis \
--enable-libvpx \
--enable-libx264 \
--enable-libx265 &&
make -j$(nproc) &&
make install