#[allow(dead_code)]
pub mod constants;
pub mod types;

use wasm_bindgen::prelude::*;

pub use self::constants::*;
use self::types::*;

pub type Buffer = JsValue;
pub type Texture = JsValue;
pub type Program = JsValue;
pub type Shader = JsValue;
pub type UniformLocation = JsValue;
pub type AttribIndex = GLuint;

pub fn bootstrap() {
    use super::eval;
    eval(include_str!("../../js/webgl.js"));
}

type GlContext = JsValue;

#[wasm_bindgen]
extern "C" {
    pub fn get_canvas_context(canvas_id: &str) -> GlContext;
    pub fn set_global_context(context: GlContext);
}

#[wasm_bindgen]
extern "C" {
    pub fn gl_enable(capability: GLenum);
    pub fn gl_blend_func(sfactor: GLenum, dfactor: GLenum);
    pub fn gl_draw_arrays(mode: GLenum, first: GLint, count: GLsizei);
    pub fn gl_clear_color(r: GLclampf, g: GLclampf, b: GLclampf, a: GLclampf);
    pub fn gl_clear(mask: GLbitfield);

    pub fn gl_drawing_buffer_width() -> i32;
    pub fn gl_drawing_buffer_height() -> i32;

    pub fn gl_create_texture() -> JsValue;
    pub fn gl_delete_texture(texture: &JsValue);
    pub fn gl_bind_texture(target: GLenum, texture: &JsValue);
    pub fn gl_active_texture(texture: GLenum);
    pub fn gl_tex_parameter_i(target: GLenum, pname: GLenum, param: i32);
    pub fn gl_tex_image_2d_empty(
        target: GLenum,
        level: GLint,
        internal_format: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        data_type: GLenum,
    );
    pub fn gl_tex_image_2d_u8(
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
    pub fn gl_tex_image_2d_u16(
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
    pub fn gl_tex_sub_image_2d_u8(
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
    pub fn gl_tex_sub_image_2d_u16(
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

    pub fn gl_create_shader(shader_type: GLenum) -> JsValue;
    pub fn gl_delete_shader(shader: &JsValue);
    pub fn gl_shader_source(shader: &JsValue, source: &str);
    pub fn gl_compile_shader(shader: &JsValue);
    pub fn gl_get_shader_parameter(shader: &JsValue, pname: GLenum) -> GLint;
    pub fn gl_get_shader_info_log(shader: &JsValue) -> String;

    pub fn gl_create_program() -> JsValue;
    pub fn gl_delete_program(program: &JsValue);
    pub fn gl_attach_shader(program: &JsValue, shader: &JsValue);
    pub fn gl_link_program(program: &JsValue);
    pub fn gl_use_program(program: &JsValue);
    pub fn gl_get_program_parameter(program: &JsValue, pname: GLenum) -> GLint;
    pub fn gl_get_program_info_log(program: &JsValue) -> String;

    pub fn gl_get_uniform_location(program: &JsValue, name: &str) -> JsValue;
    pub fn gl_uniform2f(location: &JsValue, v0: GLfloat, v1: GLfloat);
    pub fn gl_uniform1i(location: &JsValue, v0: GLint);

    pub fn gl_create_buffer() -> JsValue;
    pub fn gl_delete_buffer(buffer: &JsValue);
    pub fn gl_bind_buffer(target: GLenum, buffer: &JsValue);
    pub fn gl_buffer_data(target: GLenum, data: &[u8], usage: GLenum);

    pub fn gl_get_attrib_location(program: &JsValue, name: &str) -> GLint;
    pub fn gl_enable_vertex_attrib_array(index: AttribIndex);
    pub fn gl_vertex_attrib_pointer(
        index: AttribIndex,
        size: GLint,
        attrib_type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        offset: GLintptr,
    );
}
