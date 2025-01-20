use glsl::parser::Parse;
use glsl::syntax::{
    Expr, FunctionDefinition, Initializer, ShaderStage, SingleDeclaration, StorageQualifier,
    TypeQualifierSpec,
};
use glsl::visitor::{Host, Visit, Visitor};

pub struct PreparseResult {
    pub(crate) uniforms: Vec<UniformInfo>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UniformInfo {
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
}

#[derive(Clone, Debug, PartialEq)]
pub struct IntUniformSpec {
    pub default: Option<i32>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FloatUniformSpec {
    pub default: Option<f32>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Vec2UniformSpec {
    pub default: Option<[f32; 2]>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Vec3UniformSpec {
    pub default: Option<[f32; 3]>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Vec4UniformSpec {
    pub default: Option<[f32; 4]>,
}

struct UniformVisitor {
    uniforms: Vec<UniformInfo>,
}

impl Default for UniformVisitor {
    fn default() -> Self {
        Self {
            uniforms: Vec::new(),
        }
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
    return false;
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
                        self.uniforms.push(UniformInfo {
                            name,
                            spec: UniformSpec::Int(IntUniformSpec {
                                default: default_number_from_declaration(declaration)
                                    .map(|i| i as i32),
                            }),
                            smell,
                        });
                    }
                    glsl::syntax::TypeSpecifierNonArray::Float => {
                        self.uniforms.push(UniformInfo {
                            name,
                            spec: UniformSpec::Float(FloatUniformSpec {
                                default: default_number_from_declaration(declaration),
                            }),
                            smell,
                        });
                    }
                    glsl::syntax::TypeSpecifierNonArray::Vec2 => {
                        self.uniforms.push(UniformInfo {
                            name,
                            spec: UniformSpec::Vec2(Vec2UniformSpec {
                                default: default_vec_from_declaration(declaration).map(|v| {
                                    let mut arr = [0.0; 2];
                                    arr.copy_from_slice(&v);
                                    arr
                                }),
                            }),
                            smell,
                        });
                    }
                    glsl::syntax::TypeSpecifierNonArray::Vec3 => {
                        self.uniforms.push(UniformInfo {
                            name,
                            spec: UniformSpec::Vec3(Vec3UniformSpec {
                                default: default_vec_from_declaration(declaration).map(|v| {
                                    let mut arr = [0.0; 3];
                                    arr.copy_from_slice(&v);
                                    arr
                                }),
                            }),
                            smell,
                        });
                    }
                    glsl::syntax::TypeSpecifierNonArray::Vec4 => {
                        self.uniforms.push(UniformInfo {
                            name,
                            spec: UniformSpec::Vec4(Vec4UniformSpec {
                                default: default_vec_from_declaration(declaration).map(|v| {
                                    let mut arr = [0.0; 4];
                                    arr.copy_from_slice(&v);
                                    arr
                                }),
                            }),
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
}

fn default_vec_from_declaration(decl: &SingleDeclaration) -> Option<Vec<f32>> {
    if let Some(Initializer::Simple(si)) = &decl.initializer {
        match si.as_ref() {
            Expr::FunCall(_fi, args) => {
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
            _ => {}
        }
    }
    eprintln!(
        "Unsupported initializer for {:?}: {:?}",
        decl.name, decl.initializer
    );
    None
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
        uniforms: visitor.uniforms,
    })
}
