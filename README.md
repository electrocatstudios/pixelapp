# Pixel App
An application for making small pixel art animations, by editing and clone each frame.

## Introduction
This is an open-source pixel drawing application. Originally it was conceived for a specific
purpose but has grown into a larger application. It is designed to run as a web-server but
there is nothing stopping you from running this locally and access through a browswer. It
requires a database to store the data, which will allow persistence. At some point I hope to
make a version available online but this is quite a while away (as of time of writing).

## History
This application was originally written in Go but this is a rewrite in Rust. The original
code was taken from a larger project so may not compile on it's own. 

## Running the application
To run the application use
```bash
cargo run -- --debug --port=5000
```
Note: You can leave the port off, the default is `8888`

## Roadmap
The current roadmap of items I'll be working on.

[] Rewrite original as working application
[] Convert PNG to pixellated version

