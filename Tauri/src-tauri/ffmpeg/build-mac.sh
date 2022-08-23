# Reference link
# https://trac.ffmpeg.org/wiki/CompilationGuide/macOS

# Install Xcode
xcode-select --install

# install dependencies 
brew install automake fdk-aac git lame libass libtool libvorbis libvpx \
opus sdl shtool texi2html theora wget x264 x265 xvid nasm

# Create the tmp directories
mkdir -p fmpeg_sources bin

cd fmpeg_sources
git clone https://git.ffmpeg.org/ffmpeg.git ffmpeg
cd ffmpeg

./configure \
    --cc=/usr/bin/clang \
    --prefix=/opt/ffmpeg \
    --extra-version=viewfx \
    --enable-static \
    --disable-shared \
    --arch=x86_64 \
    --disable-debug \
    --disable-doc \
    --disable-x86asm \
    --disable-gpl \
    --enable-version3 \
    --disable-libx264 \
    --pkg-config-flags=--static \
    --disable-ffplay \
    --disable-libxcb \
    --disable-sdl2 \
    --disable-xlib

make 

cp ffmpeg ../../../bin/ffmpeg-x86_64-apple-darwin