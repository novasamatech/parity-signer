# This dockerfile is based on Bitrise but with a lot of extra cruft we don't need removed
FROM ubuntu:focal
MAINTAINER Parity Technologies <admin@parity.io>

ENV ANDROID_SDK_ROOT /opt/android-sdk-linux
ENV ANDROID_HOME /opt/android-sdk-linux
ENV NDK_HOME /opt/android-ndk
ENV ANDROID_NDK_VERSION r22
ENV JAVA_HOME /usr/lib/jvm/java-11-openjdk-amd64/

RUN apt-get -y update && apt-get -y install default-jdk wget unzip curl clang

# SDK setup is taken from https://github.com/bitrise-io/android/blob/master/Dockerfile
# NDK setup is taken from https://github.com/bitrise-io/android-ndk/blob/master/Dockerfile

RUN cd /opt \
    && wget -q https://dl.google.com/android/repository/commandlinetools-linux-6858069_latest.zip -O android-commandline-tools.zip \
    && mkdir -p ${ANDROID_SDK_ROOT}/cmdline-tools \
    && unzip -q android-commandline-tools.zip -d /tmp/ \
    && mv /tmp/cmdline-tools/ ${ANDROID_SDK_ROOT}/cmdline-tools/latest \
    && rm android-commandline-tools.zip && ls -la ${ANDROID_SDK_ROOT}/cmdline-tools/latest/

ENV PATH ${PATH}:${ANDROID_SDK_ROOT}/platform-tools:${ANDROID_SDK_ROOT}/cmdline-tools/latest/bin

RUN mkdir /opt/android-ndk-tmp && \
    cd /opt/android-ndk-tmp && \
    wget -q https://dl.google.com/android/repository/android-ndk-${ANDROID_NDK_VERSION}-linux-x86_64.zip && \
# uncompress
    unzip -q android-ndk-${ANDROID_NDK_VERSION}-linux-x86_64.zip && \
# move to its final location
    mv ./android-ndk-${ANDROID_NDK_VERSION} ${NDK_HOME} && \
# remove temp dir
    cd ${NDK_HOME} && \
    rm -rf /opt/android-ndk-tmp

# add to PATH
ENV PATH ${PATH}:${NDK_HOME}

# rust stuff

ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH

WORKDIR /build

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

COPY scripts /tmp/scripts/
RUN bash /tmp/scripts/init.sh

# Cleanup
RUN rm -rf /tmp/scripts ;\
    apt-get autoremove -y ; \
    apt-get clean ; \
    rm -rf /var/lib/apt/lists/*
