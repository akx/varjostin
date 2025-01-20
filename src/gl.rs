#![allow(clippy::undocumented_unsafe_blocks)]

use eframe::egui_glow;
use eframe::egui_glow::ShaderVersion;
use eframe::glow::{HasContext, NativeProgram};
use egui_glow::glow;

const VERTEX_SHADER: &str = include_str!("vertex.glsl");
const FRAGMENT_PRELUDE: &str = include_str!("fragment_prelude.glsl");

pub fn compile_program(gl: &glow::Context, fragment_source: &str) -> eyre::Result<NativeProgram> {
    let shader_version = ShaderVersion::get(gl);

    unsafe {
        let program = gl.create_program().expect("Cannot create program");

        if !shader_version.is_new_shader_interface() {
            panic!(
                "Custom 3D painting hasn't been ported to {:?}",
                shader_version
            );
        }

        let shader_sources = [
            (glow::VERTEX_SHADER, VERTEX_SHADER, ""),
            (glow::FRAGMENT_SHADER, FRAGMENT_PRELUDE, fragment_source),
        ];

        let shaders: Vec<_> = shader_sources
            .iter()
            .map(|(shader_type, shader_prelude, shader_source)| {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect("Cannot create shader");
                gl.shader_source(
                    shader,
                    &format!(
                        "{}\n{}\n#line 1\n{}",
                        shader_version.version_declaration(),
                        shader_prelude,
                        shader_source,
                    ),
                );
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    return Err(eyre::eyre!(
                        "Failed to compile {}: {}",
                        shader_type,
                        gl.get_shader_info_log(shader)
                    ));
                }

                gl.attach_shader(program, shader);
                Ok(shader)
            })
            .collect::<Result<_, eyre::Error>>()?;

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            return Err(eyre::eyre!(
                "Failed to link program: {}",
                gl.get_program_info_log(program)
            ));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }
        Ok(program)
    }
}
