use egui::ahash::HashMap;
use glsl::parser::Parse;
use glsl::syntax::{
    Expr, FunctionDefinition, Initializer, PreprocessorPragma, ShaderStage, SingleDeclaration,
    StorageQualifier, TypeQualifierSpec,
};
use glsl::visitor::{Host, Visit, Visitor};
use serde::Deserialize;
use std::ops::RangeInclusive;

#[derive(Clone)]
pub struct PreparseResult {
    pub(crate) uniforms: Vec<UniformInfo>,
}

impl PreparseResult {
    pub fn sampler_uniform_names(&self) -> Vec<String> {
        self.uniforms
            .iter()
            .filter_map(|ui| match &ui.spec {
                UniformSpec::Sampler2D => Some(ui.name.clone()),
                _ => None,
            })
            .collect()
    }
}

#[derive(Deserialize, Debug)]
struct UniformPragmaInfo {
    pub range: Option<[f32; 2]>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UniformInfo {
    pub name: String,
    pub spec: UniformSpec,
    pub smell: UniformSmell,
    pub range: RangeInclusive<f32>,
}

struct UniformVisitation {
    pub name: String,
    pub spec: UniformSpec,
    pub smell: UniformSmell,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UniformSmell {
    Unperfumed,
    Color,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UniformSpec {
    Int(IntUniformSpec),
    Float(FloatUniformSpec),
    Vec2(Vec2UniformSpec),
    Vec3(Vec3UniformSpec),
    Vec4(Vec4UniformSpec),
    Sampler2D,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IntUniformSpec {
    pub default: Option<i32>,
}

impl IntUniformSpec {
    pub fn certain_default(&self) -> i32 {
        self.default.unwrap_or(0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FloatUniformSpec {
    pub default: Option<f32>,
}

impl FloatUniformSpec {
    pub fn certain_default(&self) -> f32 {
        self.default.unwrap_or(0.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Vec2UniformSpec {
    pub default: Option<[f32; 2]>,
}

impl Vec2UniformSpec {
    pub fn certain_default(&self) -> [f32; 2] {
        self.default.unwrap_or([0.0; 2])
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Vec3UniformSpec {
    pub default: Option<[f32; 3]>,
}

impl Vec3UniformSpec {
    pub fn certain_default(&self) -> [f32; 3] {
        self.default.unwrap_or([0.0; 3])
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Vec4UniformSpec {
    pub default: Option<[f32; 4]>,
}

impl Vec4UniformSpec {
    pub fn certain_default(&self) -> [f32; 4] {
        self.default.unwrap_or([0.0; 4])
    }
}

#[derive(Default)]
struct UniformVisitor {
    uniform_visitations: Vec<UniformVisitation>,
    pragma_infos: HashMap<String, UniformPragmaInfo>,
}

impl UniformVisitor {
    pub(crate) fn bake(&self) -> Vec<UniformInfo> {
        self.uniform_visitations
            .iter()
            .map(|uv| {
                let upi = self.pragma_infos.get(&uv.name);
                let [min, max] = upi
                    .and_then(|upi| upi.range)
                    .unwrap_or([0.0, 1.0])
                    .map(|f| f);
                UniformInfo {
                    name: uv.name.clone(),
                    spec: uv.spec.clone(),
                    smell: uv.smell.clone(),
                    range: min..=max,
                }
            })
            .collect()
    }
}

fn is_uniform(declaration: &SingleDeclaration) -> bool {
    if let Some(qu) = &declaration.ty.qualifier {
        for q in &qu.qualifiers {
            if let TypeQualifierSpec::Storage(StorageQualifier::Uniform) = q {
                return true;
            }
        }
    }
    false
}

impl Visitor for UniformVisitor {
    fn visit_single_declaration(&mut self, declaration: &SingleDeclaration) -> Visit {
        if is_uniform(declaration) {
            // eprintln!("{:#?}", declaration);
            let typ = &declaration.ty.ty;
            if let Some(idfr) = &declaration.name {
                if typ.array_specifier.is_some() {
                    eprintln!("Array uniforms are not supported yet");
                    return Visit::Parent;
                }
                let name = idfr.clone().to_string();
                let smell = if name.to_lowercase().contains("color") {
                    UniformSmell::Color
                } else {
                    UniformSmell::Unperfumed
                };
                match typ.ty {
                    glsl::syntax::TypeSpecifierNonArray::Int => {
                        let default =
                            default_number_from_declaration(declaration).map(|i| i as i32);
                        self.uniform_visitations.push(UniformVisitation {
                            name,
                            spec: UniformSpec::Int(IntUniformSpec { default }),
                            smell,
                        });
                    }
                    glsl::syntax::TypeSpecifierNonArray::Float => {
                        let default = default_number_from_declaration(declaration);
                        self.uniform_visitations.push(UniformVisitation {
                            name,
                            spec: UniformSpec::Float(FloatUniformSpec { default }),
                            smell,
                        });
                    }
                    glsl::syntax::TypeSpecifierNonArray::Vec2 => {
                        let default =
                            default_vec_from_declaration(declaration).map(vec_to_slice_repeating);
                        self.uniform_visitations.push(UniformVisitation {
                            name,
                            spec: UniformSpec::Vec2(Vec2UniformSpec { default }),
                            smell,
                        });
                    }
                    glsl::syntax::TypeSpecifierNonArray::Vec3 => {
                        let default =
                            default_vec_from_declaration(declaration).map(vec_to_slice_repeating);
                        self.uniform_visitations.push(UniformVisitation {
                            name,
                            spec: UniformSpec::Vec3(Vec3UniformSpec { default }),
                            smell,
                        });
                    }
                    glsl::syntax::TypeSpecifierNonArray::Vec4 => {
                        let default =
                            default_vec_from_declaration(declaration).map(vec_to_slice_repeating);
                        self.uniform_visitations.push(UniformVisitation {
                            name,
                            spec: UniformSpec::Vec4(Vec4UniformSpec { default }),
                            smell,
                        });
                    }
                    glsl::syntax::TypeSpecifierNonArray::Sampler2D => {
                        self.uniform_visitations.push(UniformVisitation {
                            name,
                            spec: UniformSpec::Sampler2D,
                            smell,
                        });
                    }
                    _ => {
                        eprintln!("Unsupported uniform type for {}: {:?}", name, typ.ty);
                    }
                }
            }
        }

        Visit::Parent
    }
    fn visit_function_definition(&mut self, _: &FunctionDefinition) -> Visit {
        Visit::Parent
    }
    fn visit_preprocessor_pragma(&mut self, pragma: &PreprocessorPragma) -> Visit {
        if pragma.command.starts_with("@") {
            if let Some((name, rest)) = pragma.command[1..].split_once(' ') {
                match serde_json5::from_str::<UniformPragmaInfo>(rest) {
                    Ok(upi) => {
                        self.pragma_infos.insert(name.to_string(), upi);
                    }
                    Err(e) => {
                        eprintln!("Error parsing pragma {:?}: {:?}", pragma.command, e);
                    }
                }
            }
        }
        Visit::Parent
    }
}

fn default_vec_from_declaration(decl: &SingleDeclaration) -> Option<Vec<f32>> {
    if let Some(Initializer::Simple(si)) = &decl.initializer {
        if let Expr::FunCall(_fi, args) = si.as_ref() {
            // TODO: check _fi for vec call...
            let mut vec = Vec::new();
            for arg in args {
                match arg {
                    Expr::IntConst(i) => vec.push(*i as f32),
                    Expr::FloatConst(f) => vec.push(*f),
                    _ => {
                        eprintln!(
                            "Unsupported initializer call element for {:?}: {:?}",
                            decl.name, si
                        );
                        return None;
                    }
                }
            }
            return Some(vec);
        }
    }
    eprintln!(
        "Unsupported initializer for {:?}: {:?}",
        decl.name, decl.initializer
    );
    None
}

fn vec_to_slice_repeating<const N: usize>(vec: Vec<f32>) -> [f32; N] {
    let mut arr = [0.0; N];
    if !vec.is_empty() {
        for index in 0..N {
            arr[index] = vec[index % vec.len()];
        }
    }
    arr
}

fn default_number_from_declaration(decl: &SingleDeclaration) -> Option<f32> {
    if let Some(Initializer::Simple(si)) = &decl.initializer {
        match si.as_ref() {
            Expr::IntConst(i) => {
                return Some((*i) as f32);
            }
            Expr::FloatConst(f) => {
                return Some(*f);
            }
            _ => {}
        }
    }
    eprintln!(
        "Unsupported initializer for {:?}: {:?}",
        decl.name, decl.initializer
    );
    None
}

pub fn preparse_shader(source: &str) -> eyre::Result<PreparseResult> {
    let stage = ShaderStage::parse(source)?;
    let mut visitor = UniformVisitor::default();
    stage.visit(&mut visitor);
    Ok(PreparseResult {
        uniforms: visitor.bake(),
    })
}
