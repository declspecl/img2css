# img2css

A command line client for creating pure CSS representations of image files written in rust

## How to download

You can either download a [release](https://github.com/declspecl/img2css/releases) or build from source. To build from source, see below:

Run `git clone https://github.com/declspecl/img2css`, navigate into the /img2css directory, and run `cargo build --release`. From there, an executable will be made in /target/release that you can execute.

## How to use

img2css has 2 modes: an interactive client or a CLI tool

Examples on how to use both can be seen below:
* `.\img2css.exe` opens the command line client where you will be asked the path and if it should be standalone through stdin.
* `.\img2css.exe /some/random/image.jpg` opens the image you passed as an argument and writes the result into .\img2css.css
* `.\img2css.exe /some/random/image.jpg -s` opens the image you passed as an argument and writes the result into a standalone .\img2css.html
