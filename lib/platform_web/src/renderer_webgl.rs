use js::webgl;
use js::webgl::types::*;

use assets::Image;
use platform::rendering_api::{Program, Renderer, Texture, Uniform, Vertex, VertexAttributeType};

struct GLVertexShader {
    handle: webgl::ShaderHandle,
}

impl GLVertexShader {
    fn new(src: &str) -> Result<GLVertexShader, String> {
        Ok(GLVertexShader {
            handle: compile_shader(src, webgl::VERTEX_SHADER)?,
        })
    }
    fn handle<'a>(&'a self) -> &'a webgl::ShaderHandle {
        &self.handle
    }
}

struct GLFragmentShader {
    handle: webgl::ShaderHandle,
}

impl GLFragmentShader {
    fn new(src: &str) -> Result<GLFragmentShader, String> {
        Ok(GLFragmentShader {
            handle: compile_shader(src, webgl::FRAGMENT_SHADER)?,
        })
    }
    fn handle<'a>(&'a self) -> &'a webgl::ShaderHandle {
        &self.handle
    }
}

type WebGLUniform = Uniform<WebGLTexture>;

pub struct WebGLProgram {
    uniforms: Vec<(String, WebGLUniform)>,
    handle: webgl::ProgramHandle,
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
    fn handle<'a>(&'a self) -> &webgl::ProgramHandle {
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
    handle: webgl::TextureHandle,
}

impl WebGLTexture {
    fn new(size: (u32, u32)) -> WebGLTexture {
        let handle = webgl::create_texture();
        webgl::bind_texture(webgl::TEXTURE_2D, &handle);
        webgl::tex_parameter_i(
            webgl::TEXTURE_2D,
            webgl::TEXTURE_MIN_FILTER,
            webgl::NEAREST as GLint,
        );
        webgl::tex_parameter_i(
            webgl::TEXTURE_2D,
            webgl::TEXTURE_MAG_FILTER,
            webgl::NEAREST as GLint,
        );

        webgl::tex_image_2d(
            webgl::TEXTURE_2D,
            0,
            webgl::RGBA,
            size.0 as GLsizei,
            size.1 as GLsizei,
            0 as GLint,
            webgl::RGBA,
            webgl::UNSIGNED_BYTE,
            None,
        );
        WebGLTexture { handle }
    }
    fn handle<'a>(&'a self) -> &'a webgl::TextureHandle {
        &self.handle
    }
}

impl Texture for WebGLTexture {
    fn set_region(&self, image: &Image, offset: (u32, u32)) {
        webgl::bind_texture(webgl::TEXTURE_2D, &self.handle);
        webgl::tex_sub_image_2d(
            webgl::TEXTURE_2D,
            0,
            offset.0 as GLint,
            offset.1 as GLint,
            image.width as GLsizei,
            image.width as GLsizei,
            webgl::RGBA,
            webgl::UNSIGNED_BYTE,
            &image.data,
        );
    }
}

pub struct WebGLRenderer();

impl Renderer for WebGLRenderer {
    type Texture = WebGLTexture;
    type Program = WebGLProgram;
    type VertexBuffer = webgl::BufferHandle; // (vertex array, vertex buffer)

    fn create_vertex_buffer() -> Result<Self::VertexBuffer, String> {
        let vbo = webgl::create_buffer();

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
        webgl::blend_func(webgl::SRC_ALPHA, webgl::ONE_MINUS_SRC_ALPHA);
        webgl::enable(webgl::BLEND);

        // push vertex data
        webgl::bind_buffer(webgl::ARRAY_BUFFER, vertex_buffer);
        unsafe {
            webgl::buffer_data(
                webgl::ARRAY_BUFFER,
                (vertices.len() * V::stride()) as GLsizeiptr,
                vertices.as_ptr() as *const u8,
                webgl::STATIC_DRAW,
            );
        }

        webgl::use_program(program.handle());

        // set uniforms
        let mut texture_index = 0;
        for &(ref name, ref uniform) in program.uniforms() {
            let attr = webgl::get_uniform_location(program.handle(), name);
            match uniform {
                &Uniform::Vec2(gl_vec2) => webgl::uniform_2f(&attr, gl_vec2.0, gl_vec2.1),
                &Uniform::Texture(ref gl_texture) => {
                    webgl::active_texture(webgl::TEXTURE0 + texture_index);
                    webgl::bind_texture(webgl::TEXTURE_2D, gl_texture.handle());
                    webgl::uniform_1i(&attr, texture_index as GLint);
                    texture_index += 1;
                }
            }
        }

        // define vertex format
        let mut step = 0;
        for (attr_name, attr_count, attr_type) in V::attributes() {
            let attr = webgl::get_attrib_location(program.handle(), &attr_name)?;
            webgl::enable_vertex_attrib_array(attr);
            match attr_type {
                VertexAttributeType::Float => {
                    webgl::vertex_attrib_pointer(
                        attr,
                        attr_count,
                        webgl::FLOAT,
                        false,
                        V::stride(),
                        step,
                    );
                }
                VertexAttributeType::Unsigned => {
                    webgl::vertex_attrib_pointer(
                        attr,
                        attr_count,
                        webgl::UNSIGNED_INT,
                        false,
                        V::stride(),
                        step,
                    );
                }
            }

            step += attr_count * attr_type.size();
        }

        webgl::draw_arrays(webgl::TRIANGLES, 0, vertices.len());

        Ok(())
    }

    fn clear(color: Option<(f32, f32, f32, f32)>) {
        let (r, g, b, a) = color.unwrap_or((0.0, 0.0, 0.0, 1.0));
        webgl::clear_color(r, g, b, a);
        webgl::clear(webgl::COLOR_BUFFER_BIT);
    }
}

fn compile_shader(src: &str, t: GLenum) -> Result<webgl::ShaderHandle, String> {
    let shader;
    shader = webgl::create_shader(t);
    webgl::shader_source(&shader, src);
    webgl::compile_shader(&shader);

    let status = webgl::get_shader_parameter(&shader, webgl::COMPILE_STATUS);
    if status != (webgl::TRUE as GLint) {
        let log = webgl::get_shader_info_log(&shader);
        return Err(format!("Error compiling shader: {}", log));
    }
    Ok(shader)
}

fn link_program(
    vs: &webgl::ShaderHandle,
    fs: &webgl::ShaderHandle,
) -> Result<webgl::ProgramHandle, String> {
    let program = webgl::create_program();
    webgl::attach_shader(&program, vs);
    webgl::attach_shader(&program, fs);
    webgl::link_program(&program);

    let status = webgl::get_program_parameter(&program, webgl::LINK_STATUS);
    if status != (webgl::TRUE as GLint) {
        let log = webgl::get_program_info_log(&program);
        return Err(format!("Error linking program: {}", log));
    }
    Ok(program)
}
