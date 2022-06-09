use glium::{uniform, IndexBuffer, Program, Surface, Texture2d, VertexBuffer};
use nalgebra::base::{Matrix4, Vector3, Vector4};

use super::plane::Plane;
use super::terrain::Terrain;

pub struct Player {
    scale: Matrix4<f32>,
    buffer: VertexBuffer<super::utils::Vertex>,
    indices: IndexBuffer<u8>,
    texture: Texture2d,
    program: Program,
    x: f32,
    y: f32,
    vel_y: f32,
    on_floor: bool,
    width: f32,
}

impl Player {
    pub fn new(display: &glium::Display) -> Player {
        let (buffer, indices) = super::utils::make_quad(display);

        let vertex_shader_src = std::fs::read_to_string("shaders/player.vert").unwrap();
        let fragment_shader_src = std::fs::read_to_string("shaders/player.frag").unwrap();

        let program =
            Program::from_source(display, &vertex_shader_src, &fragment_shader_src, None).unwrap();

        let image = image::io::Reader::open("assets/character.png").unwrap().decode().unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::Texture2d::new(display, image).unwrap();

        Player {
            scale: Matrix4::new_scaling(0.5),
            buffer,
            indices,
            program,
            texture,
            x: 0.0,
            y: 0.5,
            on_floor: false,
            vel_y: 0.0,
            width: 8.0 / 16.0,
        }
    }

    pub fn draw(
        &self,
        target: &mut glium::Frame,
        params: &glium::DrawParameters,
        view: [[f32; 4]; 4],
        perspective: [[f32; 4]; 4],
        transform: [[f32; 4]; 4],
    ) {
        let translate: [[f32; 4]; 4] =
            Matrix4::new_translation(&Vector3::new(self.x, self.y, 0.0)).into();
        let scale: [[f32; 4]; 4] = self.scale.into();
        target
            .draw(
                &self.buffer,
                &self.indices,
                &self.program,
                &uniform! {
                    view: view,
                    perspective: perspective,
                    transform: transform,
                    translate: translate,
                    scale: scale,
                    tex: &self.texture,
                },
                params,
            )
            .unwrap();
    }

    fn intersect(&self, plane: &Plane, terrain: &Terrain) -> bool {
        let corners = [(self.x - self.width / 2.0, self.y - 0.5), (self.x + self.width / 2.0, self.y - 0.5)];
        let mut corners = corners
            .iter()
            .map(|(x, y)| plane.transform() * Vector4::new(*x, *y, 0.0, 1.0));
        let origin = corners.next().unwrap();
        let dir = corners.next().unwrap() - origin;
        let mut t = 0.0;

        while t <= 1.0 + std::f32::EPSILON {
            let pos = origin + t * dir;
            let x = pos.x.floor() as isize;
            let y = pos.y.floor() as isize;
            let z = pos.z.floor() as isize;
            let value = terrain.get(x, y, z).unwrap_or(0);
            if value != 0 {
                return true;
            } else {
                let dt = (((dir.x.signum() + 1.0) / 2.0 - pos.x.fract()) / dir.x)
                    .min(((dir.y.signum() + 1.0) / 2.0 - pos.y.fract()) / dir.y)
                    .min(((dir.z.signum() + 1.0) / 2.0 - pos.z.fract()) / dir.z);
                t += dt + std::f32::EPSILON;
            }
        }

        false
    }

    pub fn update(&mut self, delta_time: f32, plane: &Plane, terrain: &Terrain) {
        self.vel_y = (self.vel_y - delta_time * 30.0).max(-20.0);
        self.y += delta_time * self.vel_y;
        if delta_time != 0.0 {
            self.on_floor = false;
        }

        while self.intersect(plane, terrain) {
            self.on_floor = true;
            self.vel_y = 0.0;
            self.y = (self.y - 0.5).floor() + 1.5;
        }
    }

    pub fn walk(&mut self, plane: &Plane, terrain: &Terrain, delta_time: f32, sign: bool) {
        let sign = if sign { 1.0 } else { -1.0 };
        let delta = delta_time * 3.5 * sign;
        self.x -= delta;

        if self.intersect(plane, terrain) {
            self.x += delta;
        }
    }

    pub fn jump(&mut self) {
        if self.on_floor {
            self.vel_y = 12.0;
        }
    }
}
