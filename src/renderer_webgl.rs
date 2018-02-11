use platform::gl;
use platform::gl::types::*;

use core::Image;
use rendering::{Program, Renderer, Texture, Uniform, Vertex, VertexAttributeType};

struct GLVertexShader {
    handle: gl::ShaderHandle,
}

impl GLVertexShader {
    fn new(src: &str) -> Result<GLVertexShader, String> {
        Ok(GLVertexShader {
            handle: compile_shader(src, gl::VERTEX_SHADER)?,
        })
    }
    fn handle<'a>(&'a self) -> &'a gl::ShaderHandle {
        &self.handle
    }
}

struct GLFragmentShader {
    handle: gl::ShaderHandle,
}

impl GLFragmentShader {
    fn new(src: &str) -> Result<GLFragmentShader, String> {
        Ok(GLFragmentShader {
            handle: compile_shader(src, gl::FRAGMENT_SHADER)?,
        })
    }
    fn handle<'a>(&'a self) -> &'a gl::ShaderHandle {
        &self.handle
    }
}

type WebGLUniform = Uniform<WebGLTexture>;

pub struct WebGLProgram {
    uniforms: Vec<(String, WebGLUniform)>,
    handle: gl::ProgramHandle,
}

impl WebGLProgram {
    fn new(
        vertex_shader: GLVertexShader,
        frag_shader: GLFragmentShader,
    ) -> Result<WebGLProgram, String> {
        Ok(WebGLProgram {
            uniforms: Vec::new(),
            handle: link_program(vertex_shader.handle(), frag_shader.handle())?,
        })
    }
    fn handle<'a>(&'a self) -> &gl::ProgramHandle {
        &self.handle
    }
}

impl Program<WebGLTexture> for WebGLProgram {
    fn set_uniform(&mut self, name: &str, uniform: WebGLUniform) {
        self.uniforms.push((name.into(), uniform));
    }
    fn uniforms(&self) -> &Vec<(String, WebGLUniform)> {
        &self.uniforms
    }
}

pub struct WebGLTexture {
    handle: gl::TextureHandle,
}

impl WebGLTexture {
    fn new(size: (u32, u32)) -> WebGLTexture {
        let handle = gl::create_texture();
        gl::bind_texture(gl::TEXTURE_2D, &handle);
        gl::tex_parameter_i(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as GLint);
        gl::tex_parameter_i(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as GLint);

        gl::tex_image_2d(
            gl::TEXTURE_2D,
            0,
            gl::RGBA,
            size.0 as GLsizei,
            size.1 as GLsizei,
            0 as GLint,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            None,
        );
        WebGLTexture { handle }
    }
    fn handle<'a>(&'a self) -> &'a gl::TextureHandle {
        &self.handle
    }
}

impl Texture for WebGLTexture {
    fn set_region(&self, image: &Image, offset: (u32, u32)) {
        gl::bind_texture(gl::TEXTURE_2D, &self.handle);
        gl::tex_sub_image_2d(
            gl::TEXTURE_2D,
            0,
            offset.0 as GLint,
            offset.1 as GLint,
            image.width as GLsizei,
            image.width as GLsizei,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            &image.data,
        );
    }
}

pub struct GLRenderer();

impl Renderer for GLRenderer {
    type Texture = WebGLTexture;
    type Program = WebGLProgram;
    type VertexBuffer = gl::BufferHandle; // (vertex array, vertex buffer)

    fn create_vertex_buffer() -> Result<Self::VertexBuffer, String> {
        let vbo = gl::create_buffer();

        Ok(vbo)
    }
    fn create_program(vs: &str, fs: &str) -> Result<WebGLProgram, String> {
        let vs = GLVertexShader::new(vs)?;
        let fs = GLFragmentShader::new(fs)?;

        Ok(WebGLProgram::new(vs, fs)?)
    }
    fn create_texture(size: (u32, u32)) -> Result<WebGLTexture, String> {
        Ok(WebGLTexture::new(size))
    }

    fn render_vertices<V: Vertex>(
        vertex_buffer: &Self::VertexBuffer,
        program: &Self::Program,
        vertices: &Vec<V>,
    ) -> Result<(), String> {
        gl::blend_func(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::enable(gl::BLEND);

        // push vertex data
        gl::bind_buffer(gl::ARRAY_BUFFER, vertex_buffer);
        unsafe {
            gl::buffer_data(
                gl::ARRAY_BUFFER,
                (vertices.len() * V::stride()) as GLsizeiptr,
                vertices.as_ptr() as *const u8,
                gl::STATIC_DRAW,
            );
        }

        gl::use_program(program.handle());

        // set uniforms
        let mut texture_index = 0;
        for &(ref name, ref uniform) in program.uniforms() {
            let attr = gl::get_uniform_location(program.handle(), name);
            match uniform {
                &Uniform::Vec2(gl_vec2) => gl::uniform_2f(&attr, gl_vec2.0, gl_vec2.1),
                &Uniform::Texture(ref gl_texture) => {
                    gl::active_texture(gl::TEXTURE0 + texture_index);
                    gl::bind_texture(gl::TEXTURE_2D, gl_texture.handle());
                    gl::uniform_1i(&attr, texture_index as GLint);
                    texture_index += 1;
                }
            }
        }

        // define vertex format
        let mut step = 0;
        for (attr_name, attr_count, attr_type) in V::attributes() {
            let attr = gl::get_attrib_location(program.handle(), &attr_name)?;
            gl::enable_vertex_attrib_array(attr);
            match attr_type {
                VertexAttributeType::Float => {
                    gl::vertex_attrib_pointer(
                        attr,
                        attr_count,
                        gl::FLOAT,
                        false,
                        V::stride(),
                        step,
                    );
                }
                VertexAttributeType::Unsigned => {
                    gl::vertex_attrib_pointer(
                        attr,
                        attr_count,
                        gl::UNSIGNED_INT,
                        false,
                        V::stride(),
                        step,
                    );
                }
            }

            step += attr_count * attr_type.size();
        }

        gl::draw_arrays(gl::TRIANGLES, 0, vertices.len());

        Ok(())
    }

    fn clear(color: Option<(f32, f32, f32, f32)>) {
        let (r, g, b, a) = color.unwrap_or((0.0, 0.0, 0.0, 1.0));
        gl::clear_color(r, g, b, a);
        gl::clear(gl::COLOR_BUFFER_BIT);
    }
}

fn compile_shader(src: &str, t: GLenum) -> Result<gl::ShaderHandle, String> {
    let shader;
    shader = gl::create_shader(t);
    gl::shader_source(&shader, src);
    gl::compile_shader(&shader);

    let status = gl::get_shader_parameter(&shader, gl::COMPILE_STATUS);
    if status != (gl::TRUE as GLint) {
        let log = gl::get_shader_info_log(&shader);
        return Err(format!("Error compiling shader: {}", log));
    }
    Ok(shader)
}

fn link_program(vs: &gl::ShaderHandle, fs: &gl::ShaderHandle) -> Result<gl::ProgramHandle, String> {
    let program = gl::create_program();
    gl::attach_shader(&program, vs);
    gl::attach_shader(&program, fs);
    gl::link_program(&program);

    let status = gl::get_program_parameter(&program, gl::LINK_STATUS);
    if status != (gl::TRUE as GLint) {
        let log = gl::get_program_info_log(&program);
        return Err(format!("Error linking program: {}", log));
    }
    Ok(program)
}
