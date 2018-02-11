use std::rc::Rc;

use vec2::Vec2;
use rendering::{Program, Renderer, Texture, TextureAtlas, TextureImage, Uniform, Vertex,
                VertexAttributeType};

static VERTEX_SHADER: &'static str = include_str!("../shaders/vertex.glsl");
static FRAGMENT_SHADER: &'static str = include_str!("../shaders/fragment.glsl");

pub struct TexturedVertex {
    pub position: (f32, f32),
    pub tex_coord: (f32, f32),
    pub color: (f32, f32, f32, f32),
}

impl Vertex for TexturedVertex {
    fn attributes() -> Vec<(String, usize, VertexAttributeType)> {
        vec![
            ("position".into(), 2, VertexAttributeType::Float),
            ("tex_coord".into(), 2, VertexAttributeType::Float),
            ("color".into(), 4, VertexAttributeType::Float),
        ]
    }
}

pub struct SimpleRenderer<R: Renderer> {
    program: R::Program,
    vertex_buffer: R::VertexBuffer,
    vertices: Vec<TexturedVertex>,
    atlas: TextureAtlas,
    texture: Rc<R::Texture>,
}

impl<R> SimpleRenderer<R>
where
    R: Renderer,
{
    pub fn new(screen_size: (f32, f32)) -> Result<SimpleRenderer<R>, String>
    where
        R: Renderer,
    {
        let mut program = R::create_program(VERTEX_SHADER, FRAGMENT_SHADER)?;

        let texture_size = (4096, 4096);

        let texture = Rc::new(R::create_texture(texture_size)?);

        program.set_uniform("screen_size", Uniform::Vec2(screen_size));
        program.set_uniform(
            "texture_size",
            Uniform::Vec2((texture_size.0 as f32, texture_size.1 as f32)),
        );
        program.set_uniform("texture", Uniform::Texture(texture.clone()));

        Ok(SimpleRenderer::<R> {
            program: program,
            vertex_buffer: R::create_vertex_buffer()?,
            vertices: Vec::new(),
            atlas: TextureAtlas::new(texture_size),
            texture: texture,
        })
    }

    pub fn draw_texture(
        &mut self,
        texture: &TextureImage,
        position: Vec2,
        scale: f32,
        rotation: f32,
    ) -> Result<(), String> {
        let tex_region = match self.atlas.get_texture_block(texture) {
            Some(region) => region,
            None => {
                let region = self.atlas.add_texture(texture)?;
                self.texture
                    .set_region(texture.image(), (region[0], region[1]));
                region
            }
        };
        let size = (tex_region[2] - tex_region[0], tex_region[3] - tex_region[1]);

        let rect = (0.0, 0.0, size.0 as f32 * scale, size.1 as f32 * scale);

        //let rotate = |(x, y), a: f32| (x * a.cos() - y * a.sin(), x * a.sin() + y * a.cos());
        let rotate = |(x, y), _| (x, y);
        let quad = [
            rotate((rect.0, rect.1), rotation),
            rotate((rect.0, rect.3), rotation),
            rotate((rect.2, rect.3), rotation),
            rotate((rect.2, rect.1), rotation),
        ];

        let ll = (position.0 + quad[0].0, position.1 + quad[0].1);
        let ul = (position.0 + quad[1].0, position.1 + quad[1].1);
        let ur = (position.0 + quad[2].0, position.1 + quad[2].1);
        let lr = (position.0 + quad[3].0, position.1 + quad[3].1);
        let verts = [
            (ll, (tex_region[0], tex_region[1])),
            (ul, (tex_region[0], tex_region[3])),
            (lr, (tex_region[2], tex_region[1])),
            (ul, (tex_region[0], tex_region[3])),
            (ur, (tex_region[2], tex_region[3])),
            (lr, (tex_region[2], tex_region[1])),
        ];
        for &(pos, tex_coord) in verts.iter() {
            self.vertices.push(TexturedVertex {
                position: pos,
                tex_coord: (tex_coord.0 as f32, tex_coord.1 as f32),
                color: (1.0, 1.0, 1.0, 1.0),
            })
        }

        Ok(())
    }

    pub fn do_render(&mut self) -> Result<(), String> {
        R::clear(Some((0.0, 0.0, 0.0, 1.0)));

        R::render_vertices(&self.vertex_buffer, &self.program, &self.vertices)?;

        self.vertices.clear();

        Ok(())
    }
}
