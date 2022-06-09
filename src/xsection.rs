use glium::{implement_vertex, uniform, Program, Surface, VertexBuffer};
use nalgebra::Matrix4;

use super::plane::Plane;
use super::terrain::Terrain;

#[derive(Clone, Copy)]
pub struct Line {
    pub position: [f32; 3],
    pub tex_coord: [f32; 3],
}

implement_vertex!(Line, position, tex_coord);

pub struct XSection {
    buffer: VertexBuffer<Line>,
    indices: glium::index::NoIndices,
    program: Program,
}

impl XSection {
    pub fn new(display: &glium::Display, terrain: &Terrain) -> XSection {
        let vertex_shader_src = std::fs::read_to_string("shaders/xsection.vert").unwrap();
        let fragment_shader_src = std::fs::read_to_string("shaders/xsection.frag").unwrap();
        let geometry_shader_src = std::fs::read_to_string("shaders/xsection.geom").unwrap();

        let program = Program::from_source(
            display,
            &vertex_shader_src,
            &fragment_shader_src,
            Some(&geometry_shader_src),
        )
        .unwrap();
        let size = terrain.size() as f32;

        XSection {
            buffer: VertexBuffer::new(display, terrain.lines()).unwrap(),
            indices: glium::index::NoIndices(glium::index::PrimitiveType::LinesList),
            program,
        }
    }

    pub fn draw<'a, T>(
        &self,
        target: &mut T,
        params: &glium::DrawParameters,
        projection: [[f32; 4]; 4],
        plane: &Plane,
        volume: glium::uniforms::Sampler<'a, glium::texture::Texture3d>,
    ) where
        T: Surface,
    {
        let transform: [[f32; 4]; 4] = plane.transform().try_inverse().unwrap().into();
        target
            .draw(
                &self.buffer,
                &self.indices,
                &self.program,
                &uniform! {
                    transform: transform,
                    projection: projection,
                    volume: volume
                },
                params,
            )
            .unwrap();
    }
}
