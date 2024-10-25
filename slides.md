# MTS: Web Assembly with Rust
![rust-wasm](bytes://rust-wasm.png)
-------------------------------------------------------------------------------

# This presentation
- Intro to Web assembly (WASM)
- Why Rust with WASM?
- Some cool things you can do with Rust and WASM

-------------------------------------------------------------------------------

# What is Web Assembly?
   - A binary instruction format designed to run code efficiently on modern web browsers.
   - Provide a fast and compact runtime for executing code.
   - Enable high-performance applications.
   - Games
   - video editing software
   - scientific simulations 

![wasm-compilers](bytes://wasm-compilers.png)

-------------------------------------------------------------------------------

# Use cases
 - Heavy Computation Tasks: simulations, data analysis
 - Games and Interactive 3D Graphics running in the browser (near native speed) 

## Success stories

 - Adobe Photoshop: successfully ported Photoshop to the web using WASM.
 - Figma: used Web Assembly for certain performance-critical parts.
 - npm libraries: Many npm packages use Web Assembly under the hood.
 - A cool example: `ffmpeg-wasm`
 - often for performance-critical operations compiled from C++ or Rust.

![ffmpeg-wasm](bytes://ffmpeg-wasm.png)
-------------------------------------------------------------------------------
# Why WASM with Rust?
- Performance with memory safety.
- Rust's borrow checker.
- Easy built in WASM support.
- Many libraries (crates) support WASM out of the box.
- Handful of UI libraries with WASM support.

![rust-wasm](bytes://rust-wasm.png)
-------------------------------------------------------------------------------
# C++
Some example code...
```cpp
#include <iostream>
#include <string>

void namePet(std::string &pet) {
    pet += " Poochie";
}

int main() {
    std::string dog = "🐶";
    namePet(dog);
    std::cout << dog << std::endl;
    return 0;
}
```

🐶 Poochie Poochie
-------------------------------------------------------------------------------
# Rust
Similar code would not compile with Rust's borrow checker.
```rs
fn main() {
    let dog = String::from("🐶");
    name_pet(dog);
    name_pet(dog);
    println!("{}", dog);
}

fn name_pet(mut pet: String) {
    pet.push_str("Poochie")
}
```
```red
error[E0382]: use of moved value: `dog`
 --> src/main.rs:4:14
  |
2 |     let dog = String::from("🐶");
  |         --- move occurs because `dog` has type `String`, which does not implement the `Copy` trait
3 |     name_pet(dog);
  |              --- value moved here
4 |     name_pet(dog);
  |              ^^^ value used here after move
  |
note: consider changing this parameter type in function `name_pet` to borrow instead if owning the value isn't necessary
 
```
-------------------------------------------------------------------------------

# Rust UI frameworks that support WASM
 - Egui
 - Iced
 - Gtk_rs
 - makepad

-------------------------------------------------------------------------------
# MIDI player in makepad
 ![makepad](bytes://makepad.png)
-------------------------------------------------------------------------------
# Why not just use the DOM?

Simple widgets or components that could be made using the JS DOM as well.
!!!toggle_widget

!!!painting
-------------------------------------------------------------------------------
# Specific use cases for using WASM for UI

- Complex
- Performance-intensive UIs (games, simulations, etc.)
- Very barebones component building (egui)

Some code that animates simple shapes and animates them:
```rs
        let mut shapes = vec![];
        for &mode in &[2, 3, 5] {
            let mode = mode as f64;
            let n = 120;
            let speed = 1.5;
            let points: Vec<Pos2> = (0..=n)
                .map(|i| {
                    let t = i as f64 / (n as f64);
                    let amp = (time * speed * mode).sin() / mode;
                    let y = amp * (t * std::f64::consts::TAU / 2.0 * mode).sin();
                    to_screen * pos2(t as f32, y as f32)
                }).collect();
            shapes.push(epaint::Shape::line(points, PathStroke::new_uv(thickness, move |rect, p| {
                    let t = remap(p.x, rect.x_range(), -1.0..=1.0).abs();
                    Color32::from_rgb(
                        lerp(center_color.r() as f32..=outer_color.r() as f32, t) as u8,
                        lerp(center_color.g() as f32..=outer_color.g() as f32, t) as u8,
                        lerp(center_color.b() as f32..=outer_color.b() as f32, t) as u8,
                    )
                }),
            ));
        }
```
-------------------------------------------------------------------------------
# Code in action
- Uses the framework Egui
- Intermediate mode
- The presentation was a WASM app all along...
```rs
#[cfg(target_arch = "wasm32")]
fn main() {
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new().start(canvas_element, web_options, Box::new(|cc| {
                    cc.egui_ctx.set_visuals(egui::Visuals::light()); 
                    Ok(Box::new(egui_presentation::Presentation::new(cc))) 
                }),
            ).await.expect("failed to start eframe");
    });
}
```
!!!dancing_strings
-------------------------------------------------------------------------------
# Q&A
- [Source code](https://github.com/Pjiwm/wasm-rust-presentation) for this presentation is available online.
- Feel free to try out: [mts-presentation.web.app](https://mts-presentation.web.app/)
- Or scan the QR code:

![QR](bytes://qr.png)
