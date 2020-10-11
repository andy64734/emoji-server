# emoji-server
The implementation of an emoji server, written by andy64734 and LanHikari22. You are welcome to use and distribute the contents of this repository as much as you like as long as you don't strip our credit from this project.

## Setup
To install the emoji server, you can clone this repository or download a ZIP of the repository. This server has been tested with the Apache web server, and has been only tested on a few Linux distros.

To install the client side files, copy the html folder into wherever Apache would look for (I suggest /var/www/html for Linux users).

## CGI Script Building
In the html/cgi-bin folder is a compiled binary for the CGI server-side script that runs searches in the emoji server. This binary is pre-compiled for x86-64 Linux. If you do not have an x86-64 Linux installation, you can rebuild the binary for your architecture. To do this, you will need to download the rustc compiler and Cargo to build Rust projects.

To rebuild the binary, in your terminal program, go to the src/search folder and run:
```
cargo build --release
```

Then, copy the generated file src/search/release/search to your wherever you copied the project's html/cgi-bin folder (probably /var/www/html/cgi-bin).

## Adding New Emojis
You can add new emojis to the server by adding them to wherever you copied the project's html/emojis folder (probably /var/www/html/emojis). Keep in mind that the emoji server expects a certain layout for its emojis.

Non-GIF emojis are stored at the location:
```
[root-of-server]/emojis/[show-name]/[png/jpg emoji-name]
```
For example:
```
/var/www/html/emojis/pokemon/pikachu_derpy_0.png
```

GIF emojis are stored at the location:
```
[root-of-server]/emojis/[show-name]/gifs/[png/jpg emoji-name]
```
For example:
```
/var/www/html/emojis/pokemon/gifs/pikachu_dancing_0.png
```
