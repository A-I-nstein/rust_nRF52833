# rust_micro-bit_v2
Embedded Rust programs for the BBC micro:bit V2. Demonstrating practical applications and learning resources.

## How to run the programs?
- Install / prepare the prerequisites.
- Connect the micro:bit to your computer.
- Move into the required project.
- Use cargo to run the program - [Run Guide](https://doc.rust-lang.org/book/ch14-01-release-profiles.html)
    - Run command to run in dev profile - cargo run
    - Run command to run in release profile - cargo run --release

## Prerequisites

### Hardware
- The BBC micro:bit V2 - [Hardware Guide](https://tech.microbit.org/hardware/)

### Software Installations
- Install "The Rust Programming Language" - [Installation Guide](https://rust-lang.github.io/rustup/installation/index.html)
- Install PuTTY - [Download Page](https://www.chiark.greenend.org.uk/~sgtatham/putty/latest.html) & [PuTTY Configuration](https://docs.rust-embedded.org/discovery-mb2/09-serial-communication/windows-tooling.html)

### rustup Setup
- Run command - rustup component add llvm-tools
- Run command - rustup target add thumbv7em-none-eabihf

### cargo Installations
- Run command - cargo install cargo-binutils --vers 0.3.3
- Run command - cargo install probe-rs-tools --locked
    - Make sure to install the prerequisites, if any - [Installation Guide](https://probe.rs/docs/getting-started/installation/)

### Final Check
- Connect the micro:bit to your computer.
- Run command - probe-rs list
    - If the installations were successful, your board should show up here.
