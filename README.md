
# Toolbox Tuner

Toolbox Tuner is a tool to improve the experience with [toolbox](https://containertoolbx.org/).

![image](https://media.githubusercontent.com/media/13hannes11/toolbox-tuner/main/data/resources/screenshots/main_light.png)
![image](https://media.githubusercontent.com/media/13hannes11/toolbox-tuner/main/data/resources/screenshots/main_dark.png)

## Project Roadmap

The project is currently only an mvp that only is able to list toolboxes and launch a terminal inside of them.

- [x] UI prototype
- [x] [Continuous development releases](https://github.com/13hannes11/toolbox-tuner/releases/tag/dev)
- [x] Flatpak support
- [ ] Publish on Flathub


## Installing the application

The application as of now is only available as a development build. You can download it [here](https://github.com/13hannes11/toolbox-tuner/releases/tag/dev) but be aware of unfinished features and possible instabilities.


## Compiling Source

This project now uses *Gnome Builder* as main build tool and the Flatpak toolchain.

### Recommended: Building using Flatpak


1. Install dependencies:
    1. `flatpak install org.flatpak.Builder`
    2. `flatpak install org.gnome.Sdk`
    3. `flatpak install org.freedesktop.Sdk.Extension.rust-stable` 
2. Compile with:
    * Flatpak Builder by opening the project folder and pressing run

### Traditional Compilation

1. This project is built with `Relm4`, therefore, the pre-requisites for gtk4 need to be installed. Follow the [gtk4-rs book's](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_linux.html) installation instructions.
2. Compile with:
    * `cargo build` or
    * `cargo run`

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
