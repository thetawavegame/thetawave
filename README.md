# <img src="https://assets.thetawave.metalmancy.tech/promo/thetawave_logo_animated_banner.gif">

[![Demo](https://img.shields.io/badge/Demo-Play%20Now%21-blue)](https://thetawave.metalmancy.tech)
[![Build](https://github.com/thetawavegame/thetawave/actions/workflows/ci.yml/badge.svg?)](https://github.com/thetawavegame/thetawave/actions/workflows/ci.yml)
[![Discord](https://img.shields.io/badge/chat-on%20discord-green.svg?logo=discord&logoColor=fff&labelColor=1e1c24&color=8d5b3f)](https://discord.gg/4smxjcheE5)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## What is Thetawave?

Thetawave is a physics-based space shooter with procedurally generated levels, diverse foes, and distinct characters. As an open-source project, it invites collaboration and creativity from the community, allowing enthusiasts and developers to contribute to its evolution. This aspect of open-source development adds a unique dimension to Thetawave, as it grows and diversifies with inputs from its user base. Engaging in both gameplay and development, players and developers alike can experience the thrill of space combat while shaping the game's future.

## Gameplay

<div>
  <img src="https://assets.thetawave.metalmancy.tech/promo/gameplay_1.gif" width="400">
  <img src="https://assets.thetawave.metalmancy.tech/promo/gameplay_2.gif" width="400">
</div>

## Controls

| Action            | Keyboard 🖮            | Controller 🎮      |
| ----------------- | --------------------- | ----------------- |
| 🕹️ Movement        | 'WASD' / Arrow Keys   | D-Pad             |
| 🔫 Fire Weapon     | Left Click / Spacebar | Right Bumper (RB) |
| 💥 Special Ability | Right Click / Shift   | Left Bumper (LB)  |

## How to Play

**Objective:** Protect the planet from invading enemies!

**Game Over Conditions:**
- When all players' health bars are depleted from damage.
- When the blue defense bar is depleted (occurs when enemies reach the bottom of the arena).

**Power-Ups:**

<table>
  <tr>
    <td><img src="https://assets.thetawave.metalmancy.tech/promo/health_wrench.png"></td>
    <td>Wrenches restore health.</td>
  </tr>
  <tr>
    <td><img src="https://assets.thetawave.metalmancy.tech/promo/money.png"></td>
    <td>Gems increase fire rate.</td>
  </tr>
  <tr>
    <td><img src="https://assets.thetawave.metalmancy.tech/promo/armor.png"></td>
    <td>Armor negates damage.</td>
  </tr>
  <tr>
    <td><img src="https://assets.thetawave.metalmancy.tech/promo/gain_projectiles.png"></td>
  <td>Energy orbs give additional projectiles.</td>
  </tr>
</table>

**Special Mechanics:**
- Letting cargo hauler mobs reach the bottom of the arena heals the defense bar.

## Building and Playing Locally

While you can play [in the browser](https://thetawave.metalmancy.tech) (works best on Google Chrome...for now), you can
play offline in a few ways. The simplest way to play a native build of the game offline is to download the latest
[Github release](https://github.com/thetawavegame/thetawave/releases) for Windows or Linux. Then run the executable.

To run the game after compiling locally, do the following.

0. Clone the repository `git clone https://github.com/thetawavegame/thetawave.git`
0. Download our assets.

   One way is to download a recent Linux `.tar.gz` release, and extract the `assets/` directory in the tarball into your
   local `assets/` directory. We do so in the following Bash command. In Windows, just copy the files manually. Replace
   the link with that of a build from the [latest release](https://github.com/thetawavegame/thetawave/releases),
   especially if you are running off of the `main` branch.

   ```bash

   curl -Ls https://github.com/thetawavegame/thetawave/releases/download/v0.2.0/thetawave-0.2.0-x86_64-unknown-linux-gnu.tar.gz  \
       | tar -xz -C assets/ --strip-components=2 --wildcards "*/assets/*"
   ```

   If when running the `main` branch you still have missing assets, it means that we are using assets that are not yet
   published in a release. In that case, you might checkout the latest tag. Find it by running `git tag` and then
   execute, for example, `git checkout v0.2.0`.

0. Compile/run using [Cargo](https://github.com/rust-lang/cargo) as follows.

   ```bash
   cargo run --release
   ```

0. Enable the install features you want. Our install features are as follows.

   - `cli` - Enables start up parameters on the built `thetawave` executable.
   - `storage` - Enables writing user stats to a local [SQLite](https://www.sqlite.org/) database.
   - `arcade` - Enables arcade-specific features, namely through serial communication to a microcontroller.

   One can run using install features as follows.

   ```bash
   cargo run --release --features "storage,cli,arcade"
   ```

## How to Contribute

We welcome contributions from all community members. Your insights and improvements help us grow.

For complete guidelines on contributing, please see our [Contributing Guide](CONTRIBUTING.md). It includes all the necessary information on how you can participate in shaping Thetawave.

## Team

<table>
  <tr>
    <td><img src="https://avatars.githubusercontent.com/u/15306815" height="100"></td>
    <td><strong>@cdsupina</strong> 💡<br><strong>Role:</strong> Creative Director<br><strong>Contribution:</strong> Visionary leadership and creative oversight.</td>
  </tr>
  <tr>
    <td><img src="https://avatars.githubusercontent.com/u/22409608" height="100"></td>
    <td><strong>@varoonp123</strong> 💻<br><strong>Role:</strong> Lead Programmer<br><strong>Contribution:</strong> Architect of the game's deployment and technical features.</td>
  </tr>
  <tr>
    <td><img src="https://avatars.githubusercontent.com/u/26803198" height="100"></td>
    <td><strong>@LordDeatHunter</strong> 🎨<br><strong>Role:</strong> Lead Artist<br><strong>Contribution:</strong> Key driver of the game's visual style and artistic assets.</td>
  </tr>
</table>


## External Assets

### 🎵 Music
[Joel Schuman](https://joelhasa.site/) - Original Game Soundtrack

### 📢 Sound Effects
[*Space Ultimate Megapack*](https://gamesupply.itch.io/ultimate-space-game-mega-asset-package) - Comprehensive Space Audio Collection

### 🎨 Art
[Kadith's icons](https://kadith.itch.io/kadiths-free-icons) - Game Iconography

### 📜 Fonts
[*Space Madness*](https://modernmodron.itch.io/) - Font Design by Rose Frye
