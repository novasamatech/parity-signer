# Troubleshooting

## Rust side builds but app crashes on start (Android) or complains about symbols not found (ioS)

One common reason for this is inconsistency in `uniffi` version - make sure that installed version matches one stated in Cargo.toml

## Build for Android fails on macOS, build for iOS fails on linux

This is a known issue, does not seem to be solvable at the moment. Please use 2 machines, as we do.

