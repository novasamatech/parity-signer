FROM ghcr.io/actions/runner-images/ubuntu22:latest

USER root
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    libjpeg-dev \
    libtiff-dev \
    libpng-dev \
    libavcodec-dev \
    libavformat-dev \
    libswscale-dev \
    libv4l-dev \
    libxvidcore-dev \
    libx264-dev \
    libgtk-3-dev \
    libatlas-base-dev \
    gfortran \
    python3-dev \
    unzip \
    wget \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /tmp

RUN wget -O opencv.zip https://github.com/opencv/opencv/archive/4.11.0.zip && \
    wget -O opencv_contrib.zip https://github.com/opencv/opencv_contrib/archive/4.11.0.zip && \
    unzip opencv.zip && \
    unzip opencv_contrib.zip && \
    cd opencv-4.11.0 && \
    mkdir build && cd build && \
    cmake -D CMAKE_BUILD_TYPE=RELEASE \
        -D CMAKE_INSTALL_PREFIX=/usr/local \
        -D OPENCV_ENABLE_NONFREE=ON \
        -D OPENCV_EXTRA_MODULES_PATH=../../opencv_contrib-4.11.0/modules \
        -D BUILD_EXAMPLES=OFF .. && \
    make -j2 && \
    make install && \
    ldconfig && \
    cd /tmp && \
    rm -rf opencv* && \
    apt-get clean

WORKDIR /build 