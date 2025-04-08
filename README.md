# Varjostin

Varjostin (Finnish for lampshade) is a real-time GLSL fragment shader editor built
with Rust and [Egui].

The GLSL "slang" it supports is a subset of [Shadertoy]'s.

This app was originally written in 2.5 frenzied nights for [Aurajoki Overflow]'s
January 2025 meetup, but I duly forgot to release the source afterwards.
Everything is still in a pretty rough shape.

Compared to Shadertoy and other tools, Varjostin:

* doesn't have a built-in code editor – bring your favorite tool
* automagically exposes your uniforms' values as egui widgets
    * supports pragmas for defining widget ranges, etc. for uniforms
* recompiles shaders every time they change on the disk

## Usage

It works on my Mac – `cargo run` should get you a build that runs.

## Acknowledgements

### Shaders

Please see the header comments in each shader.
Where there is no header comment, the shader is original work for Varjostin.
Other shaders are included here for compatibility testing and demo purposes.

### Textures

* `grove.jpg`: generated with Stable Diffusion
* `texture_05.png`: https://kenney.nl/assets/prototype-textures

[Egui]: https://github.com/emilk/egui

[Shadertoy]: https://www.shadertoy.com/

[Aurajoki Overflow]: https://aurajoki-overflow.github.io/website/