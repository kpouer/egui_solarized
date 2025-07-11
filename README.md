# egui_solarized

## Description

This crate is a port of [solarized](https://ethanschoonover.com/solarized) theme
to [egui](https://github.com/emilk/egui)

The crate provides a Theme. The theme exists in dark and light. These themes contain the colors
defined by the Solarized theme and can be converted into egui Visuals.
Note that some colors of the theme are not used in egui but are still available in the library if
you need them (can be useful if you do a text editor)

You can use them with egui by calling `egui_solarized::install(&ctx.egui_ctx)`

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
egui_solarized = "0.3.1"
```

Then add the following to your `main.rs`:

```rust
use egui_solarized::solarized;

fn main() {
    eframe::run_native(
        "egui demo app",
        options,
        Box::new(|ctx| {
            /// Install egui's default image loaders to open image files from the file system.
            egui_solarized::install(&ctx.egui_ctx);
            Ok(Box::new(DemoApp::default()))
        }),
    )
}
```

### Showcase

You can find a demo in the examples/demo folder.
Basically, I "stole" the widget gallery of the egui crate to demonstrate solarized theme on it

## Dark Mode
![Dark Mode](media/dark.png)

## Light Mode
![Light Mode](media/light.png)