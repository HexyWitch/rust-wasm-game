pub mod types;
#[allow(dead_code)]
pub mod constants;

use wasm_bindgen::prelude::*;

pub use self::constants::*;
use self::types::*;

pub type Buffer = JsValue;
pub type Texture = JsValue;
pub type Program = JsValue;
pub type Shader = JsValue;
pub type UniformLocation = JsValue;
pub type AttribIndex = GLuint;

#[wasm_bindgen(module = "./webgl")]
extern "C" {
    pub type GlContext;
    pub fn set_context(context: GlContext);
}

#[wasm_bindgen(module = "./webgl")]
extern "C" {
    pub fn enable(capability: GLenum);
    pub fn blend_func(sfactor: GLenum, dfactor: GLenum);
    pub fn draw_arrays(mode: GLenum, first: GLint, count: GLsizei);
    pub fn clear_color(r: GLclampf, g: GLclampf, b: GLclampf, a: GLclampf);
    pub fn clear(mask: GLbitfield);

    pub fn drawing_buffer_width() -> i32;
    pub fn drawing_buffer_height() -> i32;

    pub fn create_texture() -> JsValue;
    pub fn delete_texture(texture: &JsValue);
    pub fn bind_texture(target: GLenum, texture: &JsValue);
    pub fn active_texture(texture: GLenum);
    pub fn tex_parameter_i(target: GLenum, pname: GLenum, param: i32);
    pub fn tex_image_2d_empty(
        target: GLenum,
        level: GLint,
        internal_format: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        data_type: GLenum,
    );
    pub fn tex_image_2d_u8(
        target: GLenum,
        level: GLint,
        internal_format: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        data_type: GLenum,
        pixels: &[u8],
    );
    pub fn tex_image_2d_u16(
        target: GLenum,
        level: GLint,
        internal_format: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        data_type: GLenum,
        pixels: &[u16],
    );
    pub fn tex_sub_image_2d_u8(
        target: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        data_type: GLenum,
        pixels: &[u8],
    );
    pub fn tex_sub_image_2d_u16(
        target: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        data_type: GLenum,
        pixels: &[u16],
    );

    pub fn create_shader(shader_type: GLenum) -> JsValue;
    pub fn delete_shader(shader: &JsValue);
    pub fn shader_source(shader: &JsValue, source: &str);
    pub fn compile_shader(shader: &JsValue);
    pub fn get_shader_parameter(shader: &JsValue, pname: GLenum) -> GLint;
    pub fn get_shader_info_log(shader: &JsValue) -> String;

    pub fn create_program() -> JsValue;
    pub fn delete_program(program: &JsValue);
    pub fn attach_shader(program: &JsValue, shader: &JsValue);
    pub fn link_program(program: &JsValue);
    pub fn use_program(program: &JsValue);
    pub fn get_program_parameter(program: &JsValue, pname: GLenum) -> GLint;
    pub fn get_program_info_log(program: &JsValue) -> String;

    pub fn get_uniform_location(program: &JsValue, name: &str) -> JsValue;
    pub fn uniform2f(location: &JsValue, v0: GLfloat, v1: GLfloat);
    pub fn uniform1i(location: &JsValue, v0: GLint);

    pub fn create_buffer() -> JsValue;
    pub fn delete_buffer(buffer: &JsValue);
    pub fn bind_buffer(target: GLenum, buffer: &JsValue);
    pub fn buffer_data(target: GLenum, data: &[u8], usage: GLenum);

    pub fn get_attrib_location(program: &JsValue, name: &str) -> GLint;
    pub fn enable_vertex_attrib_array(index: AttribIndex);
    pub fn vertex_attrib_pointer(
        index: AttribIndex,
        size: GLint,
        attrib_type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        offset: GLintptr,
    );
}
