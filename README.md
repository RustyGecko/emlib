# emlib
[![Build Status](https://travis-ci.org/RustyGecko/emlib.svg?branch=master)](https://travis-ci.org/RustyGecko/emlib)

Rust bindings for Silicon Labs emlib.

# Installation and Compilation
In order to compile and use the library, the [ARM GCC Embedded Toolchain](https://launchpad.net/gcc-arm-embedded)
must be available in the path.

To install on Linux:
```bash
$ sudo add-apt-repository ppa:terry.guo/gcc-arm-embedded -y
$ sudo apt-get update -q
$ sudo apt-get install gcc-arm-none-eabi
```

# testing

Run `make run-tests`