# penrose-config

My window manager, built with the [Penrose](https://github.com/sminez/penrose) library, plus an auto rice script for the rest of the desktop: st, dmenu, slock, dwmblocks and dunst in [gruvbox](https://github.com/morhetz/gruvbox) dark, the same colors as the penrose bar. Based on [DARBS](https://github.com/tcvscheppingen/DARBS).

## Install

For Arch and Arch based distributions, Fedora, and Artix with dinit. **Read a script before you run it.** Run it as your own user, not as root; it uses `sudo` where needed.

```bash
git clone <this repo>
cd penrose-config
./auto-rice.sh        # on Artix: ./auto-rice-artix.sh
```

Then reboot. The script:
- installs Xorg, Rust, xmodmap, PipeWire, NetworkManager, Bluetooth, fonts, Neovim, Thunar and LibreWolf, plus the Intel or AMD GPU drivers it finds (not NVIDIA)
- backs up the configs it replaces to `~/.config/penrose-backup-<date>/`
- copies the dotfiles, builds st, dmenu, slock and dwmblocks and installs them to `/usr/local`
- copies this config to `~/.local/src/penrose`, builds it and installs it as `/usr/local/bin/penrose`
- asks how to start penrose: on login on TTY1, or from a minimal LightDM login screen (`RICE_LOGIN=tty|dm|none` answers in advance)

Already have the packages? `./move-config-files.sh` only copies the dotfiles and builds penrose and the suckless programs.

Penrose doesn't start? Run `./check-install.sh`. It prints one line per check, `ok` or `PROBLEM` with the cause.

## Key bindings

`M` is the Super key.

| Keys | Action |
|---|---|
| `M + Return` / `M + d` | Terminal (st) / run a program (dmenu) |
| `M + shift + q` | Close the window |
| `M + j/k` / `M + shift + j/k` | Focus / move the window down or up the stack |
| `M + Up/Down` | More or fewer main windows |
| `M + Left/Right` | Main area smaller or larger |
| `M + grave` / `M + shift + grave` | Next / previous layout |
| `M + 1…9` / `M + shift + 1…9` | Go to a workspace / move the window to it |
| `M + Tab` | Previous workspace |
| `M + bracketleft/bracketright` | Focus the previous / next display |
| `M + Backspace` | System menu: lock, leave penrose, sleep, reboot, shutdown (`sysact`) |
| `M + F3` | Display arrangement (`displayselect`) |
| `M + shift + s` | Write the window manager state to the log |
| `M + alt + Escape` | Exit penrose |

`maimpick` (screenshots) and `dmenurecord` (recordings) are installed too, without a key: run them from dmenu, or bind them in `src/main.rs`.

## Good to know

- **Status bar:** the penrose bar shows the workspaces, the layout, the window title and, on the right, the dwmblocks text (network, recording, volume, battery, memory, clock) without icons. There is no system tray, and clicking a block does nothing: that needs dwm's statuscmd patch.
- **Log:** penrose writes to `~/.local/share/penrose.log`. Look there when it doesn't start.
- **Changing penrose:** edit `~/.local/src/penrose/src/main.rs`, then run `cargo build --release` there and `sudo install -m755 target/release/penrose_config /usr/local/bin/penrose`. Leave penrose with `sysact` and log in again to use the new build.
- **Key bindings:** penrose looks up key names with `xmodmap -pke`. Run `cargo test` in a terminal inside X to check them; outside X every key name is unknown.
- **Changing st, dmenu, slock or dwmblocks:** edit `config.h` in `~/.local/src/<program>`, then `make && sudo make install` there.
- **No sound?** If the volume shows `vol dummy` on an Intel laptop, run `sound-fix`.
- **Locking:** after 5 minutes idle, and before sleep. The displays turn off after 10 minutes. The times are the `xset` lines in `~/.config/x11/xinitrc`.
- **Wallpaper:** `setbg image.jpg` changes it, and `setbg folder` picks a random image from a folder.

## Palette

[gruvbox](https://github.com/morhetz/gruvbox) dark:

| | Color | |
|---|---|---|
| Background | `#282828` | Bar, terminal, menus |
| Border | `#3c3836` | Unfocused windows |
| Text | `#ebdbb2` | |
| Accent | `#458588` | Focused workspace, dmenu, notifications |
| Red | `#cc241d` / `#fb4934` | Wrong password / critical notifications |

The colors are in `src/main.rs`, in each program's `config.h` in `~/.local/src`, in `~/.config/dunst/dunstrc` and in `~/.config/nvim/colors/gruvbox.lua`. Helix uses its built-in `gruvbox` theme.

## Credits
- [Penrose](https://github.com/sminez/penrose) by sminez
- [DARBS](https://github.com/tcvscheppingen/DARBS), which this rice is based on
- [suckless.org](https://suckless.org) for st, dmenu and slock, and Luke Smith's [st](https://github.com/LukeSmithxyz/st), [dwmblocks](https://github.com/LukeSmithxyz/dwmblocks) and [voidrice](https://github.com/LukeSmithxyz/voidrice) scripts
- [gruvbox](https://github.com/morhetz/gruvbox) by Pavel Pertsev
- Wallpaper: ["Cosmic Cliffs" in the Carina Nebula](https://science.nasa.gov/asset/webb/cosmic-cliffs-in-the-carina-nebula-nircam-and-miri-composite-image/) by NASA, ESA, CSA and STScI (James Webb Space Telescope, public domain)
