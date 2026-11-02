#!/bin/bash
# Script to move the dotfiles to the config folders and build the suckless
# programs, without installing any packages.
set -e

REPO_DIR="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
. "$REPO_DIR/common.sh"

echo "Moving the penrose dotfiles to your home directory"
install_dotfiles
build_suckless
build_penrose

report_backup
echo "Installed the penrose rice successfully"
echo "Log out (sysact: mod + Backspace, leave penrose) and log in again to use the new build"
