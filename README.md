# `Storyboard68`
> Improvement of [68keys.io](https://68keys.io).
> Hotswap compatiable, *blazingly fast*™, less height, inexpensive.

Keyboard firmware for storyboard68 (was previously using [qmk fork](https://github.com/storycraft/qmk_firmware/tree/storyboard68/keyboards/storyboard68))

See `hardware` directory for PCB and case design.

## Requirements
AVR GCC Toolchain, dfu-programmer

## Building and flashing
Run `cargo xtask build` to build firmware as elf and hex (can be found at `target/atmega32u4/release`)

Run `cargo xtask deploy` to build and flash firmware into device directly

## License
The project is licensed under MIT or Apache-2.0