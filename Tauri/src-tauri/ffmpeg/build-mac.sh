

# WIP

git clone https://git.ffmpeg.org/ffmpeg.git ffmpeg
brew install fdk-aac

./configure --cc=/usr/bin/clang --prefix=/opt/ffmpeg --extra-version=viewfx --enable-static --disable-shared --arch=x86_64 --disable-debug --disable-doc --disable-x86asm --disable-gpl --enable-version3 --disable-libx264 --pkg-config-flags=--static --disable-ffplay --disable-libxcb --disable-sdl2 --disable-xlib