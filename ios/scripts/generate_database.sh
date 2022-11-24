# Generate cold release database with built-in metadata

# XCode tries to be helpful and overwrites the PATH. Reset that.
PATH="$(bash -l -c 'echo $PATH')"

cd "$(dirname "${0}")/../../rust/generate_message"
env -i PATH="${PATH}" \
"${HOME}"/.cargo/bin/cargo run --locked make-cold-release

# Move database to assets

rm -rf ../../ios/NativeSigner/Database
mkdir ../../ios/NativeSigner/Database/
mkdir ../../ios/NativeSigner/Database/Database/
cp -R ../database/database_cold_release/ ../../ios/NativeSigner/Database/Database
