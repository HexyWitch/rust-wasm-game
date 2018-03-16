pub mod types;
#[allow(dead_code)]
pub mod constants;

use failure::Error;
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
    pub fn gl_load_context(canvas_id: &str);

    fn gl_enable(capability: GLenum);
    fn gl_blend_func(sfactor: GLenum, dfactor: GLenum);
    fn gl_draw_arrays(mode: GLenum, first: GLint, count: GLsizei);
    fn gl_clear_color(r: GLclampf, g: GLclampf, b: GLclampf, a: GLclampf);
    fn gl_clear(mask: GLbitfield);

    fn gl_create_texture() -> JsValue;
    fn gl_delete_texture(texture: &JsValue);
    fn gl_bind_texture(target: GLenum, texture: &JsValue);
    fn gl_active_texture(texture: GLenum);
    fn gl_tex_parameter_i(target: GLenum, pname: GLenum, param: i32);
    fn gl_tex_image_2d_empty(
        target: GLenum,
        level: GLint,
        internal_format: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        data_type: GLenum,
    );
    fn gl_tex_image_2d(
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
    fn gl_tex_sub_image_2d(
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

    fn gl_create_shader(shader_type: GLenum) -> JsValue;
    fn gl_delete_shader(shader: &JsValue);
    fn gl_shader_source(shader: &JsValue, source: &str);
    fn gl_compile_shader(shader: &JsValue);
    fn gl_get_shader_parameter(shader: &JsValue, pname: GLenum) -> GLint;
    fn gl_get_shader_info_log(shader: &JsValue) -> String;

    fn gl_create_program() -> JsValue;
    fn gl_delete_program(program: &JsValue);
    fn gl_attach_shader(program: &JsValue, shader: &JsValue);
    fn gl_link_program(program: &JsValue);
    fn gl_use_program(program: &JsValue);
    fn gl_get_program_parameter(program: &JsValue, pname: GLenum) -> GLint;
    fn gl_get_program_info_log(program: &JsValue) -> String;

    fn gl_get_uniform_location(program: &JsValue, name: &str) -> JsValue;
    fn gl_uniform2f(location: &JsValue, v0: GLfloat, v1: GLfloat);
    fn gl_uniform1i(location: &JsValue, v0: GLint);

    fn gl_create_buffer() -> JsValue;
    fn gl_delete_buffer(buffer: &JsValue);
    fn gl_bind_buffer(target: GLenum, buffer: &JsValue);
    fn gl_buffer_data(target: GLenum, data: &[u8], usage: GLenum);

    fn gl_get_attrib_location(program: &JsValue, name: &str) -> GLint;
    fn gl_enable_vertex_attrib_array(index: AttribIndex);
    fn gl_vertex_attrib_pointer(
        index: AttribIndex,
        size: GLint,
        attrib_type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        offset: GLintptr,
    );
}

pub fn enable(cap: GLenum) {
    gl_enable(cap);
}
pub fn blend_func(sfactor: GLenum, dfactor: GLenum) {
    gl_blend_func(sfactor, dfactor);
}
pub fn draw_arrays(mode: GLenum, first: GLint, count: usize) {
    gl_draw_arrays(mode, first, count as GLsizei);
}
pub fn clear_color(r: GLclampf, g: GLclampf, b: GLclampf, a: GLclampf) {
    gl_clear_color(r, g, b, a);
}
pub fn clear(mask: GLbitfield) {
    gl_clear(mask);
}

pub fn create_texture() -> Texture {
    gl_create_texture()
}
pub fn delete_texture(texture: &Texture) {
    gl_delete_texture(texture);
}
pub fn bind_texture(target: GLenum, texture: &Texture) {
    gl_bind_texture(target, texture);
}
pub fn active_texture(texture: GLenum) {
    gl_active_texture(texture);
}
pub fn tex_parameter_i(target: GLenum, pname: GLenum, param: GLint) {
    gl_tex_parameter_i(target, pname, param);
}
pub fn tex_image_2d(
    target: GLenum,
    level: GLint,
    internal_format: GLenum,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    format: GLenum,
    data_type: GLenum,
    pixels: Option<&[u8]>,
) {
    if let Some(pixels) = pixels {
        gl_tex_image_2d(
            target,
            level,
            internal_format,
            width,
            height,
            border,
            format,
            data_type,
            pixels,
        );
    } else {
        gl_tex_image_2d_empty(
            target,
            level,
            internal_format,
            width,
            height,
            border,
            format,
            data_type,
        );
    }
}
pub fn tex_sub_image_2d(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    data_type: GLenum,
    pixels: &[u8],
) {
    gl_tex_sub_image_2d(
        target,
        level,
        xoffset,
        yoffset,
        width,
        height,
        format,
        data_type,
        pixels,
    );
}

pub fn create_shader(shader_type: GLenum) -> Shader {
    gl_create_shader(shader_type)
}
pub fn delete_shader(shader: &Shader) {
    gl_delete_shader(shader);
}
pub fn shader_source(shader: &Shader, source: &str) {
    gl_shader_source(shader, source);
}
pub fn compile_shader(shader: &Shader) {
    gl_compile_shader(shader);
}
pub fn get_shader_parameter(shader: &Shader, pname: GLenum) -> GLint {
    gl_get_shader_parameter(shader, pname)
}
pub fn get_shader_info_log<'a>(shader: &Shader) -> String {
    gl_get_shader_info_log(shader)
}

pub fn create_program() -> Program {
    gl_create_program()
}
pub fn delete_program(program: &Program) {
    gl_delete_program(program);
}
pub fn attach_shader(program: &Program, shader: &Shader) {
    gl_attach_shader(program, shader);
}
pub fn link_program(program: &Program) {
    gl_link_program(program);
}
pub fn use_program(program: &Program) {
    gl_use_program(program);
}
pub fn get_program_parameter(program: &Program, pname: GLenum) -> GLint {
    gl_get_program_parameter(program, pname)
}
pub fn get_program_info_log<'a>(program: &Program) -> String {
    gl_get_program_info_log(program)
}

pub fn get_uniform_location(program: &Program, name: &str) -> UniformLocation {
    gl_get_uniform_location(program, name)
}
pub fn uniform_2f(location: &UniformLocation, v0: GLfloat, v1: GLfloat) {
    gl_uniform2f(location, v0, v1);
}
pub fn uniform_1i(location: &UniformLocation, v0: GLint) {
    gl_uniform1i(location, v0);
}

pub fn create_buffer() -> Buffer {
    gl_create_buffer()
}
pub fn delete_buffer(buffer: &Buffer) {
    gl_delete_buffer(buffer);
}
pub fn bind_buffer(target: GLenum, buffer: &Buffer) {
    gl_bind_buffer(target, buffer);
}
pub unsafe fn buffer_data(target: GLenum, data: &[u8], usage: GLenum) {
    gl_buffer_data(target, data, usage);
}

pub fn get_attrib_location(program: &Program, name: &str) -> Result<AttribIndex, Error> {
    let location = gl_get_attrib_location(program, name);
    if location < 0 {
        Err(format_err!("Attribute '{}' could not be found", name))
    } else {
        Ok(location as AttribIndex)
    }
}

pub fn enable_vertex_attrib_array(index: AttribIndex) {
    gl_enable_vertex_attrib_array(index);
}

pub fn vertex_attrib_pointer(
    index: AttribIndex,
    size: usize,
    attrib_type: GLenum,
    normalized: bool,
    stride: usize,
    offset: usize,
) {
    gl_vertex_attrib_pointer(
        index,
        size as GLint,
        attrib_type,
        normalized,
        stride as GLsizei,
        offset as GLintptr,
    );
}
