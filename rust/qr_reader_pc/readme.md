# QR reader crate for PC

QR reader crate for PC is a utility to capture (via webcam) QR codes from Signer mobile app
and extracting data from it.  
It prints a string with decoded QR message in HEX format on display (and to file "decoded_output.txt").

## Getting Started

### Dependencies

You need to install `OpenCV`. You can check this manual: https://crates.io/crates/opencv and https://docs.opencv.org

#### Linux

* For Debian/Ubuntu you need: `clang` and `libclang-dev`
* For Gentoo/Fedora you need: `clang`
* For Linux system, it is preferable to build latest version of opencv+opencv_contrib from source. OpenCV package from the system repository may not contain the necessary libraries. Use this manual: https://docs.opencv.org/4.5.3/d7/d9f/tutorial_linux_install.html

### Executing program

* Run the program: `cargo run` + arguments

#### Arguments

* When you run program without arguments, program provide a list of available cameras for v4l backend,
* `d` | `-d` | `--device` : set index of camera (from list of available cameras),
* `l` | `-l` | `--list` : get a list of available camera indexes,
* `h` | `-h` | `--help` : refers to this manual,

Camera resolution is hardcoded (640x480).

#### Examples

* `cargo run d 0` : camera index = 0,
* `cargo run l`


