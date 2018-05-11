# thinbasic_rusty
Sample module template for ThinBASIC module in Rust to enable the community to create safe, high-performance extensions for our precious interpreter.

The main goal is to provide example showing how to reach for thinCore functions and some really minimal examples of module function implementation.

_The thinCore interface is work in progress and not yet complete._

## Let's get started
You will need Rust and thinBasic Core SDK to compile the module and ThinBASIC to call it.

### Installing Rust
* Download and install the [Build Tools for Visual Studio](https://www.visualstudio.com/cs/downloads/?q=Build+Tools+for+Visual+Studio).
* [Install Rust](https://www.rust-lang.org/en-US/install.html) via Rustup. **Please** customize the installation to ensure 32 bit pipeline currently needed by ThinBASIC:
  * Run the rustup-init.exe
  * Press *2* to alter the default settings
  * For *Default host triple?* enter *i686-pc-windows-msvc*
  * For *Default toolchain?* enter *stable*
  * For *Modify PATH variable?* enter *y*
  * Proceed with installation with *1*

### Installing thinBasic Core SDK
Retrieving thinBasic Core SDK is as simple as cloning the [thinBasic Core SDK](https://github.com/petrSchreiber/thinbasic_core_sdk).
Once the API stabilises a bit, it will be available as crate from crates.io.

### Installing ThinBASIC
A bit more _basic_ indeed. Just [download the installer](http://www.thinbasic.com/index.php/download/thinbasic-beta-1-10-4-0/) of the bleeding edge version.

### How to make the module?
* Checkout this repo and GOTO root directory (where this README is).
* Open console and run the `cargo build --release` command.
* `thinbasic_rusty.dll` will appear in `target/release`

### How to test the DLL?
* Copy the dll to _thinBasic/Mod_ directory
* Run the script _tests/test_module.tbasic_.

## IDE recommendations
I strongly recommend [Sublime Text 3](https://www.sublimetext.com/3) for Rust editing. It is very elegant. Just boost it with [rust-enhanced](https://github.com/rust-lang/rust-enhanced) plugin.

For ThinBASIC, grab ThinAIR, which is part of the default installation. Made with love.
