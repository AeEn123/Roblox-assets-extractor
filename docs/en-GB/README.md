[![Download for Windows](https://img.shields.io/github/downloads/AeEn123/Roblox-assets-extractor/latest/Roblox-assets-extractor-windows.exe?label=Download&color=blue&style=for-the-badge)](https://github.com/AeEn123/Roblox-assets-extractor/releases/latest/download/Roblox-assets-extractor-windows.exe)
[![Download for Linux](https://img.shields.io/github/downloads/AeEn123/Roblox-assets-extractor/latest/Roblox-assets-extractor-linux?label=Download&style=for-the-badge)](https://github.com/AeEn123/Roblox-assets-extractor/releases/latest/download/Roblox-assets-extractor-linux)
[![Website](https://img.shields.io/badge/Website-red?logo=googlechrome&style=for-the-badge)](https://aeen123.github.io/Roblox-assets-extractor/)

[![Windows Build](https://github.com/AeEn123/Roblox-assets-extractor/actions/workflows/build_win.yml/badge.svg)](https://github.com/AeEn123/Roblox-assets-extractor/actions/workflows/build_win.yml)
[![Linux Build](https://github.com/AeEn123/Roblox-assets-extractor/actions/workflows/build_linux.yml/badge.svg)](https://github.com/AeEn123/Roblox-assets-extractor/actions/workflows/build_linux.yml)
[![Discord invite](https://img.shields.io/discord/470242481582243860?label=Discord)](https://discord.gg/xqNA5jt6DN)

# Roblox Assets Extractor
This tool extracts cached data from your Roblox installation by looking at the headers of cached files.

![Screenshot](/assets/screenshot.png)

# FAQ
See [FAQ.md](/docs/en-GB/FAQ.MD)

# Usage
## Tabs
You can see multiple tabs. Roblox Assets Extractor catagorises the files into multiple catagories. You can filter them by clicking on the tab.
## Delete this directory
If you click delete this directory, it will delete the files within the directory where the tab is shown. This makes easy to extract assets from specific games, by clicking this button before joining into the game you want to extract from.
## Extract all assets
There is also an extract all assets of this type button, you can use this button to extract all the assets of that type and put that into a folder of your liking.<br>
## Settings menu
There are similar buttons in the settings menu, where you can delete your cache or extract all assets, where it will automatically create folders within a folder you choose.

# CLI mode
CLI is work-in-progress.
See [CLI.md](/docs/en-GB/CLI.md)

# Installing for Windows
The program only comes portable on Windows for now, this may change in the future

# Installing for Linux
Installing for Linux varies for each distro. We hope we can eventually create a flatpak for universal installation.
## Arch Linux
You can install on Arch Linux by using the PKGBUILD located in `packages/arch`
An example installation script:
```bash
mkdir /tmp/Roblox-assets-extractor
cd /tmp/Roblox-assets-extractor
wget raw.githubusercontent.com/AeEn123/Roblox-assets-extractor/refs/heads/main/packages/arch/PKGBUILD
makepkg -si
```
## Other distros
Other distros will hopefully be supported soon. If you know how to make one and want it merged in this project, create a pull request!

# Testing development builds
The development builds can be downloaded below

[![Development build | Windows](https://img.shields.io/badge/Development_build-Windows-blue)](https://nightly.link/AeEn123/Roblox-assets-extractor/workflows/build_win/main/artifact.zip)
[![Development build | Linux](https://img.shields.io/badge/Development_build-Linux-yellow)](https://nightly.link/AeEn123/Roblox-assets-extractor/workflows/build_linux/main/artifact.zip)

# More Info
This is my first project written in rust/egui so bugs may appear, in the circumstance that a bug does appear, report an issue and use the legacy python version if the bug makes it unusable.

> [!IMPORTANT]
> This tool is designed for Windows and GNU/Linux and may not work on other operating systems.

> [!TIP]
> If file listing is too slow, you can clear your cache with the clear cache button in the settings. Also, turning off Windows Defender will speed up file listing, as it scans every time a file is opened.

# Building from source

Building from source requires cargo, [which can be installed from rustup.](https://rustup.rs/)

## 1. Clone the repository
```bash
git clone https://github.com/AeEn123/Roblox-assets-extractor
cd Roblox-assets-extractor
```
## 2. Build with cargo, the command you run depends on your use-case
If you want a finished build which runs fast but compiles slowly (recommended for normal use)
```bash
cargo build --release
```

If you want a development build which runs slowly but compiles fast (recommended for development)
```bash
cargo build
```
Wait for it to build all the dependencies and the application. After that you should find it in the `target` folder.

# Python version
See [python.md](/docs/en-GB/python.md)