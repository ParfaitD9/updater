# UPDATER

## Description

**UPDATER** is a command line program written in Rust to help you directly download the [driver](https://www.w3.org/TR/webdriver/)(s) compatible with your browser. For programmers writing test automation and web scraping scripts, it can quickly become tedious to have to download the new driver depending on the update of your browser. This is where Updater comes in by automatically downloading the compatible driver. For the moment, only [chromedriver](https://chromedriver.chromium.org/downloads) download is supported and identification is automatic for browsers installed from [pacman](https://wiki.archlinux.org/title/pacman).

## Installation

Installation is so simple that it just involves downloading the release file and saving it to one of the folders in your PATH environment variable. You can also decide to compile from source, in which case you will need a [Rust](https://www.rust-lang.org/) compiler installed.

## Usage

From your shell just run:
```bash
updater -o ~
```
This will download the chromedriver compatible with your current browser version. You can always use help menu via :
```bash
updater --help
```

## License
This project is under [MIT License](https://choosealicense.com/licenses/mit/)
