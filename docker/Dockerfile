# This dockerfile is based on Bitrise but with a lot of extra cruft we don't need removed
FROM ubuntu:focal
LABEL maintainer="Parity Technologies <devops-team@parity.io>"

ENV ANDROID_SDK_ROOT /opt/android-sdk-linux
ENV ANDROID_HOME /opt/android-sdk-linux
ENV NDK_HOME /opt/android-ndk
ENV JAVA_HOME /usr/lib/jvm/java-11-openjdk-amd64/
# skip prompts
ENV DEBIAN_FRONTEND=noninteractive
# sdk env
ENV PATH ${PATH}:${ANDROID_SDK_ROOT}/platform-tools:${ANDROID_SDK_ROOT}/cmdline-tools/latest/bin
# Rust env
ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH

COPY scripts /tmp/scripts/

RUN apt-get -y update && \
    apt-get -y install python git default-jdk wget unzip curl clang && \
# SDK setup is taken from https://github.com/bitrise-io/android/blob/master/Dockerfile
    cd /opt && \
    wget -q https://dl.google.com/android/repository/commandlinetools-linux-8092744_latest.zip -O android-commandline-tools.zip && \
    mkdir -p ${ANDROID_SDK_ROOT}/cmdline-tools && \
    unzip -q android-commandline-tools.zip -d /tmp/ && \
    mv /tmp/cmdline-tools/ ${ANDROID_SDK_ROOT}/cmdline-tools/latest && \
    rm android-commandline-tools.zip && ls -la ${ANDROID_SDK_ROOT}/cmdline-tools/latest/ && \
# We need at least one set of build-tools installed for apksigner
    yes | sdkmanager --licenses && \
    yes | sdkmanager "build-tools;30.0.3" && \
    echo "y" | sdkmanager --install "ndk;24.0.8215888" --sdk_root=${ANDROID_SDK_ROOT} && \
# rust stuff
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
# install additional rust targets
    rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android && \
    bash /tmp/scripts/init.sh && \
# versions
    rustup show && \
    cargo --version && \
# cleanup
    rm -rf /tmp/scripts ;\
    apt-get autoremove -y && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/* && \
    # cargo clean up
    # removes compilation artifacts cargo install creates (>250M)
    rm -rf "${CARGO_HOME}/registry" "${CARGO_HOME}/git"

WORKDIR /build
