mod plane;
mod player;
mod terrain;
mod utils;
mod xsection;

use glium::{glutin, Surface};
use glutin::event::VirtualKeyCode;
use nalgebra::base::{Matrix, Matrix4, Vector3};
use nalgebra::geometry::Point3;

use plane::Plane;
use player::Player;
use terrain::Terrain;
use xsection::XSection;

fn main() {
    let width = 1440.0;
    let height = 720.0;
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(width, height))
        .with_title("XSection");
    let cb = glium::glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut terrain = Terrain::new(&display, 16);
    terrain.rebuild(&display);

    let size = terrain.size() as f32;
    let mut plane = Plane::new(&display, terrain.size());
    let mut player = Player::new(&display);
    let mut player_mode = false;
    let xsection = XSection::new(&display, &terrain);

    let perspective = Matrix::new_perspective(
        width / 2.0 / height,
        std::f32::consts::PI / 3.0,
        0.1,
        1000.0,
    );
    let orthographic =
        Matrix4::new_orthographic(-size / 2.0, size / 2.0, -size / 2.0, size / 2.0, -1.0, 1.0);
    let camera_position = Point3::new(-size / 2.0, size, -size / 2.0);
    let view = Matrix4::look_at_rh(
        &camera_position,
        &Point3::new(size / 2.0, size / 4.0, size / 2.0),
        &Vector3::new(0.0, 1.0, 0.0),
    );

    let identity = Matrix4::identity().into();

    let volume_size = 16;
    let mut volume_data: Vec<Vec<Vec<(u8, u8, u8)>>> = Vec::with_capacity(volume_size);
    for x in 0..volume_size {
        let mut volume_data1 = Vec::with_capacity(volume_size);
        for y in 0..volume_size * 2 {
            let mut volume_data2 = Vec::with_capacity(volume_size);
            for z in 0..volume_size {
                if y > volume_size * 2 - 3 - (((x * 3 + z) as f32).sin() * 3.0) as usize {
                    volume_data2.push((42, 110, 40));
                } else if ((x * 271 + y * 167 + z * 83) as f32).sin() > 0.5 {
                    volume_data2.push((47, 30, 22));
                } else {
                    volume_data2.push((58, 30, 16));
                }
            }
            volume_data1.push(volume_data2);
        }
        volume_data.push(volume_data1);
    }

    let texture =
        glium::texture::DepthTexture2d::empty(&display, width as u32 / 2, height as u32).unwrap();
    let volume = glium::texture::Texture3d::with_format(
        &display,
        volume_data,
        glium::texture::UncompressedFloatFormat::U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
    )
    .unwrap();

    let mut keys_held = std::collections::HashSet::new();
    let mut last_time = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        let delta_time = last_time.elapsed().as_millis() as f32 / 1000.0;
        last_time = std::time::Instant::now();
        let next_frame_time =
            last_time + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    if keys_held.contains(&VirtualKeyCode::Tab) {
                        if input.state == glutin::event::ElementState::Released {
                            player_mode = !player_mode;
                        } 
                    }

                    match input.state {
                        glutin::event::ElementState::Pressed => keys_held.insert(input.virtual_keycode.unwrap()),
                        glutin::event::ElementState::Released => keys_held.remove(&input.virtual_keycode.unwrap()),
                    };

                }
                _ => (),
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => (),
            },
            _ => (),
        }

        if keys_held.contains(&VirtualKeyCode::Escape) {
            *control_flow = glutin::event_loop::ControlFlow::Exit;
        }
        if keys_held.contains(&VirtualKeyCode::W) {
            if player_mode {
            } else {
                plane.strafe_x(delta_time, true);
            }
        }
        if keys_held.contains(&VirtualKeyCode::S) {
            if player_mode {
            } else {
                plane.strafe_x(delta_time, false);
            }
        }
        if keys_held.contains(&VirtualKeyCode::A) {
            if player_mode {
                player.walk(&plane, &terrain, delta_time, true);
            } else {
                plane.strafe_z(delta_time, true);
            }
        }
        if keys_held.contains(&VirtualKeyCode::D) {
            if player_mode {
                player.walk(&plane, &terrain, delta_time, false);
            } else {
                plane.strafe_z(delta_time, false);
            }
        }
        if keys_held.contains(&VirtualKeyCode::Space) {
            if player_mode {
                player.jump();
            }
        }
        if keys_held.contains(&VirtualKeyCode::Q) {
            plane.rotate(delta_time, true);
        }
        if keys_held.contains(&VirtualKeyCode::E) {
            plane.rotate(delta_time, false);
        }

        let mut target = display.draw();
        target.clear_color_and_depth((121.0 / 255.0, 183.0 / 255.0, 226.0 / 255.0, 1.0), 1.0);
        let view: [[f32; 4]; 4] = view.into();
        let perspective: [[f32; 4]; 4] = perspective.into();

        let mut params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            blend: glium::Blend::alpha_blending(),
            viewport: Some(glium::Rect {
                left: width as u32 / 2,
                bottom: 0,
                width: width as u32 / 2,
                height: height as u32,
            }),
            ..Default::default()
        };

        let framebuffer = glium::framebuffer::SimpleFrameBuffer::depth_only(&display, &texture).unwrap();

        let volume_sampler = glium::uniforms::Sampler::new(&volume)
            .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest);

        let orthographic = orthographic.into();

        xsection.draw(
            &mut target,
            &params,
            orthographic,
            &plane,
            volume_sampler,
        );
        framebuffer.blit_buffers_from_frame(
            &params.viewport.unwrap(),
            &glium::BlitTarget {
                left: 0,
                bottom: 0,
                width: width as i32 / 2,
                height: height as i32,
            },
            glium::uniforms::MagnifySamplerFilter::Nearest,
            glium::BlitMask {
                color: false,
                depth: true,
                stencil: false,
            },
        );
        player.draw(
            &mut target,
            &params,
            identity,
            orthographic,
            identity,
        );

        params.viewport = Some(glium::Rect {
            left: 0,
            bottom: 0,
            width: width as u32 / 2,
            height: height as u32,
        });
        terrain.draw(&mut target, &params, view, perspective, volume_sampler);

        params.depth = Default::default();
        plane.draw(
            &mut target,
            &params,
            view,
            perspective,
            &texture,
            width,
            height,
        );
        player.draw(
            &mut target,
            &params,
            view,
            perspective,
            plane.transform().into(),
        );
        target.finish().unwrap();

        player.update(delta_time, &plane, &terrain);
    });
}
