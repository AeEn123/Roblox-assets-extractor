> [!CAUTION]
> # The rust rewrite is unfinished, please use the python version for now

> [!NOTE]
> Roblox Assets Extractor has been rewritten in Rust with egui. The python version is kept for updating to the rust version.

> [!NOTE]
> If you are using version 0.4 or earlier, autoupdate functionality will be broken. In such cases, please download from here instead.

# Roblox Assets Extractor
This tool extracts cached data from your Roblox installation by looking at the headers of cached files.

# FAQ
### Does this interfere with the roblox client?
No, it is opening files that your client has already created. You can see them yourself in %Temp%\Roblox

### Is this malware?
No, this is not malware, similar to other Free and Open Source Software, the code is available for everyone to see. It would be impossible for me to hide malware in here, if you are still not confident, you can always check the source code and [build it yourself!](https://github.com/AeEn123/Roblox-assets-extractor/tree/rust-rewrite?tab=readme-ov-file#building-from-source)

### Windows says "Windows protected your PC" What do I do?
If Windows detects a program from an unverified publisher, this popup will appear. If this popup does appear, click "More info" and click "Run anyway".

### Can this get me banned?
No, unlike cheats, this **does not** inject into roblox. Making this an anti-cheat friendly way of extracting assets. I have been using this myself for years and I have not got any account warnings because of it.

### My extracted assets don’t play in my media player, what can I do?
Some media players may not support the format that the file is in. If that is the case, please try another media player that supports all of the formats this supports, e.g VLC. **If the file is really broken, please [create an issue.](https://github.com/AeEn123/Roblox-assets-extractor/issues)**

### Can I switch to the light/dark theme?
Yes, the theme of the program automatically syncs to your system theme.

### Why is KTX files in a different tab? Shouldn't it be in the Textures tab?
Technically it should, but most image viewers don't support KTX files, so it is best to move this aside to a different tab to avoid compatability issues, this tab should be used for more advanced users.

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
See [CLI.md](./docs/CLI.md)

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
git clone -b rust-rewrite https://github.com/AeEn123/Roblox-assets-extractor
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
I am quite new to Tkinter, so the UI looks pretty bad messy.

> [!TIP]
> If file listing becomes too slow, try clearing your Roblox cache. You can do this by going to your cache folder (%Temp%\Roblox) and deleting all the files.