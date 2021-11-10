FROM bitriseio/android-ndk:latest
MAINTAINER Parity Technologies <admin@parity.io>

WORKDIR /build

ENV RUSTUP_HOME=/usr/local/rustup
ENV CARGO_HOME=/usr/local/cargo
ENV PATH=/usr/local/cargo/bin:$PATH

RUN apt-get -y update; apt-get -y install default-jdk
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV NDK_HOME=/opt/android-ndk
ENV JAVA_HOME=/usr/lib/jvm/java-11-openjdk-amd64/

COPY scripts /tmp/scripts/
RUN bash /tmp/scripts/init.sh

