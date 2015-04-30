# emlib
[![Build Status](https://travis-ci.org/RustyGecko/emlib.svg?branch=master)](https://travis-ci.org/RustyGecko/emlib)

Rust bindings for Silicon Labs emlib.

`emlib` compiles and runs on Silicon Labs EFM32 Microcontrollers. 
It's a proof-of-concept and work in progress, so currently only parts the
[EFM32 Giant Gecko](http://www.silabs.com/products/mcu/32-bit/efm32-giant-gecko/pages/efm32-giant-gecko.aspx)
family of their microcontrollers are supported.

The library have been tested on the following development kits:
* [STK3700 Starter Kit](http://www.silabs.com/products/mcu/lowpower/Pages/efm32gg-stk3700.aspx)
* [DK3750 Development Kit](http://www.silabs.com/products/mcu/lowpower/Pages/efm32gg-dk3750.aspx)

# Compiling emlib
`emlib` requires the following tools to build:
* [cargo-linkargs](https://github.com/RustyGecko/cargo-linkargs) - A Cargo subcommand used to link 
with an EFM32 linker script
* [ARM GCC Embedded Toolchain](https://launchpad.net/gcc-arm-embedded) - Used to build 
Silicon Labs emlib for the EFM32.
* [SEGGER JLink](https://www.segger.com/jlink-software.html) - Used for GDB debugging and to 
flash the EFM32's from the command line

If you're on linux, you can install the ARM GCC toolchain like this:
```bash
$ sudo add-apt-repository ppa:terry.guo/gcc-arm-embedded -y
$ sudo apt-get update -q
$ sudo apt-get install gcc-arm-none-eabi
```

## Compiling the examples
The project uses `make` to invoke `cargo-linkargs` and compile the library and the different 
[examples](https://github.com/RustyGecko/emlib/tree/master/examples).

Some common usage is shown below, look to the Makefile for all options.
```bash
$ make lib           # Compiles the library only
$ make               # Compiles the library and the default `buttons_int` example
$ make OUT=rtc_blink # Compiles the library and the `rtc_blink` example
$ make test          # Compiles the library and all examples
```

# Testing
The project also features a test-runner and a small framework for testing and verifying the `emlib` bindings.
The testing uses [Unity](https://github.com/ThrowTheSwitch/Unity) and [CMock](https://github.com/ThrowTheSwitch/CMock)
and requires `ruby` to be installed on the system in order to work.

The following builds the tests:
```bash
$ make clean-emlib
$ make build-tests
```

You can then run the tests on your STK3700 Starter Kit. This compiles the test runner, and flashes and
runs the tests on the EFM32.
The tests are configured to write output over a USART and can be used to help make sure that the bindings
are set up correctly.
```bash
$ make run-tests
```
