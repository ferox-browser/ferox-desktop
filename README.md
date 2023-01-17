# Ferox

<!-- Bandges -->
[![](https://img.shields.io/github/v/release/ferox-browser/ferox-desktop?include_prereleases)]()
[![MPL-2.0](https://img.shields.io/github/license/ferox-browser/ferox-desktop)](https://github.com/ferox-browser/ferox-desktop/blob/master/LICENSE)
[![gtk4 0.5.5](https://img.shields.io/crates/v/gtk4?label=gtk4&logo=gtk)](https://crates.io/crates/gtk4)
[![webkit2gtk 0.18.2](https://img.shields.io/crates/v/webkit2gtk?label=webkit2gtk)](https://crates.io/crates/webkit2gtk)

> A simple, privacy respecting modern browser.

## What is Ferox?
Ferox is a simple browser that

- respects privacy of the user
- aims for minimalism
- aims for maximium of ease to use
- aims to spread the Gecko engine in the world
  
**Currently we use Webkit2, untill we managed to create Gecko bindings for Rust and GTK4.<br>You wanna help? feel free to make a pull request!**

## Build
See [official gtk4-rs installation guide](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation.html) for your platform.<br>
> we might change our build system and steps in the future
```console
git clone git@github.com:ferox-browser/ferox-desktop.git

cd ferox-desktop

cargo build --release
```

## Install

```sh
cargo install
```

## Contributing
If you wanna contribute to this repository, feel free to do so.<br>
You only can contribute throu a **pull** request to the **develop** branch. The pull request should include:
- Title must include what did you add/change/fix (short text, max 51 chars)?
- Description must include what you did and why you did it

Pull requests to other branches as **develop**, will be ignored.
*i don't know what to write here*

## Credits
Give a ⭐️ if this project helped you!
### Contributors
### Crates/Packages/Modules
> - [GTK for Rust](https://gtk-rs.org/)
> - [webkit2gtk](https://crates.io/crates/webkit2gtk)