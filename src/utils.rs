use glium::{implement_vertex, IndexBuffer, VertexBuffer};

#[derive(Clone, Copy)]
pub struct Vertex {
    position: [f32; 2],
    tex_coord: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coord);

pub fn make_quad(display: &glium::Display) -> (VertexBuffer<Vertex>, IndexBuffer<u8>) {
    let vertices = [
        Vertex {
            position: [-1.0, 1.0],
            tex_coord: [0.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0],
            tex_coord: [1.0, 1.0],
        },
        Vertex {
            position: [1.0, -1.0],
            tex_coord: [1.0, 0.0],
        },
        Vertex {
            position: [-1.0, -1.0],
            tex_coord: [0.0, 0.0],
        },
    ];
    let indices = [0, 1, 2, 0, 2, 3];

    let buffer = VertexBuffer::new(display, &vertices).unwrap();
    let indices = IndexBuffer::new(
        display,
        glium::index::PrimitiveType::TrianglesList,
        &indices,
    )
    .unwrap();

    (buffer, indices)
}
