# QR reader crate for PC

QR reader crate for PC is a utility to capture (via webcam) QR codes from Signer mobile app
and extracting data from it.  
It prints a string with decoded QR message in HEX format on display (and to file "decoded_output.txt").

## Getting Started

### Dependencies

The main requirement is the OpenCV. You can check this manuals: https://crates.io/crates/opencv and https://docs.opencv.org.

#### Arch Linux:

OpenCV package in Arch is suitable for this crate. It requires some dependencies.

* `pacman -S clang qt5-base opencv`

#### Other Linux systems:

* For Debian/Ubuntu also you need: `clang` and `libclang-dev`
* For Gentoo/Fedora also you need: `clang`
* It is preferable to build latest version of opencv+opencv_contrib from source. OpenCV package from the system repository may not contain the necessary libraries.\
Use this manual: https://docs.opencv.org/4.5.3/d7/d9f/tutorial_linux_install.html

### Executing program

* Run the program: `cargo run` + arguments
* Press any key to stop

#### Arguments

* `d` | `-d` | `--device` : set index of camera (from list of available cameras)
* `l` | `-l` | `--list` : get a list of available camera indexes
* `h` | `-h` | `--help` : refers to this manual

Camera resolution is hardcoded (640x480).

#### Examples

* `cargo run d 0` (camera index = 0)
* `cargo run l`


