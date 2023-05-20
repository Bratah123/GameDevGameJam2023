# GameDevGameJam2023
This is the repository for our entry to the [GameDev.tv Game Jam 2023](https://itch.io/jam/gamedevtv-jam-2023).

## Technical Information
### Tech Stack
This project is initialised with `Rust 1.69.0`.  
A quick cheat sheet for Rust conventions can be found [here](https://rustc-dev-guide.rust-lang.org/conventions.html), and the style guide [here](https://rust-lang.github.io/api-guidelines/).
Note that there is no need to format anything by hand, since the Rust formatting tool can accomplish this automatically.

*Bevy*, the game engine, has [API docs](https://docs.rs/bevy/latest/bevy/), as well as [code examples for every feature](https://github.com/bevyengine/bevy/tree/latest/examples#examples) provided.    
*Rapier 3D*, the physics engine, has [API docs](https://docs.rs/bevy_rapier3d/latest/bevy_rapier3d/), as well as [user guides](https://rapier.rs/docs/user_guides/bevy_plugin/getting_started_bevy/) provided.

### Set Up Your Development Environment
*This guide assumes you are using IntelliJ on 64-bit Windows 10/11, and have Visual Studio installed*
1. Download and run the Rust init tool
    - [Click here](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe) for the 64-bit Windows version
    - It should bring up a shell prompt. Hit `Enter` to proceed with installation.
    - Use `refreshenv` in PowerShell if `cargo --version` doesn't work after installing.
2. Install the `Rust` plugin on IntelliJ
3. Open Visual Studio Installer, and click `Modify` on your Visual Studio installation
4. Install `Desktop development with C++`
    - This should include the following OS dependencies that is required by *Bevy*:
    - MSVC
    - Windows SDK
    - C++ CMake tools
5. *Bevy* has a dynamic linking configuration toggle that speeds up compilation times for dev builds (NOT supported on Windows)

## Build
We periodically build and upload to the [repository releases page](https://github.com/Bratah123/GameDevGameJam2023/releases).
However, if you would like to compile from source, here are the instructions:

*Note: at the time of writing, `jigen_tensei` is currently the placeholder name for the Cargo project.*
1. `cd jigen_tensei`
2. `cargo build --release` to compile a release build for the platform you are running on
   - The output can be found at `jigen_tensei/target/release/`
   - The `.exe` file produced on Windows will run standalone (only needing the asset folder to be in the same directory)
   - See [here](https://bevy-cheatbook.github.io/platforms/wasm.html) for WASM instructions
