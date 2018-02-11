pub mod types;
#[allow(dead_code)]
pub mod constants;

use std::ffi::{CStr, CString};
use std::slice;
use std::io::Write;
use std::os::raw::{c_char, c_void};
use std::mem;
use std::ptr;

pub use self::constants::*;
use self::types::*;

extern "C" {
    fn js_gl_create_texture() -> GLuint;
    fn js_gl_bind_texture(target: GLenum, texture: GLuint);
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
        pixels: *const u8,
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
    fn js_gl_create_buffer() -> GLuint;
    fn js_gl_blend_func(sfactor: GLenum, dfactor: GLenum);
    fn js_gl_enable(capability: GLenum);
    fn js_gl_bind_buffer(target: GLenum, buffer: GLuint);
    fn js_gl_buffer_data(target: GLenum, size: GLsizeiptr, data: *const c_void, usage: GLenum);
    fn js_gl_use_program(program: GLuint);
    fn js_gl_get_uniform_location(program: GLuint, name: *const GLchar) -> GLuint;
    fn js_gl_uniform2f(location: GLuint, v0: GLfloat, v1: GLfloat);
    fn js_gl_active_texture(texture: GLuint);
    fn js_gl_uniform1i(location: GLuint, v0: GLint);
    fn js_gl_get_attrib_location(program: GLuint, name: *const GLchar) -> GLint;
    fn js_gl_enable_vertex_attrib_array(index: GLuint);
    fn js_gl_vertex_attrib_pointer(
        index: GLuint,
        size: GLint,
        attrib_type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        offset: GLintptr,
    );
    fn js_gl_draw_arrays(mode: GLenum, first: GLint, count: GLsizei);
    fn js_gl_clear_color(r: GLclampf, g: GLclampf, b: GLclampf, a: GLclampf);
    fn js_gl_clear(mask: GLbitfield);
    fn js_gl_create_shader(shader_type: GLenum) -> GLuint;
    fn js_gl_shader_source(shader: GLuint, source: *const GLchar);
    fn js_gl_compile_shader(shader: GLuint);
    fn js_gl_get_shader_parameter(shader: GLuint, pname: GLenum) -> GLint;
    fn js_gl_shader_info_log_len(shader: GLuint) -> GLint;
    fn js_gl_get_shader_info_log(shader: GLuint, size: GLsizei, log: *mut GLchar);
    fn js_gl_create_program() -> GLuint;
    fn js_gl_attach_shader(program: GLuint, shader: GLuint);
    fn js_gl_link_program(program: GLuint);
    fn js_gl_get_program_parameter(program: GLuint, pname: GLenum) -> GLint;
    fn js_gl_program_info_log_len(program: GLuint) -> GLint;
    fn js_gl_get_program_info_log(program: GLuint, size: GLsizei, log: *mut GLchar);
}
pub unsafe fn GenTextures(n: GLsizei, textures: &mut GLuint) {
    // WebGL API only supports creating one texture at a time
    assert_eq!(n, 1);
    let gl_ref = js_gl_create_texture();
    *textures = gl_ref;
}

pub unsafe fn BindTexture(target: GLenum, texture: GLuint) {
    js_gl_bind_texture(target, texture);
}

pub unsafe fn TexParameteri(target: GLenum, pname: GLenum, param: GLint) {
    unsafe {
        js_gl_tex_parameter_i(target, pname, param);
    }
}

pub unsafe fn TexImage2D(
    target: GLenum,
    level: GLint,
    internalFormat: GLenum,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    format: GLenum,
    data_type: GLenum,
    pixels: *const u8,
) {
    unsafe {
        js_gl_tex_image_2d(
            target,
            level,
            internalFormat,
            width,
            height,
            border,
            format,
            data_type,
            pixels,
        );
    }
}

pub unsafe fn TexSubImage2D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    data_type: GLenum,
    pixels: *const u8,
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
            pixels,
        );
    }
}

pub unsafe fn GenBuffers(n: GLint, buffers: &mut GLuint) {
    // WebGL API only supports creating one buffer at a time
    assert_eq!(n, 1);
    let gl_ref = js_gl_create_buffer();
    *buffers = gl_ref;
}

pub unsafe fn BlendFunc(sfactor: GLenum, dfactor: GLenum) {
    js_gl_blend_func(sfactor, dfactor);
}

pub unsafe fn Enable(cap: GLenum) {
    js_gl_enable(cap);
}

pub unsafe fn BindBuffer(target: GLenum, buffer: GLuint) {
    js_gl_bind_buffer(target, buffer);
}

pub unsafe fn BufferData(target: GLenum, size: GLsizeiptr, data: *const u8, usage: GLenum) {
    js_gl_buffer_data(target, size, mem::transmute(data), usage);
}

pub unsafe fn UseProgram(program: GLuint) {
    js_gl_use_program(program);
}

pub unsafe fn GetUniformLocation(program: GLuint, name: *const c_char) -> GLuint {
    js_gl_get_uniform_location(program, name as *const u8)
}

pub unsafe fn Uniform2f(location: GLuint, v0: GLfloat, v1: GLfloat) {
    js_gl_uniform2f(location, v0, v1);
}

pub unsafe fn ActiveTexture(texture: GLenum) {
    js_gl_active_texture(texture);
}

pub unsafe fn Uniform1i(location: GLuint, v0: GLint) {
    js_gl_uniform1i(location, v0);
}

pub unsafe fn GetAttribLocation(program: GLuint, name: *const c_char) -> GLint {
    js_gl_get_attrib_location(program, name as *const u8)
}

pub unsafe fn EnableVertexAttribArray(index: GLuint) {
    js_gl_enable_vertex_attrib_array(index);
}

pub unsafe fn VertexAttribPointer(
    index: u32,
    size: GLsizei,
    attrib_type: GLenum,
    normalized: GLboolean,
    stride: GLsizei,
    offset: *const c_void,
) {
    js_gl_vertex_attrib_pointer(
        index,
        size,
        attrib_type,
        normalized,
        stride,
        offset as GLintptr,
    );
}

pub unsafe fn DrawArrays(mode: GLenum, first: GLint, count: GLsizei) {
    js_gl_draw_arrays(mode, first, count);
}

pub unsafe fn ClearColor(r: GLclampf, g: GLclampf, b: GLclampf, a: GLclampf) {
    js_gl_clear_color(r, g, b, a);
}

pub unsafe fn Clear(mask: GLbitfield) {
    js_gl_clear(mask);
}

pub unsafe fn CreateShader(shader_type: GLenum) -> GLuint {
    js_gl_create_shader(shader_type)
}

pub unsafe fn ShaderSource(
    shader: GLuint,
    count: GLsizei,
    source: *const *const c_char,
    _len: *const GLint,
) {
    // WebGL only allows setting one source string
    assert_eq!(count, 1);
    js_gl_shader_source(shader, (*source) as *const u8);
}

pub unsafe fn CompileShader(shader: GLuint) {
    js_gl_compile_shader(shader);
}

pub unsafe fn GetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) {
    if pname == constants::INFO_LOG_LENGTH {
        *params = js_gl_shader_info_log_len(shader);
    } else {
        *params = js_gl_get_shader_parameter(shader, pname);
    }
}

pub unsafe fn GetShaderInfoLog<'a>(
    shader: GLuint,
    max_len: GLsizei,
    _len: *mut GLsizei,
    info_log: *mut GLchar,
) {
    js_gl_get_shader_info_log(shader, max_len, info_log as *mut GLchar);
}

pub unsafe fn CreateProgram() -> GLuint {
    js_gl_create_program()
}

pub unsafe fn AttachShader(program: GLuint, shader: GLuint) {
    js_gl_attach_shader(program, shader);
}

pub unsafe fn LinkProgram(program: GLuint) {
    js_gl_link_program(program);
}

pub unsafe fn GetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint) {
    if pname == constants::INFO_LOG_LENGTH {
        *params = js_gl_program_info_log_len(program);
    } else {
        *params = js_gl_get_program_parameter(program, pname);
    }
}

pub unsafe fn GetProgramInfoLog(
    program: GLuint,
    max_len: GLsizei,
    _len: *mut GLsizei,
    info_log: *mut GLchar,
) {
    js_gl_get_program_info_log(program, max_len, info_log);
}
