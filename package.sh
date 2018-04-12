rm -rf /tmp/tody-cli-brew-build
rm -rf /tmp/tody-cli-brew-build-tar
mkdir /tmp/tody-cli-brew-build-tar
mkdir /tmp/tody-cli-brew-build
cargo build --release
cp ./target/release/tody /tmp/tody-cli-brew-build
# cp ./config.json /tmp/tody-cli-brew-build
cp ./homebrew/configure /tmp/tody-cli-brew-build


# Change version number . . .
(cd /tmp/tody-cli-brew-build && tar -cvzf /tmp/tody-cli-brew-build-tar/tody-0.0.1.tar.gz .)

echo "Your shasum mate:\n"
shasum -a 256 /tmp/tody-cli-brew-build-tar/tody-0.0.1.tar.gz

cp -P /tmp/tody-cli-brew-build-tar/tody-0.0.1.tar.gz ./homebrew/Release/