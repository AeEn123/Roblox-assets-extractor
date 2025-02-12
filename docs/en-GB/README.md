[![Download for Windows](https://img.shields.io/github/downloads/AeEn123/Roblox-assets-extractor/latest/Roblox-assets-extractor-windows.exe?label=Download&color=blue&style=for-the-badge)](https://github.com/AeEn123/Roblox-assets-extractor/releases/latest/download/Roblox-assets-extractor-windows.exe)
[![Download for Linux](https://img.shields.io/github/downloads/AeEn123/Roblox-assets-extractor/latest/Roblox-assets-extractor-linux?label=Download&style=for-the-badge)](https://github.com/AeEn123/Roblox-assets-extractor/releases/latest/download/Roblox-assets-extractor-linux)
[![Website](https://img.shields.io/badge/Website-red?logo=googlechrome&style=for-the-badge)](https://aeen123.github.io/Roblox-assets-extractor/)

[![Build and Release](https://github.com/AeEn123/Roblox-assets-extractor/actions/workflows/build-and-release.yml/badge.svg)](https://github.com/AeEn123/Roblox-assets-extractor/actions/workflows/build-and-release.yml)
[![Discord invite](https://img.shields.io/discord/470242481582243860?label=Discord)](https://discord.gg/xqNA5jt6DN)
![Total downloads](https://img.shields.io/github/downloads/AeEn123/Roblox-assets-extractor/total?label=Total%20Downloads)


# Roblox Assets Extractor
This tool extracts cached data from your Roblox installation by looking at the headers of cached files.

![Screenshot](/assets/screenshot.png)

# FAQ
See [FAQ.md](/docs/en-GB/FAQ.MD)

# Usage
## Tabs
You can see multiple tabs. Roblox Assets Extractor catagorises the files into multiple catagories. You can filter them by clicking on the tab.
## The toolbar
Each item in the toolbar allows you to do different operations with the directory or the asset, you can also access the toolbar as a context menu by right clicking. You can disable the toolbar at the top of the screen in the settings, **Enable toolbar** under the **Behaviour** section.
## Keyboard navigation and shortcuts
The program is designed to be easy to use with a mouse but also allow for keyboard navigation and shortcuts for more advanced users, the shortcuts are shown on the buttons to show how you can access them quickly.<br>

You can cycle through the tabs with Alt (or ctrl) + 1-8 allowing you to navigate between tabs only using the keyboard, you can select assets with tab and confirm with enter.
## Settings menu
In the settings menu you will find general customization options as well as actions to do with your roblox cache. Here you can extract all of your roblox cache, change the directory or clear the cache.

# CLI mode
CLI is work-in-progress.
See [CLI.md](/docs/en-GB/CLI.md)

# Installing for Windows
The program only comes portable on Windows for now, this may change in the future

# Installing for Linux
Installing for Linux varies for each distro. We hope we can eventually create a flatpak for universal installation.
We would greatly welcome a pull request for a flatpak.
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
The development builds can be downloaded from the [releases](https://github.com/AeEn123/Roblox-assets-extractor/releases) page.

If you already have the latest development build of Roblox Assets Extractor installed, you can enable development builds in settings 
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
