NDK_STANDALONE = $(shell pwd)/../../NDK
ARCHS_IOS = armv7-apple-ios armv7s-apple-ios aarch64-apple-ios
ARCHS_ANDROID = aarch64-linux-android armv7-linux-androideabi i686-linux-android
LIB=libsigner.a

all: ios android

ios: $(LIB)

android: $(ARCHS_ANDROID)
	sh copy_android.sh

.PHONY: $(ARCHS_IOS)
$(ARCHS_IOS): %:
	cargo build --target $@ --release --no-default-features

aarch64-linux-android:
	PATH=$(PATH):$(NDK_STANDALONE)/arm64/bin \
	CC=$@-gcc \
	CXX=$@-g++ \
	cargo build --target $@ --release

armv7-linux-androideabi:
	PATH=$(PATH):$(NDK_STANDALONE)/arm/bin \
	CC=arm-linux-androideabi-gcc \
	CXX=arm-linux-androideabi-g++ \
	cargo build --target $@ --release

i686-linux-android:
	PATH=$(PATH):$(NDK_STANDALONE)/x86/bin \
	CC=i686-linux-android-gcc \
	CXX=i686-linux-android-g++ \
	cargo build --target $@ --release --lib

$(LIB): $(ARCHS_IOS)
	lipo -create -output $@ $(foreach arch,$(ARCHS_IOS),$(wildcard target/$(arch)/release/$(LIB)))
