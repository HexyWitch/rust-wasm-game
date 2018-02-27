pub mod types;
#[allow(dead_code)]
pub mod constants;

use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::ptr;
use failure::Error;

use super::{JsInner, JsValue};
pub use self::constants::*;
use self::types::*;

pub type Buffer = JsValue;
pub type Texture = JsValue;
pub type Program = JsValue;
pub type Shader = JsValue;
pub type UniformLocation = JsValue;
pub type AttribIndex = GLuint;

extern "C" {
    fn js_gl_enable(capability: GLenum);
    fn js_gl_blend_func(sfactor: GLenum, dfactor: GLenum);
    fn js_gl_draw_arrays(mode: GLenum, first: GLint, count: GLsizei);
    fn js_gl_clear_color(r: GLclampf, g: GLclampf, b: GLclampf, a: GLclampf);
    fn js_gl_clear(mask: GLbitfield);

    fn js_gl_create_texture() -> JsInner;
    fn js_gl_delete_texture(texture: JsInner);
    fn js_gl_bind_texture(target: GLenum, texture: JsInner);
    fn js_gl_active_texture(texture: GLenum);
    fn js_gl_tex_parameter_i(target: GLenum, pname: GLenum, param: i32);
    fn js_gl_tex_image_2d(
        target: GLenum,
        level: GLint,
        internalFormat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        data_type: GLenum,
        pixels: *const c_void,
    );
    fn js_gl_tex_sub_image_2d(
        target: GLuint,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        data_type: GLenum,
        pixels: *const u8,
    );

    fn js_gl_create_shader(shader_type: GLenum) -> JsInner;
    fn js_gl_delete_shader(shader: JsInner);
    fn js_gl_shader_source(shader: JsInner, source: *const c_char, source_len: usize);
    fn js_gl_compile_shader(shader: JsInner);
    fn js_gl_get_shader_parameter(shader: JsInner, pname: GLenum) -> GLint;
    fn js_gl_shader_info_log_len(shader: JsInner) -> GLsizei;
    fn js_gl_get_shader_info_log(shader: JsInner, size: GLsizei, log: *mut c_char);

    fn js_gl_create_program() -> JsInner;
    fn js_gl_delete_program(program: JsInner);
    fn js_gl_attach_shader(program: JsInner, shader: JsInner);
    fn js_gl_link_program(program: JsInner);
    fn js_gl_use_program(program: JsInner);
    fn js_gl_get_program_parameter(program: JsInner, pname: GLenum) -> GLint;
    fn js_gl_program_info_log_len(program: JsInner) -> GLsizei;
    fn js_gl_get_program_info_log(program: JsInner, size: GLsizei, log: *mut c_char);

    fn js_gl_get_uniform_location(
        program: JsInner,
        name: *const c_char,
        name_len: usize,
    ) -> JsInner;
    fn js_gl_uniform2f(location: JsInner, v0: GLfloat, v1: GLfloat);
    fn js_gl_uniform1i(location: JsInner, v0: GLint);

    fn js_gl_create_buffer() -> JsInner;
    fn js_gl_delete_buffer(buffer: JsInner);
    fn js_gl_bind_buffer(target: GLenum, buffer: JsInner);
    fn js_gl_buffer_data(target: GLenum, size: GLsizeiptr, data: *const u8, usage: GLenum);

    fn js_gl_get_attrib_location(program: JsInner, name: *const c_char, name_len: usize) -> GLint;
    fn js_gl_enable_vertex_attrib_array(index: AttribIndex);
    fn js_gl_vertex_attrib_pointer(
        index: AttribIndex,
        size: GLint,
        attrib_type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        offset: GLintptr,
    );
}

pub fn enable(cap: GLenum) {
    unsafe {
        js_gl_enable(cap);
    }
}
pub fn blend_func(sfactor: GLenum, dfactor: GLenum) {
    unsafe {
        js_gl_blend_func(sfactor, dfactor);
    }
}
pub fn draw_arrays(mode: GLenum, first: GLint, count: usize) {
    unsafe {
        js_gl_draw_arrays(mode, first, count as GLsizei);
    }
}
pub fn clear_color(r: GLclampf, g: GLclampf, b: GLclampf, a: GLclampf) {
    unsafe {
        js_gl_clear_color(r, g, b, a);
    }
}
pub fn clear(mask: GLbitfield) {
    unsafe {
        js_gl_clear(mask);
    }
}

pub fn create_texture() -> Texture {
    unsafe { Texture::new(js_gl_create_texture()) }
}
pub fn delete_texture(texture: &Texture) {
    unsafe { js_gl_delete_texture(texture.0) }
}
pub fn bind_texture(target: GLenum, texture: &Texture) {
    unsafe {
        js_gl_bind_texture(target, texture.0);
    }
}
pub fn active_texture(texture: GLenum) {
    unsafe {
        js_gl_active_texture(texture);
    }
}
pub fn tex_parameter_i(target: GLenum, pname: GLenum, param: GLint) {
    unsafe {
        js_gl_tex_parameter_i(target, pname, param);
    }
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
    unsafe {
        js_gl_tex_image_2d(
            target,
            level,
            internal_format,
            width,
            height,
            border,
            format,
            data_type,
            pixels.map_or(ptr::null(), |d| d.as_ptr() as *const c_void),
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
    unsafe {
        js_gl_tex_sub_image_2d(
            target,
            level,
            xoffset,
            yoffset,
            width,
            height,
            format,
            data_type,
            pixels.as_ptr(),
        );
    }
}

pub fn create_shader(shader_type: GLenum) -> Shader {
    unsafe { Shader::new(js_gl_create_shader(shader_type)) }
}
pub fn delete_shader(shader: &Shader) {
    unsafe { js_gl_delete_shader(shader.0) }
}
pub fn shader_source(shader: &Shader, source: &str) {
    let c_str = CString::new(source).expect("Shader source not valid UTF-8");
    unsafe {
        js_gl_shader_source(shader.0, c_str.as_ptr(), c_str.as_bytes().len());
    }
}
pub fn compile_shader(shader: &Shader) {
    unsafe {
        js_gl_compile_shader(shader.0);
    }
}
pub fn get_shader_parameter(shader: &Shader, pname: GLenum) -> GLint {
    unsafe { js_gl_get_shader_parameter(shader.0, pname) }
}
pub fn get_shader_info_log<'a>(shader: &Shader) -> String {
    unsafe {
        let len = js_gl_shader_info_log_len(shader.0);
        let mut buf = vec![0; len as usize];
        js_gl_get_shader_info_log(
            shader.0,
            len as GLsizeiptr,
            (&mut buf).as_mut_ptr() as *mut c_char,
        );
        String::from_utf8(buf).expect("Shader info log is not valid UTF-8")
    }
}

pub fn create_program() -> Program {
    unsafe { Program::new(js_gl_create_program()) }
}
pub fn delete_program(program: &Program) {
    unsafe { js_gl_delete_program(program.0) }
}
pub fn attach_shader(program: &Program, shader: &Shader) {
    unsafe {
        js_gl_attach_shader(program.0, shader.0);
    }
}
pub fn link_program(program: &Program) {
    unsafe {
        js_gl_link_program(program.0);
    }
}
pub fn use_program(program: &Program) {
    unsafe {
        js_gl_use_program(program.0);
    }
}
pub fn get_program_parameter(program: &Program, pname: GLenum) -> GLint {
    unsafe { js_gl_get_program_parameter(program.0, pname) }
}
pub fn get_program_info_log<'a>(program: &Program) -> String {
    unsafe {
        let len = js_gl_program_info_log_len(program.0);
        let mut buf = vec![0; len as usize];
        js_gl_get_program_info_log(
            program.0,
            len as GLsizeiptr,
            (&mut buf).as_mut_ptr() as *mut c_char,
        );
        String::from_utf8(buf).expect("Program info log is not valid UTF-8")
    }
}

pub fn get_uniform_location(program: &Program, name: &str) -> UniformLocation {
    let c_str = CString::new(name).expect("uniform location name not valid UTF-8");
    let location =
        unsafe { js_gl_get_uniform_location(program.0, c_str.as_ptr(), c_str.as_bytes().len()) };
    UniformLocation::new(location)
}
pub fn uniform_2f(location: &UniformLocation, v0: GLfloat, v1: GLfloat) {
    unsafe {
        js_gl_uniform2f(location.0, v0, v1);
    }
}
pub fn uniform_1i(location: &UniformLocation, v0: GLint) {
    unsafe {
        js_gl_uniform1i(location.0, v0);
    }
}

pub fn create_buffer() -> Buffer {
    unsafe { Buffer::new(js_gl_create_buffer()) }
}
pub fn delete_buffer(buffer: &Buffer) {
    unsafe { js_gl_delete_buffer(buffer.0) }
}
pub fn bind_buffer(target: GLenum, buffer: &Buffer) {
    unsafe {
        js_gl_bind_buffer(target, buffer.0);
    }
}
pub unsafe fn buffer_data(target: GLenum, size: GLsizei, data: *const u8, usage: GLenum) {
    js_gl_buffer_data(target, size, data, usage);
}

pub fn get_attrib_location(program: &Program, name: &str) -> Result<AttribIndex, Error> {
    let c_str = CString::new(name).expect("attrib location name not UTF-8");
    let location =
        unsafe { js_gl_get_attrib_location(program.0, c_str.as_ptr(), c_str.as_bytes().len()) };
    if location < 0 {
        Err(format_err!("Attribute '{}' could not be found", name))
    } else {
        Ok(location as AttribIndex)
    }
}
pub fn enable_vertex_attrib_array(index: AttribIndex) {
    unsafe {
        js_gl_enable_vertex_attrib_array(index);
    }
}
pub fn vertex_attrib_pointer(
    index: AttribIndex,
    size: usize,
    attrib_type: GLenum,
    normalized: bool,
    stride: usize,
    offset: usize,
) {
    unsafe {
        js_gl_vertex_attrib_pointer(
            index,
            size as GLint,
            attrib_type,
            normalized,
            stride as GLsizei,
            offset as GLintptr,
        );
    }
}
