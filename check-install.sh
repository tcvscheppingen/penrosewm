#!/bin/sh
# Checks why penrose doesn't start and prints one line per check, short
# enough to read off the screen of a TTY. Run it as the user who logs in.

ok() { echo "ok       $1"; }
problem() { echo "PROBLEM  $1"; }

# Edition 2024 in Cargo.toml needs Rust 1.85 or newer
rust="$(rustc --version 2> /dev/null | awk '{ print $2 }')"
if [ -z "$rust" ]; then
    problem "rustc is not installed"
elif [ "$(printf '1.85.0\n%s\n' "$rust" | sort -V | head -n 1)" != 1.85.0 ]; then
    problem "rustc $rust is too old, 1.85 or newer builds penrose"
else
    ok "rustc $rust"
fi

if [ -x /usr/local/bin/penrose ]; then
    ok "/usr/local/bin/penrose is installed"
else
    problem "no /usr/local/bin/penrose: run ./move-config-files.sh and read its last lines"
fi

for prog in startx xmodmap st dmenu dwmblocks; do
    if command -v "$prog" > /dev/null; then
        ok "$prog found"
    else
        problem "$prog not found"
    fi
done

if [ -e "$HOME/.config/x11/xinitrc" ]; then
    ok "~/.config/x11/xinitrc exists"
else
    problem "no ~/.config/x11/xinitrc: run ./move-config-files.sh"
fi

# The TTY1 login snippet goes into the profile of the login shell
if grep -qs "exec startx" "$HOME/.bash_profile" "$HOME/.bash_login" "$HOME/.profile" "${ZDOTDIR:-$HOME}/.zprofile"; then
    ok "login profile starts X on TTY1"
else
    echo "info     no startx in your login profile: start X with startx ~/.config/x11/xinitrc"
fi

if fc-list | grep -q "JetBrainsMono Nerd Font"; then
    ok "JetBrainsMono Nerd Font installed"
else
    problem "JetBrainsMono Nerd Font missing: the bar can't start without it"
fi

# The last run of penrose and of X, if there was one
log="$HOME/.local/share/penrose.log"
if [ -e "$log" ]; then
    error="$(grep -iE "error|panic|unknown" "$log" | sed 's/\x1b\[[0-9;]*m//g' | tail -n 1 | cut -c1-150)"
    if [ -n "$error" ]; then
        problem "penrose.log: $error"
    else
        ok "penrose.log has no errors (last line: $(tail -n 1 "$log" | sed 's/\x1b\[[0-9;]*m//g' | cut -c1-80))"
    fi
else
    echo "info     no penrose.log: the session never reached penrose"
fi

xlog="$HOME/.local/share/xorg/Xorg.0.log"
if [ -e "$xlog" ]; then
    error="$(grep "(EE)" "$xlog" | tail -n 1 | cut -c1-150)"
    if [ -n "$error" ]; then
        problem "Xorg.0.log: $error"
    else
        ok "Xorg.0.log has no errors"
    fi
else
    echo "info     no Xorg.0.log in ~/.local/share/xorg"
fi
