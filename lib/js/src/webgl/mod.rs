pub mod types;
#[allow(dead_code)]
pub mod constants;

use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::ptr;
use failure::Error;

pub use self::constants::*;
use self::types::*;

extern "C" {
    fn js_gl_enable(capability: GLenum);
    fn js_gl_blend_func(sfactor: GLenum, dfactor: GLenum);
    fn js_gl_draw_arrays(mode: GLenum, first: GLint, count: GLsizei);
    fn js_gl_clear_color(r: GLclampf, g: GLclampf, b: GLclampf, a: GLclampf);
    fn js_gl_clear(mask: GLbitfield);

    fn js_gl_create_texture() -> TextureRef;
    fn js_gl_delete_texture(texture: TextureRef);
    fn js_gl_bind_texture(target: GLenum, texture: TextureRef);
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

    fn js_gl_create_shader(shader_type: GLenum) -> ShaderRef;
    fn js_gl_delete_shader(shader: ShaderRef);
    fn js_gl_shader_source(shader: ShaderRef, source: *const c_char);
    fn js_gl_compile_shader(shader: ShaderRef);
    fn js_gl_get_shader_parameter(shader: ShaderRef, pname: GLenum) -> GLint;
    fn js_gl_shader_info_log_len(shader: ShaderRef) -> GLsizei;
    fn js_gl_get_shader_info_log(shader: ShaderRef, size: GLsizei, log: *mut c_char);

    fn js_gl_create_program() -> GLuint;
    fn js_gl_delete_program(program: ProgramRef);
    fn js_gl_attach_shader(program: ProgramRef, shader: ShaderRef);
    fn js_gl_link_program(program: GLuint);
    fn js_gl_use_program(program: ProgramRef);
    fn js_gl_get_program_parameter(program: GLuint, pname: GLenum) -> GLint;
    fn js_gl_program_info_log_len(shader: ShaderRef) -> GLsizei;
    fn js_gl_get_program_info_log(program: GLuint, size: GLsizei, log: *mut c_char);

    fn js_gl_get_uniform_location(program: ProgramRef, name: *const c_char) -> UniformLocation;
    fn js_gl_delete_uniform_location(location: UniformLocation);
    fn js_gl_uniform2f(location: UniformLocation, v0: GLfloat, v1: GLfloat);
    fn js_gl_uniform1i(location: UniformLocation, v0: GLint);

    fn js_gl_create_buffer() -> BufferRef;
    fn js_gl_delete_buffer(buffer: BufferRef);
    fn js_gl_bind_buffer(target: GLenum, buffer: BufferRef);
    fn js_gl_buffer_data(target: GLenum, size: GLsizeiptr, data: *const u8, usage: GLenum);

    fn js_gl_get_attrib_location(program: ProgramRef, name: *const c_char) -> GLint;
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

pub type BufferRef = u32;
pub type TextureRef = u32;
pub type ProgramRef = u32;
pub type ShaderRef = u32;
pub type UniformLocation = u32;
pub type AttribIndex = GLuint;

pub struct TextureHandle(TextureRef);
impl Drop for TextureHandle {
    fn drop(&mut self) {
        unsafe {
            js_gl_delete_texture(self.0);
        }
    }
}

pub struct ShaderHandle(ShaderRef);
impl Drop for ShaderHandle {
    fn drop(&mut self) {
        unsafe {
            js_gl_delete_shader(self.0);
        }
    }
}

pub struct BufferHandle(BufferRef);
impl Drop for BufferHandle {
    fn drop(&mut self) {
        unsafe {
            js_gl_delete_buffer(self.0);
        }
    }
}

pub struct ProgramHandle(ProgramRef);
impl Drop for ProgramHandle {
    fn drop(&mut self) {
        unsafe {
            js_gl_delete_program(self.0);
        }
    }
}

pub struct UniformLocationHandle(UniformLocation);
impl Drop for UniformLocationHandle {
    fn drop(&mut self) {
        unsafe {
            js_gl_delete_uniform_location(self.0);
        }
    }
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

pub fn create_texture() -> TextureHandle {
    unsafe { TextureHandle(js_gl_create_texture()) }
}
pub fn bind_texture(target: GLenum, texture: &TextureHandle) {
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

pub fn create_shader(shader_type: GLenum) -> ShaderHandle {
    let gl_ref = unsafe { js_gl_create_shader(shader_type) };
    ShaderHandle(gl_ref)
}
pub fn shader_source(shader: &ShaderHandle, source: &str) {
    let c_source = CString::new(source).expect("Shader source not valid UTF-8");
    unsafe {
        js_gl_shader_source(shader.0, c_source.as_ptr());
    }
}
pub fn compile_shader(shader: &ShaderHandle) {
    unsafe {
        js_gl_compile_shader(shader.0);
    }
}
pub fn get_shader_parameter(shader: &ShaderHandle, pname: GLenum) -> GLint {
    unsafe { js_gl_get_shader_parameter(shader.0, pname) }
}
pub fn get_shader_info_log<'a>(shader: &ShaderHandle) -> String {
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

pub fn create_program() -> ProgramHandle {
    let gl_ref = unsafe { js_gl_create_program() };
    ProgramHandle(gl_ref)
}
pub fn attach_shader(program: &ProgramHandle, shader: &ShaderHandle) {
    unsafe {
        js_gl_attach_shader(program.0, shader.0);
    }
}
pub fn link_program(program: &ProgramHandle) {
    unsafe {
        js_gl_link_program(program.0);
    }
}
pub fn use_program(program: &ProgramHandle) {
    unsafe {
        js_gl_use_program(program.0);
    }
}
pub fn get_program_parameter(program: &ProgramHandle, pname: GLenum) -> GLint {
    unsafe { js_gl_get_program_parameter(program.0, pname) }
}
pub fn get_program_info_log<'a>(program: &ProgramHandle) -> String {
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

pub fn get_uniform_location(program: &ProgramHandle, name: &str) -> UniformLocationHandle {
    let c_str = CString::new(name).expect("uniform location name not valid UTF-8");
    let location = unsafe { js_gl_get_uniform_location(program.0, c_str.as_ptr()) };
    UniformLocationHandle(location)
}
pub fn uniform_2f(location: &UniformLocationHandle, v0: GLfloat, v1: GLfloat) {
    unsafe {
        js_gl_uniform2f(location.0, v0, v1);
    }
}
pub fn uniform_1i(location: &UniformLocationHandle, v0: GLint) {
    unsafe {
        js_gl_uniform1i(location.0, v0);
    }
}

pub fn create_buffer() -> BufferHandle {
    let gl_ref = unsafe { js_gl_create_buffer() };
    BufferHandle(gl_ref)
}
pub fn bind_buffer(target: GLenum, buffer: &BufferHandle) {
    unsafe {
        js_gl_bind_buffer(target, buffer.0);
    }
}
pub unsafe fn buffer_data(target: GLenum, size: GLsizei, data: *const u8, usage: GLenum) {
    js_gl_buffer_data(target, size, data, usage);
}

pub fn get_attrib_location(program: &ProgramHandle, name: &str) -> Result<AttribIndex, Error> {
    let c_str = CString::new(name).expect("attrib location name not UTF-8");
    let location = unsafe { js_gl_get_attrib_location(program.0, c_str.as_ptr()) };
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
