ARCHS = i386-apple-ios x86_64-apple-ios armv7-apple-ios armv7s-apple-ios aarch64-apple-ios
LIB = libsigner.a

all: $(LIB)

.PHONY: $(ARCHS)
$(ARCHS): %:
	cargo build --target $@ --release

$(LIB): $(ARCHS)
	lipo -create -output $@ $(foreach arch,$(ARCHS),$(wildcard target/$(arch)/release/$(LIB)))
