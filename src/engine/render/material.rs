use engine::core::ComponentBased;
use engine::asset::{Asset, AssetSystem};
use engine::render::{ShaderProgram, Texture};

use std::rc::Rc;
use std::collections::HashMap;

pub enum MaterialParam {
    Texture(Rc<Texture>),
    Float(f32),
}

pub struct Material {
    pub program: Rc<ShaderProgram>,
    pub params: HashMap<String, MaterialParam>,
}

impl Material {
    pub fn new(program: Rc<ShaderProgram>, hm: HashMap<String, MaterialParam>) -> Material {
        return Material {
            program: program,
            params: hm,
        };
    }
}

impl Asset for Material {
    type Resource = ();

    fn gather<T: AssetSystem>(_asys: &T, _fname: &str) -> Self::Resource {
        unimplemented!();
    }

    fn new_with_resource(_r: Self::Resource) -> Rc<Self> {
        unimplemented!();
    }
}

impl ComponentBased for Material {}
