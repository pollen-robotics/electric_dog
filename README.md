# Electric Dog
Replica of the original electric dog robot.

## Software Installation

### Rust nightly

The software is entirely written in Rust. To compile it you must have rust nightly installed on your machine. The easiest way is to use rustup:

```curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly```

### Xargo and cross compilation

You will also need to install xargo to cross compile the project for arm. At the moment the latest version (0.3.9) is broken and you should force to install the 0.3.8:

```cargo install xargo --vers 0.3.8 -f```

*Make sure you do not run this command from a rust project folder with a Xargo.toml file. Otherwise the build will fail.*

Add rust src via:

```rustup component add rust-src```

You will also need the cross compilation toolchain for you OS:

* on linux: ```sudo apt-get install openocd binutils-arm-none-eabi gdb-arm-none-eabi```
* on mac: ```brew cask install gcc-arm-embedded``` and ```brew install openocd```

*If the brew cask command doesn't work (Error: Unknown command: cask), then run brew tap Caskroom/tap first and try again.*

### Compile the program

Go to the electric_dog/software directory and run:

```xargo build --target thumbv6m-none-eabi --release --example follow-light-app```

You can replace the example by another example.

This should compile everything needed without error.

### Upload to the STM32F0Discovery

You will need two terminals to upload the program.

#### On the first one

```openocd -f $OPENCD_ROOT/interface/stlink-v2.cfg -f $OPENCD_ROOT/target/stm32f0x.cfg```

where ```OPENCD_ROOT``` should be set to the config folder for openocd.

It is typically:

* On linux: ```export OPENCD_ROOT=/usr/share/openocd/scripts```
* On mac: ```export OPENCD_ROOT=/usr/local/share/openocd/scripts```

#### On the second one

From the software folder where you built the project:

```arm-none-eabi-gdb target/thumbv6m-none-eabi/release/examples/follow-light-app```

That's all folks!
