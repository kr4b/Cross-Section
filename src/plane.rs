use glium::{uniform, IndexBuffer, Program, Surface, VertexBuffer};
use nalgebra::base::{Matrix4, Vector3};

pub struct Plane {
    transform: Matrix4<f32>,
    scale: Matrix4<f32>,
    buffer: VertexBuffer<super::utils::Vertex>,
    indices: IndexBuffer<u8>,
    program: Program,
}

impl Plane {
    pub fn new(display: &glium::Display, size: usize) -> Plane {
        let vertex_shader_src = std::fs::read_to_string("shaders/plane.vert").unwrap();
        let fragment_shader_src = std::fs::read_to_string("shaders/plane.frag").unwrap();

        let program =
            Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

        let size = size as f32;
        let mut transform = Matrix4::identity();
        transform = transform * Matrix4::new_rotation(Vector3::new(0.0, std::f32::consts::PI, 0.0));
        transform = transform
            * Matrix4::new_translation(&Vector3::new(-size / 2.0, size / 2.0, -size / 2.0));

        transform =
            transform * Matrix4::new_rotation(Vector3::new(0.0, std::f32::consts::PI / 4.0, 0.0));

        let (buffer, indices) = super::utils::make_quad(display);

        Plane {
            transform,
            scale: Matrix4::new_scaling(size / 2.0),
            buffer,
            indices, 
            program,
        }
    }

    pub fn draw(
        &self,
        target: &mut glium::Frame,
        params: &glium::DrawParameters,
        view: [[f32; 4]; 4],
        perspective: [[f32; 4]; 4],
        frame: &glium::texture::DepthTexture2d,
        width: f32,
        height: f32,
    ) {
        let transform: [[f32; 4]; 4] = self.transform().into();
        let scale: [[f32; 4]; 4] = self.scale.into();
        target
            .draw(
                &self.buffer,
                &self.indices,
                &self.program,
                &uniform! {
                    transform: transform,
                    view: view,
                    perspective: perspective,
                    scale: scale,
                    frame: frame,
                    width: width,
                    height: height,
                },
                params,
            )
            .unwrap();
    }

    pub fn strafe_x(&mut self, delta_time: f32, sign: bool) {
        let sign = if sign { 1.0 } else { -1.0 };
        self.transform =
            self.transform * Matrix4::new_translation(&Vector3::new(delta_time * sign * 5.0, 0.0, 0.0));
    }

    pub fn strafe_z(&mut self, delta_time: f32, sign: bool) {
        let sign = if sign { 1.0 } else { -1.0 };
        self.transform =
            self.transform * Matrix4::new_translation(&Vector3::new(0.0, 0.0, delta_time * sign * 5.0));
    }

    pub fn rotate(&mut self, delta_time: f32, sign: bool) {
        let sign = if sign { 1.0 } else { -1.0 };
        self.transform = self.transform
            * Matrix4::new_rotation(Vector3::new(0.0, std::f32::consts::PI / 4.0 * sign * delta_time, 0.0));
    }

    pub fn transform(&self) -> Matrix4<f32> {
        let r31 = self.transform[4 * 0 + 2];
        let r32 = self.transform[4 * 1 + 2];
        let r33 = self.transform[4 * 2 + 2];
        let mut angle = f32::atan2(-r31, f32::sqrt(r32 * r32 + r33 * r33));
        angle = angle.abs() % (std::f32::consts::PI / 2.0);
        angle = f32::min(std::f32::consts::PI / 2.0 - angle, angle);
        let scale = 1.0 / angle.cos();
        self.transform //* Matrix4::new_nonuniform_scaling(&Vector3::new(scale, 1.0, scale))
    }
}
