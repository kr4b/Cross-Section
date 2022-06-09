use glium::{implement_vertex, index::NoIndices, uniform, Display, Program, Surface, VertexBuffer};

use super::xsection::Line;

#[derive(Clone, Copy)]
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    tex_coord: [f32; 3],
}

implement_vertex!(Vertex, position, normal, tex_coord);

pub struct Terrain {
    buffer: VertexBuffer<Vertex>,
    indices: NoIndices,
    program: Program,
    tiles: Vec<u8>,
    vertices: Vec<Vertex>,
    lines: Vec<Line>,
    size: usize,
}

static QUAD: [[f32; 2]; 6] = [
    [0.0, 0.0],
    [1.0, 1.0],
    [0.0, 1.0],
    [0.0, 0.0],
    [1.0, 0.0],
    [1.0, 1.0],
];

impl Terrain {
    pub fn new(display: &Display, size: usize) -> Terrain {
        let mut tiles = Vec::with_capacity(size * size * size);

        for x in 0..size {
            for y in 0..size {
                for z in 0..size {
                    let xx = (x as f32 / 4.0 - 2.0).sin();
                    let zz = (z as f32 / 4.0 - 2.0).sin();
                    if -(xx * xx) * 2.5 - (zz * zz) * 4.5 + size as f32 / 2.0 > y as f32 {
                        tiles.push(1);
                    } else {
                        tiles.push(0);
                    }
                }
            }
        }

        let buffer = VertexBuffer::new(display, &Vec::new()).unwrap();
        let indices = NoIndices(glium::index::PrimitiveType::TrianglesList);

        let vertex_shader_src = std::fs::read_to_string("shaders/terrain.vert").unwrap();
        let fragment_shader_src = std::fs::read_to_string("shaders/terrain.frag").unwrap();

        let program =
            glium::Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None)
                .unwrap();

        Terrain {
            buffer,
            indices,
            program,
            tiles,
            vertices: Vec::new(),
            lines: Vec::new(),
            size,
        }
    }

    pub fn draw<'a>(
        &self,
        target: &mut glium::Frame,
        params: &glium::DrawParameters,
        view: [[f32; 4]; 4],
        perspective: [[f32; 4]; 4],
        volume: glium::uniforms::Sampler<'a, glium::texture::Texture3d>,
    ) {
        target
            .draw(
                &self.buffer,
                &self.indices,
                &self.program,
                &uniform! {
                    view: view,
                    perspective: perspective,
                    volume: volume,
                },
                params,
            )
            .unwrap();
    }

    pub fn get_unsafe(&self, x: usize, y: usize, z: usize) -> u8 {
        self.tiles[z * self.size * self.size + y * self.size + x]
    }

    pub fn get(&self, x: isize, y: isize, z: isize) -> Option<u8> {
        if x < 0
            || y < 0
            || z < 0
            || x >= self.size as isize
            || y >= self.size as isize
            || z >= self.size as isize
        {
            None
        } else {
            Some(self.get_unsafe(x as usize, y as usize, z as usize))
        }
    }

    pub fn set_unsafe(&mut self, x: usize, y: usize, z: usize, value: u8) -> u8 {
        let old = self.tiles[z * self.size * self.size + y * self.size + x];
        self.tiles[z * self.size * self.size + y * self.size + x] = value;
        old
    }

    pub fn set(&mut self, x: isize, y: isize, z: isize, value: u8) -> Option<u8> {
        if x < 0
            || y < 0
            || z < 0
            || x >= self.size as isize
            || y >= self.size as isize
            || z >= self.size as isize
        {
            None
        } else {
            Some(self.set_unsafe(x as usize, y as usize, z as usize, value))
        }
    }

    fn make_primitives(&self, x: usize, y: usize, z: usize) -> (Vec<Vertex>, Vec<Line>) {
        let have_above = self
            .get(x as isize, (y + 1) as isize, z as isize)
            .unwrap_or(0)
            != 0;
        let y_offset = if have_above { 0.0 } else { 0.5 };
        let mut vertices = Vec::new();
        let lines = vec![
            Line {
                position: [x as f32, y as f32, z as f32],
                tex_coord: [0.0, y_offset, 0.0],
            },
            Line {
                position: [x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0],
                tex_coord: [1.0, 0.5 + y_offset, 1.0],
            },
        ];

        for i in 0..2 {
            if self
                .get(x as isize + i * 2 - 1, y as isize, z as isize)
                .unwrap_or(0)
                == 0
            {
                for j in 0..6 {
                    let tex_coord = [i as f32, QUAD[j][0] * 0.5 + y_offset, QUAD[j][1]];
                    let position = [
                        x as f32 + tex_coord[0],
                        y as f32 + QUAD[j][0],
                        z as f32 + tex_coord[2],
                    ];
                    vertices.push(Vertex {
                        position,
                        normal: [i as f32 * 2.0 - 1.0, 0.0, 0.0],
                        tex_coord,
                    });
                }
            }
            if self
                .get(x as isize, y as isize + i * 2 - 1, z as isize)
                .unwrap_or(0)
                == 0
            {
                for j in 0..6 {
                    let tex_coord = [QUAD[j][0], i as f32 * 0.5 + y_offset, QUAD[j][1]];
                    let position = [
                        x as f32 + tex_coord[0],
                        y as f32 + i as f32,
                        z as f32 + tex_coord[2],
                    ];
                    vertices.push(Vertex {
                        position,
                        normal: [0.0, i as f32 * 2.0 - 1.0, 0.0],
                        tex_coord,
                    });
                }
            }
            if self
                .get(x as isize, y as isize, z as isize + i * 2 - 1)
                .unwrap_or(0)
                == 0
            {
                for j in 0..6 {
                    let tex_coord = [QUAD[j][0], QUAD[j][1] * 0.5 + y_offset, i as f32];
                    let position = [
                        x as f32 + tex_coord[0],
                        y as f32 + QUAD[j][1],
                        z as f32 + tex_coord[2],
                    ];
                    vertices.push(Vertex {
                        position,
                        normal: [0.0, 0.0, i as f32 * 2.0 - 1.0],
                        tex_coord,
                    });
                }
            }
        }

        (vertices, lines)
    }

    pub fn rebuild(&mut self, display: &Display) {
        for x in 0..self.size {
            for y in 0..self.size {
                for z in 0..self.size {
                    if self.get_unsafe(x, y, z) > 0 {
                        let (vertices, lines) = self.make_primitives(x, y, z);
                        self.vertices.extend(vertices);
                        self.lines.extend(lines);
                    }
                }
            }
        }

        self.buffer = glium::VertexBuffer::new(display, self.vertices()).unwrap();
    }

    pub fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    pub fn lines(&self) -> &[Line] {
        &self.lines
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
