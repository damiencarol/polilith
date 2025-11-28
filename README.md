# Polilith

Polilith is a SAST tool to detect some common misconfiguration of Docker images

## Installation

## Command Line Options

```
$ polilith -h
polilith 0.1.0
Damien Carol <damien.carol@gmail.com>
Docker image quality tool

USAGE:
    polilith [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>     Docker image file
    -o, --out <output>    Report file
```

## Getting started

### Scan an image

1. get the image as an archive.
```bash
docker pull "python:3.6"
docker save "python:3.6" -o python_3.6.tar
```
2. scan it
```bash
cargo run -- -f python_3.6.tar -o report-python_3.6.tar.sarif
```
