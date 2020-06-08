ARCHS_IOS = x86_64-apple-ios armv7-apple-ios armv7s-apple-ios aarch64-apple-ios
ARCHS_ANDROID = aarch64-linux-android armv7-linux-androideabi i686-linux-android
LIB=libsigner.a
API_LEVEL=$(shell grep API_LEVEL ../../scripts/variables.sh |tr -c -d '[:digit:]')
OS_ARCH=$(shell uname | tr '[:upper:]' '[:lower:]')
ANDROID_PREBUILD_BIN="${NDK_HOME}/toolchains/llvm/prebuilt/${OS_ARCH}-x86_64/bin"

all: ios android

ios: $(LIB)

android: $(ARCHS_ANDROID)
	sh copy_android.sh

.PHONY: $(ARCHS_IOS)
$(ARCHS_IOS): %:
	cargo build --target $@ --release --no-default-features

aarch64-linux-android:
	CC_aarch64_linux_android="${ANDROID_PREBUILD_BIN}/aarch64-linux-android${API_LEVEL}-clang" \
	CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="${ANDROID_PREBUILD_BIN}/aarch64-linux-android${API_LEVEL}-clang" \
	AR_aarch64_linux_android="${ANDROID_PREBUILD_BIN}/aarch64-linux-android-ar" \
	cargo build --target $@ --release

armv7-linux-androideabi:
	CC_armv7_linux_androideabi="${ANDROID_PREBUILD_BIN}/armv7a-linux-androideabi${API_LEVEL}-clang" \
    CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER="${ANDROID_PREBUILD_BIN}/armv7a-linux-androideabi${API_LEVEL}-clang" \
    AR_armv7_linux_androideabi="${ANDROID_PREBUILD_BIN}/arm-linux-androideabi-ar" \
	cargo build --target $@ --release

i686-linux-android:
	CC_i686_linux_android="${ANDROID_PREBUILD_BIN}/i686-linux-android${API_LEVEL}-clang" \
	CARGO_TARGET_I686_LINUX_ANDROID_LINKER="${ANDROID_PREBUILD_BIN}/i686-linux-android${API_LEVEL}-clang" \
	AR_i686_linux_android="${ANDROID_PREBUILD_BIN}/i686-linux-android-ar" \
	cargo build --target $@ --release --lib

$(LIB): $(ARCHS_IOS)
	lipo -create -output $@ $(foreach arch,$(ARCHS_IOS),$(wildcard target/$(arch)/release/$(LIB)))
