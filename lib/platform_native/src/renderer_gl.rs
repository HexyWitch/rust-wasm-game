use std;
use gl;
use gl::types::*;
use std::ffi::CString;
use std::ptr;
use std::mem;
use std::os::raw::c_void;

use assets::Image;
use platform::rendering_api::{Program, Renderer, Texture, Uniform, Vertex, VertexAttributeType};

struct GLVertexShader {
    gl_ref: GLuint,
}

impl GLVertexShader {
    fn new(src: &str) -> Result<GLVertexShader, String> {
        Ok(GLVertexShader {
            gl_ref: compile_shader(src, gl::VERTEX_SHADER)?,
        })
    }
    fn gl_ref(&self) -> GLuint {
        self.gl_ref
    }
}

struct GLFragmentShader {
    gl_ref: GLuint,
}

impl GLFragmentShader {
    fn new(src: &str) -> Result<GLFragmentShader, String> {
        Ok(GLFragmentShader {
            gl_ref: compile_shader(src, gl::FRAGMENT_SHADER)?,
        })
    }
    fn gl_ref(&self) -> GLuint {
        self.gl_ref
    }
}

type GLUniform = Uniform<GLTexture>;

pub struct GLProgram {
    uniforms: Vec<(String, GLUniform)>,
    gl_ref: GLuint,
}

impl GLProgram {
    fn new(
        vertex_shader: GLVertexShader,
        frag_shader: GLFragmentShader,
    ) -> Result<GLProgram, String> {
        Ok(GLProgram {
            uniforms: Vec::new(),
            gl_ref: link_program(vertex_shader.gl_ref(), frag_shader.gl_ref())?,
        })
    }
    fn gl_ref(&self) -> GLuint {
        self.gl_ref
    }
}

impl Program<GLTexture> for GLProgram {
    fn set_uniform(&mut self, name: &str, uniform: GLUniform) {
        self.uniforms.push((name.into(), uniform));
    }
    fn uniforms(&self) -> &Vec<(String, GLUniform)> {
        &self.uniforms
    }
}

pub struct GLTexture {
    gl_ref: GLuint,
}

impl GLTexture {
    fn new(size: (u32, u32)) -> GLTexture {
        let mut gl_ref = 0;
        unsafe {
            gl::GenTextures(1, &mut gl_ref);
            gl::BindTexture(gl::TEXTURE_2D, gl_ref);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as GLint,
                size.0 as GLint,
                size.1 as GLint,
                0 as GLint,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                ptr::null() as *const _,
            );
        }
        GLTexture { gl_ref: gl_ref }
    }
    fn gl_ref(&self) -> GLuint {
        self.gl_ref
    }
}

impl Texture for GLTexture {
    fn set_region(&self, image: &Image, offset: (u32, u32)) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.gl_ref);
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                offset.0 as GLint,
                offset.1 as GLint,
                image.width as GLint,
                image.width as GLint,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image.data.as_ptr() as *const _,
            );
        }
    }
}

pub struct GLRenderer();

impl Renderer for GLRenderer {
    type Texture = GLTexture;
    type Program = GLProgram;
    type VertexBuffer = (GLuint, GLuint); // (vertex array, vertex buffer)

    fn create_vertex_buffer() -> Result<(GLuint, GLuint), String> {
        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
        }

        Ok((vao, vbo))
    }
    fn create_program(vs: &str, fs: &str) -> Result<GLProgram, String> {
        let vs = GLVertexShader::new(vs)?;
        let fs = GLFragmentShader::new(fs)?;

        Ok(GLProgram::new(vs, fs)?)
    }
    fn create_texture(size: (u32, u32)) -> Result<GLTexture, String> {
        Ok(GLTexture::new(size))
    }

    fn render_vertices<V: Vertex>(
        vertex_buffer: &(GLuint, GLuint),
        program: &GLProgram,
        vertices: &Vec<V>,
    ) -> Result<(), String> {
        unsafe {
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::BLEND);

            // push vertex data
            let &(vao, vbo) = vertex_buffer;
            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * V::stride()) as GLsizeiptr,
                mem::transmute(vertices.as_ptr()),
                gl::STATIC_DRAW,
            );

            gl::UseProgram(program.gl_ref());

            // set uniforms
            let mut texture_index = 0;
            for &(ref name, ref uniform) in program.uniforms() {
                let attr = gl::GetUniformLocation(
                    program.gl_ref(),
                    CString::new(name.clone().into_bytes()).unwrap().as_ptr(),
                );
                match uniform {
                    &Uniform::Vec2(gl_vec2) => {
                        gl::Uniform2f(attr, gl_vec2.0 as GLfloat, gl_vec2.1 as GLfloat)
                    }
                    &Uniform::Texture(ref gl_texture) => {
                        gl::ActiveTexture(gl::TEXTURE0 + texture_index);
                        gl::BindTexture(gl::TEXTURE_2D, gl_texture.gl_ref());
                        gl::Uniform1i(attr, texture_index as GLint);
                        texture_index += 1;
                    }
                }
            }

            // define vertex format
            let mut step = 0;
            for (attr_name, attr_count, attr_type) in V::attributes() {
                let attr = gl::GetAttribLocation(
                    program.gl_ref(),
                    CString::new(attr_name).unwrap().as_ptr(),
                );
                gl::EnableVertexAttribArray(attr as GLuint);
                match attr_type {
                    VertexAttributeType::Float => {
                        gl::VertexAttribPointer(
                            attr as GLuint,
                            attr_count as GLsizei,
                            gl::FLOAT,
                            gl::FALSE as GLboolean,
                            V::stride() as GLsizei,
                            step as *const c_void,
                        );
                    }
                    VertexAttributeType::Unsigned => {
                        gl::VertexAttribPointer(
                            attr as GLuint,
                            attr_count as GLsizei,
                            gl::UNSIGNED_INT,
                            gl::FALSE as GLboolean,
                            V::stride() as GLsizei,
                            step as *const c_void,
                        );
                    }
                }

                step += attr_count * attr_type.size();
            }

            gl::DrawArrays(gl::TRIANGLES, 0, vertices.len() as GLsizei);
        }

        Ok(())
    }

    fn clear(color: Option<(f32, f32, f32, f32)>) {
        let (r, g, b, a) = color.unwrap_or((0.0, 0.0, 0.0, 1.0));
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }
}

fn compile_shader(src: &str, t: GLenum) -> Result<GLuint, String> {
    let shader;
    unsafe {
        shader = gl::CreateShader(t);
        let c_str = CString::new(src.as_bytes()).expect("Error converting src string to c string");
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
        if status != (gl::TRUE as GLint) {
            let mut log_len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_len);
            let mut log_buffer = vec![0; log_len as usize];
            gl::GetShaderInfoLog(
                shader,
                log_len,
                ptr::null_mut(),
                log_buffer.as_mut_ptr() as *mut GLchar,
            );
            return Err(format!(
                "Error compiling shader: {}",
                std::str::from_utf8(log_buffer.as_slice())
                    .expect("Shader Info Log not in utf8 format")
            ));
        }
    }
    Ok(shader)
}

fn link_program(vs: GLuint, fs: GLuint) -> Result<GLuint, String> {
    let program;
    unsafe {
        program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut log_len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_len);
            let mut log_buffer = vec![0; log_len as usize];
            gl::GetProgramInfoLog(
                program,
                log_len - 1,
                ptr::null_mut(),
                log_buffer.as_mut_ptr() as *mut GLchar,
            );
            return Err(format!(
                "Error linking program: {}",
                std::str::from_utf8(log_buffer.as_slice())
                    .expect("Program Info Log not in utf8 format")
            ));
        }
    }
    Ok(program)
}
