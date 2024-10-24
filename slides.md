# egui
test
[www.egui.rs](www.egui.rs)
-------------------------------------------------------------------------------

# This presentation
### - What is it?
### - How does it work?
Demo:

!!!dancing_strings
-------------------------------------------------------------------------------

# What is WebAssembly?
   - a binary instruction format designed to run code efficiently on modern web browsers.
   - Provide a fast and compact runtime for executing code.
   - Enable high-performance applications
   - games
   - video editing software
   - scientific simulations 

![wasm-compilers](bytes://wasm-compilers.png)

-------------------------------------------------------------------------------

# Use cases
 - Heavy Computation Tasks: simulations, data analysis
 - Games and Interactive 3D Graphics running in the browser (near native speed) 

## Success stories

 - Adobe Photoshop: successfully ported Photoshop to the web using WASM.
 - Figma: used WebAssembly for certain performance-critical parts.
 - npm libraries: Many npm packages use WebAssembly under the hood.
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

# Scopes

* Different layouts
* Containers widgets (`ScrollArea`, `Window`, …)
* Scopes with different styling
* …
-------------------------------------------------------------------------------

# Writing a widget

``` rs
fn toggle_widget(ui: &mut Ui, on: &mut bool) -> Response {
    let desired_size = ui.spacing().interact_size.y * vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| WidgetInfo::selected(WidgetType::Checkbox, *on, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}
```

``` rs
toggle_widget(ui, &mut some_bool);
```

!!!toggle_widget
-------------------------------------------------------------------------------

# eframe
The official egui framework

* Windows, Mac, Linux, Android, iOS, Web
* `winit` on native
* `js-sys` on web
* Renders using either `glow` (OpenGL) or `wgpu`
-------------------------------------------------------------------------------

# Q&A
