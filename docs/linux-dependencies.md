## Dependencies for linux

### Ubuntu

```sh
sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev
```

---

### Archlinux / Manjaro
The project haven't been built or tested on Arch yet.

```sh
sudo pacman -S libx11 pkgconf alsa-lib
```

---
### NixOS
The project can be built successfully on NixOS 22.11 and NixOS unstable.

Copying following code to ```shell.nix```:
```
{ pkgs ? import <nixpkgs> {} }: 
with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
  ];
  buildInputs = [
    udev alsa-lib vulkan-loader
    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
    libxkbcommon wayland # To use the wayland feature
  ];
  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
}
```

The template shown above doesn't include rustc and cargo, because there are many way to make it.

**Notice that:** If you're using NixOS 22.11 on which the rustc isn't the latest, you should install the latest rustc(>=1.65.0) by applying [rust-overlay](https://github.com/oxalica/rust-overlay) or doing as [NixOS-Wiki suggests](https://nixos.wiki/wiki/Rust).



then run:

```sh
$ nix-shell
$ cargo build --release
```
