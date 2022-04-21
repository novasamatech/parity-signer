# This dockerfile is based on Bitrise but with a lot of extra cruft we don't need removed
FROM ubuntu:focal
LABEL maintainer="Parity Technologies <admin@parity.io>"

ENV ANDROID_SDK_ROOT /opt/android-sdk-linux
ENV ANDROID_HOME /opt/android-sdk-linux
ENV NDK_HOME /opt/android-ndk
ENV JAVA_HOME /usr/lib/jvm/java-11-openjdk-amd64/

RUN apt-get -y update && apt-get -y install default-jdk wget unzip curl clang
RUN apt-get -y install python git

# SDK setup is taken from https://github.com/bitrise-io/android/blob/master/Dockerfile

RUN cd /opt \
    && wget -q https://dl.google.com/android/repository/commandlinetools-linux-8092744_latest.zip -O android-commandline-tools.zip \
    && mkdir -p ${ANDROID_SDK_ROOT}/cmdline-tools \
    && unzip -q android-commandline-tools.zip -d /tmp/ \
    && mv /tmp/cmdline-tools/ ${ANDROID_SDK_ROOT}/cmdline-tools/latest \
    && rm android-commandline-tools.zip && ls -la ${ANDROID_SDK_ROOT}/cmdline-tools/latest/

ENV PATH ${PATH}:${ANDROID_SDK_ROOT}/platform-tools:${ANDROID_SDK_ROOT}/cmdline-tools/latest/bin

RUN yes | sdkmanager --licenses
# We need at least one set of build-tools installed for apksigner
RUN yes | sdkmanager "build-tools;30.0.3"
RUN echo "y" | sdkmanager --install "ndk;24.0.8215888" --sdk_root=${ANDROID_SDK_ROOT}

# rust stuff

ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH

WORKDIR /build

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# install additional rust targets
RUN rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android

COPY scripts /tmp/scripts/
RUN bash /tmp/scripts/init.sh

# Cleanup
RUN rm -rf /tmp/scripts ;\
    apt-get autoremove -y ; \
    apt-get clean ; \
    rm -rf /var/lib/apt/lists/*
