## Smile

A simple rust program for drawing a smiley and blinking LED for ESP32-C6+LCD from [waveshare](https://www.waveshare.com/esp32-c6-lcd-1.47.htm).

## How to Setup?
First install [rustup](https://rustup.rs/)

Now, install the required [riscv](https://docs.espressif.com/projects/rust/book/getting-started/toolchain.html#risc-v-devices) toolchain using the below commands.

```shell
rustup toolchain install nightly --component rust-src
rustup target add riscv32imac-unknown-none-elf
```

Since we also depend on C libs, you also need to have clang or similar installed.

If you are running nixos, you can have it using the below command
```shell
nix-shell -p clang
```
On other distros, you can get it via the package-manager.

Finally, you need a tool to flash the image. [espflash](https://github.com/esp-rs/espflash/tree/main/espflash/) can do it.
You can install [espflash](https://docs.espressif.com/projects/rust/book/getting-started/tooling/espflash.html) with the below command.

```shell
cargo install espflash --locked
```

[probe-rs](https://docs.espressif.com/projects/rust/book/getting-started/tooling/probe-rs.html) is another useful tool.

Once everything is ready, simple use cargo as shown below, to compile and run the program.
```shell
cargo run --release
```

## Useful Links
* https://docs.espressif.com/projects/rust/book/getting-started/index.html to setup the environment and tools.
* https://github.com/esp-rs/espup is also useful to install Espressif SoC toolchains
* https://github.com/danclive/esp-examples provides example programs using [esp-hal](https://github.com/esp-rs/esp-hal)
* https://github.com/esp-rs/awesome-esp-rust provides a curated list of useful resources