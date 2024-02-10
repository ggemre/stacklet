# Stacklet

Stacklet allows you to create custom menus and applets using the standard output of any program, script, or executable.

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ul>
    <li>
      <a href="#usage">Usage</a>
    </li>
    <li>
      <a href="#installation">Installation</a>
    </li>
    <ul>
      <li>
        <a href="#for-nixos-users">For NixOS Users</a>
      </li>
      <li>
        <a href="#for-archlinux-users">For Archlinux Users</a>
      </li>
    </ul>
    <li>
      <a href="#building-from-source">Building from Source</a>
    </li>
    <ul>
      <li>
        <a href="#1-setup-with-nix">1. Setup with Nix</a>
      </li>
      <li>
        <a href="1-setup-for-everybody-else">1. Setup for Everybody Else</a>
      </li>
      <li>
        <a href="2-running-with-cargo">2. Running with Cargo</a>
      </li>
    </ul>
  </ul>
</details>

## Usage

## Installation

Stacklet is available in both the [NUR](https://nur.nix-community.org/) and the [AUR](https://aur.archlinux.org/).

### For NixOS Users

Install from the [NUR Stacklet Namespace](https://github.com/nix-community/nur-combined/blob/master/repos/ggemre/pkgs/stacklet/default.nix) using the method of your choice:

```sh
$ nix-shell -p nur.repos.ggemre.stacklet
```

or

```sh
$ nix-env -f '<nixpkgs>' -iA nur.repos.ggemre.stacklet
```

or 

```nix
# configuration.nix
environment.systemPackages = with pkgs; [
  nur.repos.ggemre.stacklet
];
```

For help on setting up the NUR on your machine, see the Nix Community [NUR How to Guide](https://github.com/nix-community/NUR?tab=readme-ov-file#how-to-use).

### For Archlinux Users

Install from the [AUR Stacklet Entry](https://aur.archlinux.org/packages/stacklet) using the AUR helper of your choice:

For example:

```sh
yay -S stacklet
```

## Building from Source

Begin by cloning this repository:

```sh
git clone https://github.com/ggemre/stacklet.git
```

In order to build Stacklet from source, you must have the necessary build dependencies installed. There are two approaches to achieving this depending on if you use Nix or not:

### 1. Setup with Nix

If you use Nix, (you are on NixOS or use the Nix package manager), then this is trivial thanks to the provided `shell.nix` and `.envrc` files, which will setup an ephemeral development environment with everything you need in the repository once you allow it do so:

```sh
direnv allow
```

### 1. Setup for everybody else

If you do not use Nix, the following build dependencies must be installed:

```
rustup
ncurses
```

Please use the stable branch of rust, (`rustup default stable`).

### 2. Running with Cargo

With the environment setup, Stacklet can be run from Cargo like so:

```sh
cargo run -- --version
```

or packaged with Cargo:

```sh
cargo build --release --locked
```

This built binary will be in `target/release`.



