#!/bin/bash
# (C) 2025 Michail Krasnov <github.com/mskrasnov>
#
# AppImage generation script
#
# USAGE:
#   ./gen_appimage.sh [TIMEKEEPER VERSION]

NAME="TimeKeeper"
LOGO_URL="https://timekeepersoft.github.io/assets/logo.png"

APP_DIR="./target/${NAME}.AppDir"

# Create some dirs, copy some files...
mkdir -pv $APP_DIR

cp ./assets/AppRun $APP_DIR/
chmod +x $APP_DIR/AppRun

cp ./assets/TimeKeeper.desktop $APP_DIR

pushd /tmp
wget $LOGO_URL
popd

cp /tmp/logo.png $APP_DIR/TimeKeeper.png

# Generate AppImage
cd ./target
wget https://github.com/AppImage/AppImageKit/releases/download/13/appimagetool-x86_64.AppImage
chmod +x ./appimagetool-x86_64.AppImage
./appimagetool-x86_64.AppImage "${NAME}.AppDir" "$NAME-$1-x86_64.AppImage"
