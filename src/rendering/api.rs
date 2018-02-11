use std::rc::Rc;

use core::Image;

#[derive(Clone, Copy)]
pub enum VertexAttributeType {
    Float,
    Unsigned,
}

impl VertexAttributeType {
    pub fn size(self) -> usize {
        match self {
            VertexAttributeType::Float => 4,
            VertexAttributeType::Unsigned => 4,
        }
    }
}

pub trait Vertex {
    fn stride() -> usize {
        Self::attributes()
            .iter()
            .fold(0, |sum, a| sum + (a.1 * a.2.size()))
    }
    fn attributes() -> Vec<(String, usize, VertexAttributeType)>;
}

pub trait Texture {
    fn set_region(&self, image: &Image, offset: (u32, u32));
}

pub enum Uniform<T> {
    Vec2((f32, f32)),
    Texture(Rc<T>),
}

pub trait Program<T> {
    fn set_uniform(&mut self, name: &str, uniform: Uniform<T>);
    fn uniforms(&self) -> &Vec<(String, Uniform<T>)>;
}

pub trait Renderer {
    type Texture: Texture;
    type Program: Program<Self::Texture>;
    type VertexBuffer;

    fn create_vertex_buffer() -> Result<Self::VertexBuffer, String>;
    fn create_program(vs: &str, fs: &str) -> Result<Self::Program, String>;
    fn create_texture(size: (u32, u32)) -> Result<Self::Texture, String>;

    fn render_vertices<V: Vertex>(
        vertex_buffer: &Self::VertexBuffer,
        program: &Self::Program,
        vertices: &Vec<V>,
    ) -> Result<(), String>;
    fn clear(color: Option<(f32, f32, f32, f32)>);
}
