#!/usr/bin/env bash
set -exo pipefail
# Builds windows and linux thetawave executables in the following directory structions:
#
# steam_out
#├── linux
#│  └── thetawave
#└── windows
#   └── thetawave.exe
# The only positional argument is the out directory. This script _always_ expects to be run on linux. It will
# cross-compile for windows as needed.
#
# This MUST be run with the git/game repo root as the current working directory to find all of the config files.


function build_thetawave_executable_to_directory(){
        # Put executable for rustc target $1 in $2/$3
        # $1 = rustc target
        # $2 = output directory (will be created if it doesn't exist)
        # $3 = executable name (thetawave or thetawave.exe)
        # $4 cargo build command. Either `build` or `zigbuild`
        local baseTargetName
        local cargoBuildCommand="${4:-build}"
        # cargo-zigbuild allows adding a suffix with the glib version to the rustc target, but that is removed for most
        # purposes. Removes everything after the first '.', preserving the rustc target if it is "nomal"/directly a
        # rustc target.
        baseTargetName=$(echo "$1" | cut -d '.' -f 1)
        rustup target add "$baseTargetName"
        cargo "$cargoBuildCommand" --release --target "$1" --features storage --features cli --features arcade
        mkdir -p "./$2/"
        cp -R "./target/$baseTargetName/release/$3" "./$2/$3"

}
function buildThetawaveForSteamMain() {
        local twRepoRoot # like ../ if run from the script location. git/game repo root
        twRepoRoot="$(dirname "$(dirname realpath "$0")")"
        local outDir="${1:-$twRepoRoot/steam_out}"
        echo "Building thetawave game at path $outDir"
        # Currently we only use zigbuild to link with older glibc versions. If cargo gets a nice way to do that, we can
        # drop this dependency. An alternative could be to compile in a VM/container with an older glibc version
        # See https://kobzol.github.io/rust/ci/2021/05/07/building-rust-binaries-in-ci-that-work-with-older-glibc.html#solution)
        # and https://stackoverflow.com/questions/57749127/how-can-i-specify-the-glibc-version-in-cargo-build-for-rust .
        cargo install cargo-zigbuild
        # We _need_ to link against a lower version of glibc for steamdeck. Lower this number to increase compatibility
        # with linux distros that do not update glibc frequently. Lowering the glibc version (strictly?) increases
        # compatibility while increasing it _might_ improve performance and security (citation needed on the last part)
        build_thetawave_executable_to_directory "x86_64-unknown-linux-gnu.2.34" "$outDir/linux/" thetawave "zigbuild"
        build_thetawave_executable_to_directory "x86_64-pc-windows-gnu" "$outDir/windows/" thetawave.exe
}
buildThetawaveForSteamMain "$@"
