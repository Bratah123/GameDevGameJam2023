# GameDevGameJam2023
This is the repository for our entry to the [GameDev.tv Game Jam 2023](https://itch.io/jam/gamedevtv-jam-2023).  

The "two dimensions" theme is meant to be met by the duality of physical and non-physical realms in the game.

We envisioned a platformer where the protagonist has reincarnated into a 2D world (probably via truck-kun, in the vein of isekai animes). 
The protagonist retains their memories of the 3D world they came from (i.e. our world), and as an additional gift from the gods of this world, is able to eject their soul from their body for a short span of time (Dimensional Shift). 
Whilst in this state, they are able to interact with the non-physical realm - however, failure to return to their bodies before Dimensional Shift times out results in death (game over). 
In the non-physical realm, they will be able to see monsters that are causing problems in the physical one: NPCs in the physical realm being anxious or depressed. 
As the protagonist searches for a purpose in life (an answer as to why they've been summoned here), they help the people they encounter by finding creative solutions to defeating non-physical monsters (given their lack of fighting capabilities).


## NOTE: INCOMPLETE REPOSITORY!
This project died to silver bullet syndrome.  
Having both of us learn Rust + Bevy + Rapier on the job was too tough, and the complications that ldtk brought about made it much worse.  
If you came here looking for a completed Bevy project to refer to, I'm sorry but you're out of luck.

## Technical Information
### Tech Stack
This project is initialised with `Rust 1.69.0` and `Bevy 0.10.1`.  
A quick cheat sheet for Rust conventions can be found [here](https://rustc-dev-guide.rust-lang.org/conventions.html), and the style guide [here](https://rust-lang.github.io/api-guidelines/).
Note that there is no need to format anything by hand, since the Rust formatting tool can accomplish this automatically.

*Bevy*, the game engine, has [API docs](https://docs.rs/bevy/latest/bevy/), as well as [code examples for every feature](https://github.com/bevyengine/bevy/tree/latest/examples#examples) provided.    
*Rapier 2D*, the physics engine, has [API docs](https://docs.rs/bevy_rapier2d/latest/bevy_rapier2d/), as well as [user guides](https://rapier.rs/docs/user_guides/bevy_plugin/getting_started_bevy/) provided.

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
