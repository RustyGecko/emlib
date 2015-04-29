#!/bin/bash

set -x
set -e

if [ "${TRAVIS_OS_NAME}" = "osx" ] || [ "${PLATFORM}" = "mac" ] || [ "`uname`" = "Darwin" ]; then
    target=apple-darwin
elif [ "${OS}" = "Windows_NT" ] || [ "${PLATFORM}" = "win" ]; then
    windows=1
else
    target=unknown-linux-gnu
fi

if [ "${TRAVIS}" = "true" ] && [ "${target}" = "unknown-linux-gnu" ]; then
    # Install gcc-arm-none-eabi for linux
    sudo add-apt-repository ppa:terry.guo/gcc-arm-embedded -y
    sudo apt-get update -q
    sudo apt-get install gcc-arm-none-eabi
fi

# Install cargo-linkargs in same directory
wget https://github.com/RustyGecko/cargo-linkargs/releases/download/v0.1.0-tmp/cargo-linkargs -q
chmod +x cargo-linkargs
sudo mv cargo-linkargs /usr/local/bin/
which cargo-linkargs

set +x
