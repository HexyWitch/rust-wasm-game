use failure::Error;

use js::webgl;
use js::webgl::types::*;

use core::assets::Image;
use platform::rendering_api::{Program, Renderer, Texture, Uniform, Vertex, VertexAttributeType};

struct WebGLVertexShader {
    handle: webgl::Shader,
}

impl WebGLVertexShader {
    fn new(src: &str) -> Result<WebGLVertexShader, Error> {
        Ok(WebGLVertexShader {
            handle: compile_shader(src, webgl::VERTEX_SHADER)?,
        })
    }
    fn handle<'a>(&'a self) -> &'a webgl::Shader {
        &self.handle
    }
}

impl Drop for WebGLVertexShader {
    fn drop(&mut self) {
        webgl::delete_shader(self.handle())
    }
}

struct WebGLFragmentShader {
    handle: webgl::Shader,
}

impl WebGLFragmentShader {
    fn new(src: &str) -> Result<WebGLFragmentShader, Error> {
        Ok(WebGLFragmentShader {
            handle: compile_shader(src, webgl::FRAGMENT_SHADER)?,
        })
    }
    fn handle<'a>(&'a self) -> &'a webgl::Shader {
        &self.handle
    }
}

impl Drop for WebGLFragmentShader {
    fn drop(&mut self) {
        webgl::delete_shader(self.handle())
    }
}

pub struct WebGLBuffer(webgl::Buffer);

impl WebGLBuffer {
    fn new(buffer: webgl::Buffer) -> WebGLBuffer {
        WebGLBuffer(buffer)
    }
    fn handle<'a>(&'a self) -> &'a webgl::Shader {
        &self.0
    }
}

impl Drop for WebGLBuffer {
    fn drop(&mut self) {
        webgl::delete_buffer(&self.0);
    }
}

type WebGLUniform = Uniform<WebGLTexture>;

pub struct WebGLProgram {
    uniforms: Vec<(String, WebGLUniform)>,
    handle: webgl::Program,
}

impl WebGLProgram {
    fn new(
        vertex_shader: WebGLVertexShader,
        frag_shader: WebGLFragmentShader,
    ) -> Result<WebGLProgram, Error> {
        Ok(WebGLProgram {
            uniforms: Vec::new(),
            handle: link_program(&vertex_shader, &frag_shader)?,
        })
    }
    fn handle<'a>(&'a self) -> &webgl::Program {
        &self.handle
    }
}

impl Drop for WebGLProgram {
    fn drop(&mut self) {
        webgl::delete_program(self.handle())
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

pub struct WebGLTexture(webgl::Texture);

impl WebGLTexture {
    fn new(size: (u32, u32)) -> WebGLTexture {
        let handle = webgl::create_texture();
        webgl::bind_texture(webgl::TEXTURE_2D, &handle);
        webgl::tex_parameter_i(
            webgl::TEXTURE_2D,
            webgl::TEXTURE_MIN_FILTER,
            webgl::LINEAR as GLint,
        );
        webgl::tex_parameter_i(
            webgl::TEXTURE_2D,
            webgl::TEXTURE_MAG_FILTER,
            webgl::LINEAR as GLint,
        );

        webgl::tex_image_2d_empty(
            webgl::TEXTURE_2D,
            0,
            webgl::RGBA,
            size.0 as GLsizei,
            size.1 as GLsizei,
            0 as GLint,
            webgl::RGBA,
            webgl::UNSIGNED_BYTE,
        );
        WebGLTexture(handle)
    }
    fn handle<'a>(&'a self) -> &'a webgl::Texture {
        &self.0
    }
}

impl Drop for WebGLTexture {
    fn drop(&mut self) {
        webgl::delete_texture(self.handle())
    }
}

impl Texture for WebGLTexture {
    fn set_region(&self, image: &Image, offset: (u32, u32)) {
        webgl::bind_texture(webgl::TEXTURE_2D, self.handle());
        webgl::tex_sub_image_2d_u8(
            webgl::TEXTURE_2D,
            0,
            offset.0 as GLint,
            offset.1 as GLint,
            image.width as GLsizei,
            image.height as GLsizei,
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
    type VertexBuffer = WebGLBuffer; // (vertex array, vertex buffer)

    fn create_vertex_buffer() -> Result<Self::VertexBuffer, Error> {
        let vbo = WebGLBuffer::new(webgl::create_buffer());

        Ok(vbo)
    }
    fn create_program(vs: &str, fs: &str) -> Result<WebGLProgram, Error> {
        let vs = WebGLVertexShader::new(vs)?;
        let fs = WebGLFragmentShader::new(fs)?;

        Ok(WebGLProgram::new(vs, fs)?)
    }
    fn create_texture(size: (u32, u32)) -> Result<WebGLTexture, Error> {
        Ok(WebGLTexture::new(size))
    }

    fn render_vertices<V: Vertex>(
        vertex_buffer: &Self::VertexBuffer,
        program: &Self::Program,
        vertices: &Vec<V>,
    ) -> Result<(), Error> {
        webgl::blend_func(webgl::SRC_ALPHA, webgl::ONE_MINUS_SRC_ALPHA);
        webgl::enable(webgl::BLEND);

        // push vertex data
        webgl::bind_buffer(webgl::ARRAY_BUFFER, vertex_buffer.handle());
        unsafe {
            let data = ::std::slice::from_raw_parts(
                vertices.as_ptr() as *const u8,
                vertices.len() * V::stride(),
            );
            webgl::buffer_data(webgl::ARRAY_BUFFER, data, webgl::STATIC_DRAW);
        }

        webgl::use_program(program.handle());

        // set uniforms
        let mut texture_index = 0;
        for &(ref name, ref uniform) in program.uniforms() {
            let attr = webgl::get_uniform_location(program.handle(), name);
            match uniform {
                &Uniform::Vec2(gl_vec2) => webgl::uniform2f(&attr, gl_vec2.0, gl_vec2.1),
                &Uniform::Texture(ref gl_texture) => {
                    webgl::active_texture(webgl::TEXTURE0 + texture_index);
                    webgl::bind_texture(webgl::TEXTURE_2D, gl_texture.handle());
                    webgl::uniform1i(&attr, texture_index as GLint);
                    texture_index += 1;
                }
            }
        }

        // define vertex format
        let mut step = 0;
        for (attr_name, attr_count, attr_type) in V::attributes() {
            let attr = webgl::get_attrib_location(program.handle(), &attr_name);
            if attr < 0 {
                return Err(format_err!(
                    "could not find location of attribute {}",
                    attr_name
                ));
            }
            let attr = attr as u32;
            webgl::enable_vertex_attrib_array(attr as u32);
            match attr_type {
                VertexAttributeType::Float => {
                    webgl::vertex_attrib_pointer(
                        attr,
                        attr_count as GLsizei,
                        webgl::FLOAT,
                        false,
                        V::stride() as GLsizei,
                        step,
                    );
                }
                VertexAttributeType::Unsigned => {
                    webgl::vertex_attrib_pointer(
                        attr,
                        attr_count as GLsizei,
                        webgl::UNSIGNED_INT,
                        false,
                        V::stride() as GLsizei,
                        step,
                    );
                }
            }

            step += (attr_count * attr_type.size()) as GLsizei;
        }

        webgl::draw_arrays(webgl::TRIANGLES, 0, vertices.len() as GLsizei);

        Ok(())
    }

    fn clear(color: Option<(f32, f32, f32, f32)>) {
        let (r, g, b, a) = color.unwrap_or((0.0, 0.0, 0.0, 1.0));
        webgl::clear_color(r, g, b, a);
        webgl::clear(webgl::COLOR_BUFFER_BIT);
    }
}

fn compile_shader(src: &str, t: GLenum) -> Result<webgl::Shader, Error> {
    let shader;
    shader = webgl::create_shader(t);
    webgl::shader_source(&shader, src);
    webgl::compile_shader(&shader);

    let status = webgl::get_shader_parameter(&shader, webgl::COMPILE_STATUS);
    if status != (webgl::TRUE as GLint) {
        let log = webgl::get_shader_info_log(&shader);
        return Err(format_err!("Error compiling shader: {}", log));
    }
    Ok(shader)
}

fn link_program(vs: &WebGLVertexShader, fs: &WebGLFragmentShader) -> Result<webgl::Program, Error> {
    let program = webgl::create_program();
    webgl::attach_shader(&program, vs.handle());
    webgl::attach_shader(&program, fs.handle());
    webgl::link_program(&program);

    let status = webgl::get_program_parameter(&program, webgl::LINK_STATUS);
    if status != (webgl::TRUE as GLint) {
        let log = webgl::get_program_info_log(&program);
        return Err(format_err!("Error linking program: {}", log));
    }
    Ok(program)
}
