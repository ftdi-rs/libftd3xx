[![crates.io](https://img.shields.io/crates/v/libftd3xx.svg)](https://crates.io/crates/libftd3xx)
[![docs.rs](https://docs.rs/libftd3xx/badge.svg)](https://docs.rs/libftd3xx/)
[![CI](https://github.com/ftdi-rs/libftd3xx/workflows/CI/badge.svg)](https://github.com/ftdi-rs/libftd3xx/actions)

# libftd3xx

Rust safe wrapper for the [FTDI D3XX drivers].

This takes the [libftd3xx-ffi] C bindings crate and extends it with rust
safe wrappers.

## Usage
Simply add this crate as a dependency in your `Cargo.toml`.

```toml
[dependencies.libftd3xx]
version = "0.0.1"
# statically link the vendor library, defaults to dynamic if not set
# this will make things "just work" on Linux and Windows
features = ["static"]
```

This is a basic example to get your started.
Check the source code or documentation for more examples.
```rust
use libftd3xx::{Ftdi, FtdiCommon};

let mut ft = Ftdi::new()?;
let info = ft.device_info()?;
println!("Device information: {:?}", info);
```

This crate is just a wrapper around the FTD3XX driver; I2C, SPI, and GPIO
examples using the [`embedded-hal`] traits can be found in
[`ftdi-embedded-hal`].

### udev rules
To access the FTDI USB device as a regular user on Linux you need to update
the [udev] rules.

Create a file called `/etc/udev/rules.d/99-ftdi.rules` with:
```
SUBSYSTEM=="usb", ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6001", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6010", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6011", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6014", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6015", MODE="0666"
```

Then, reload the rules:
```bash
sudo udevadm control --reload-rules
sudo udevadm trigger
```

### Linking

By default this crate with use dynamic linking for the vendor library.
Use the `static` feature flag to enable static linking.

#### Dynamic Linking on Linux

The shared object `libftd3xx.so` must exist on your system.
See [FTDI Drivers Installation Guide for Linux] for instructions.

#### Dynamic Linking on Windows

The FTD3XX DLL must exist on your system PATH.
The easiest way to install this is with the vendor provided [setup executable].

#### Static Linking on Linux or Windows

No special considerations are needed, the static library is distributed with
permission from FTDI in the [libftd3xx-ffi] crate.

## References

* [D3XX Programmers Guide V1.4]
* [D3XX Drivers Download Page]

## Troubleshooting
### Unknown Device on Linux
Remove the VCP FTDI driver.
```bash
sudo rmmod ftdi_sio
sudo rmmod usbserial
```
See [FTDI Drivers Installation Guide for Linux] for more details.

[D3XX Drivers Download Page]: https://www.ftdichip.com/Drivers/D3XX.htm
[D3XX Programmers Guide V1.4]: https://ftdichip.com/document/programming-guides/
[FTDI D3XX drivers]: https://www.ftdichip.com/Drivers/D3XX.htm
[FTDI Drivers Installation Guide for Linux]: http://www.ftdichip.cn/Support/Documents/AppNotes/AN_220_FTDI_Drivers_Installation_Guide_for_Linux.pdf
[libftd3xx-ffi]: https://github.com/ftdi-rs/libftd3xx-ffi
[udev]: https://en.wikipedia.org/wiki/Udev
[`ftdi-embedded-hal`]: https://github.com/ftdi-rs/ftdi-embedded-hal
[`embedded-hal`]: https://crates.io/crates/embedded-hal
