# Releasing Thetawave to Steam

We currently do not use Github actions/CD to publish Thetawave to Steam because Steam wants to use one time passwords in
a bit of an awkward way. So one of our local machines will act as the build server. All of these commands should be run
from the root of the repository.

```bash
$ ls steam_out/
assets  linux  windows
```

Generally the goal is to get a single directory with all build artifacts for all platforms, and use a
[Dockerized](https://www.docker.com/) steampipe image to automate that deployment as much as possible. That Docker
command is interactive and will require entering the password and a MFA one time code.

0. Build the game on all platforms
```bash
./scripts/build_thetawave_for_steam.sh steam_out/
```

1. Download game audio and image assets.

See the [Assets section of the CONTRIBUTING.md](./CONTRIBUTING.md) for information about this CLI, or run
`python3 ./asset_manager.py --help`

```bash
python3 ./asset_manager.py --profile <OPTIONAL_AWS_PROFILE_NAME> --no-dryrun download
cp -R assets steam_out/
```

2. Upload artifacts to steam.
This will: update the steam sdk, authenticate our special "builder" account (permissioned only to push builds for the
Thetawave game), and run a script that will package all assets to their respective depots to upload to Steam. This
script is tailored to our specific app and depot configuration, so changing those on Steamworks will likely require also
changing the deployment script. The [vdf is mostly just a data file](./steam_build/thetawave_build.vdf) invoked by the
[sdk](https://partner.steamgames.com/doc/sdk/uploading)

```bash
docker run --net=host \
    -e STEAMUSER="thetawave_builder" \
    -e VDFAPPBUILD="thetawave_build.vdf" \
    -e STEAMAPPBUILDESC="Automated CD Upload" \
    -v "$(pwd)/steam_out:/home/steam/steamsdk/sdk/tools/ContentBuilder/content" \
    -v "$(pwd)/steam_build:/home/steam/steamsdk/sdk/tools/ContentBuilder/scripts" \
    -it \
    --rm "cm2network/steampipe:contentbuilder"
```

3. There will be a new build on [the Steamworks website](https://partner.steamgames.com/apps/builds/2427510). Set branch
   of that build to deploy it. Ideally deploy and test the beta branch before releasing on the default branch.
