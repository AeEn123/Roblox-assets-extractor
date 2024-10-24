> [!CAUTION]
> # The rust rewrite is unfinished, please use the python version for now

> [!NOTE]
> Roblox Assets Extractor has been rewritten in Rust with egui. The python version is kept for updating to the rust version.

> [!NOTE]
> If you are using version 0.4 or earlier, autoupdate functionality will be broken. In such cases, please download from here instead.

# Roblox Assets Extractor
This tool is designed to extract cached data from your Roblox installation by inspecting the headers of cached files.

## Usage
You can extract Roblox music, sounds, and textures from your Roblox cache (ensure your client is open for music).

Use the "Delete All" button to clear the currently selected cache folder. This is useful for extracting assets from a specific game.

## More Info
This is my first project written in rust/egui so bugs may appear, in the circumstance that a bug does appear, report an issue and use the legacy python version if the bug makes it unusable.

> [!IMPORTANT]
> This tool is designed for Windows and Linux and may not work on other operating systems.

# Building from source

Building from source requires cargo, [which can be installed from rustup.](https://rustup.rs/)

## 1. Clone the repository
```bash
git clone https://github.com/AeEn123/Roblox-assets-extractor
cd Roblox-assets-extractor
```
## 2. Build with cargo
```bash
cargo build --release
```
Wait for it to build all the dependencies and the application. After that you should find it in the `target` folder.

# Python version
Info below here is about the python version, and would not apply if you are using the rust version.



## Requirements
- **Tkinter:** Tkinter is required to run this tool. On Windows, ensure that Tkinter is selected during the Python installation, or you can install it via pip. On Linux, you can install it using your package manager or download it via pip.
  To install it via pip:
  ```bash
  pip install tkinter
  ```
- *(Optional)* **Pip:** Pip can be used for auto-downloading modules. On Windows, ensure that the corresponding option is checked during the Python installation. On Linux, you can install it using your package manager.
- *(Optional)* **Requests:** This will be downloaded automatically via pip (with your consent) for the ability to check for updates. If automatic download fails, you can install it through pip:
  ```bash
  pip install requests
  ```

![Screenshot of the python version](https://img.guildedcdn.com/ContentMediaGenericFiles/d64200649953156687eb159ea5efcb25-Full.webp?w=1920&h=1040)

## More Info
I am relatively new to Tkinter, so please be aware that the UI may appear a bit messy.

> [!TIP]
> If file listing becomes too slow, consider clearing your Roblox cache. You can do this by navigating to your cache folder (%Temp%\Roblox) and deleting all the files.

*I admit. This readme is AI generated. I am really bad at writing up documents. Sorry.*
