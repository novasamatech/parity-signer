FROM rust
MAINTAINER Parity Technologies <admin@parity.io>

WORKDIR /build

RUN su -c 'curl -sL https://deb.nodesource.com/setup_14.x | bash -'
RUN apt-get update -y; apt-get install -y openjdk-11-jdk-headless wget curl unzip zipalign nodejs
RUN curl https://dl.google.com/android/repository/commandlinetools-linux-6858069_latest.zip > /tmp/cli-tools.zip
RUN unzip /tmp/cli-tools.zip -d /opt
RUN yes | ANDROID_SDK_ROOT=/opt/cmdline-tools/bin/ /opt/cmdline-tools/bin/sdkmanager --licenses --sdk_root=/opt/cmdline-tools/bin/
RUN npm install -g react-native-cli yarn
RUN curl https://dl.google.com/android/repository/android-ndk-r21e-linux-x86_64.zip > /tmp/ndk.zip
RUN unzip /tmp/ndk.zip -d /opt
ENV NDK_HOME /opt/android-ndk-r21e/
ENV ANDROID_SDK_ROOT /opt/cmdline-tools/bin/

WORKDIR /tmp/scripts
COPY scripts /tmp/scripts/
RUN bash /tmp/scripts/init.sh
WORKDIR /build

# cleanup
RUN apt-get autoremove -y
RUN apt-get clean -y
RUN rm -rf /tmp/* /var/tmp/*
