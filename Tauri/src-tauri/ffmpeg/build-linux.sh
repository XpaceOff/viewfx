# Reference link
# https://trac.ffmpeg.org/wiki/CompilationGuide/Ubuntu

# install dependencies 
sudo apt-get update -qq && sudo apt-get -y install \
  autoconf \
  automake \
  build-essential \
  cmake \
  git-core \
  libass-dev \
  libfreetype6-dev \
  libgnutls28-dev \
  libmp3lame-dev \
  libsdl2-dev \
  libtool \
  libva-dev \
  libvdpau-dev \
  libvorbis-dev \
  libxcb1-dev \
  libxcb-shm0-dev \
  libxcb-xfixes0-dev \
  meson \
  ninja-build \
  pkg-config \
  texinfo \
  wget \
  yasm \
  zlib1g-dev

sudo apt install libunistring-dev libaom-dev libdav1d-dev

# Create the tmp directories
mkdir -p fmpeg_sources bin

# Install nasm
sudo apt-get install nasm

# Download FFmepeg and compile it.

cd fmpeg_sources
git clone https://git.ffmpeg.org/ffmpeg.git ffmpeg
cd ffmpeg

PATH="../../bin/:$PATH" PKG_CONFIG_PATH="../../ffmpeg_build/lib/pkgconfig" ./configure \
    --prefix="../../ffmpeg_build" \
    --pkg-config-flags="--static" \
    --ld="g++" \
    --bindir="../../bin" \
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
    --disable-ffplay \
    --disable-libxcb \
    --disable-sdl2 \
    --disable-xlib \

PATH="../../bin/:$PATH" make 

# Copy the FFmpeg binary to the right folder
cp ffmpeg ../../../bin/ffmpeg-x86_64-unknown-linux-gnu

hash -r 