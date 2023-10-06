#!/usr/bin/env bash
set -exo pipefail
# Builds windows and linux thetawave executables in the following directory structions:
#
# steam_out
#├── linux
#│  └── thetawave
#└── windows
#   └── thetawave.exe
# The only positional argument is the out directory


function build_thetawave_executable_to_directory(){
        # Put executable for rustc target $1 in $2/$3
        # $1 = rustc target
        # $2 = output directory (will be created if it doesn't exist)
        # $3 = executable name (thetawave or thetawave.exe)
        rustup target add "$1"
        cargo build --release --target "$1" --features storage --features cli
        mkdir -p "./$2/"
        cp -R "./target/$1/release/$3" "./$2/$3"

}
function buildThetawaveForSteamMain() {
        local outDir="${1:steam_out}"
        build_thetawave_executable_to_directory "x86_64-unknown-linux-gnu" "$outDir/linux/" thetawave
        build_thetawave_executable_to_directory "x86_64-pc-windows-gnu" "$outDir/windows/" thetawave.exe
}
buildThetawaveForSteamMain "$@"
