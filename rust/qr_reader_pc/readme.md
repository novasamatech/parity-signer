# QR reader crate for PC

QR reader crate for PC is a utility to scan (via webcam) QR codes from Signer mobile app
and extracting data from it.  
Prints on display (and to file "decoded_output.txt") a string with decoded QR message in HEX format.

## Getting Started

### Dependencies

#### Arch Linux:

* `pacman -S clang`

#### Other Linux systems:

* You may need to install these dependencies first: `sudo apt install clang libxkbcommon-dev libwayland-cursor0 libwayland-dev`

### Executing program

* Run the program: `cargo run` + arguments

To check available camera parameters: `v4l2-ctl --list-formats-ext`

### Arguments

* `d`|`-d`|`--device` : set index of camera (from list of available cameras)
* `l` | `-l` | `--list` : get a list of available camera indexes
* `ff`|`-ff` : set frame format for camera, `YUYV` and `MJPEG` are suported
* `fps`|`-fps` : set fps parameter for camera

You can only provide camera index. Other parameters will be setted by default (YUYV frame format and 30 fps).
Camera resolution is hardcoded (640x480).

### Examples

* `cargo run d 0` : camera index = 0,
* `cargo run d 2 ff MJPEG fps 30`: camera index = 2, frame format = MJPEG, fps = 30,



