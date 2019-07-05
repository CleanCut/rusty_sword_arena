//! The OpenGL window is (-1.0, -1.0) in the bottom left to (1.0, 1.0) in the top right.

use crate::game::{ButtonState, ButtonValue, Color, GameEvent, Vector2};

use glium::{
    self,
    glutin::{self, ElementState},
    implement_vertex, uniform, Frame, IndexBuffer, Surface,
};
use std::cmp::min;
use std::f64::consts::PI;

#[derive(Copy, Clone, Debug)]
struct ShapeVertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(ShapeVertex, position, color);

fn create_circle_vertices(radius: f32, num_vertices: usize, color: Color) -> Vec<ShapeVertex> {
    let mut v = Vec::<ShapeVertex>::with_capacity(num_vertices + 2);
    // The center of the circle/fan
    v.push(ShapeVertex {
        position: [0.0, 0.0],
        color: [color.r, color.g, color.b],
    });
    for x in 0..=num_vertices {
        let inner: f64 = 2.0 * PI / num_vertices as f64 * x as f64;
        // Color the forward-facing vertex of the circle differently so we can have a small "sword"
        // indicator of our forward-facing direction
        let color = if x == 0 || x == num_vertices {
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            }
        } else {
            color
        };
        v.push(ShapeVertex {
            position: [inner.cos() as f32 * radius, inner.sin() as f32 * radius],
            color: [color.r, color.g, color.b],
        });
    }
    v
}

fn create_ring_vertices(radius: f32, num_vertices: usize, color: Color) -> Vec<ShapeVertex> {
    let mut v = Vec::<ShapeVertex>::with_capacity(num_vertices + 1);
    for x in 0..=num_vertices {
        let inner: f64 = 2.0 * PI / num_vertices as f64 * x as f64;
        v.push(ShapeVertex {
            position: [inner.cos() as f32 * radius, inner.sin() as f32 * radius],
            color: [color.r, color.g, color.b],
        });
    }
    v
}

/// A `Shape` can be drawn to a `Window` using its `draw_shape()` method. Use the provided `new_*`
/// methods to make a `Shape`.
#[derive(Debug)]
pub struct Shape {
    pub pos: Vector2,
    pub direction: f32,
    vertex_buffer: glium::vertex::VertexBuffer<ShapeVertex>,
    indices: glium::index::NoIndices,
}

impl Shape {
    /// Create a solid circle with a stripe that always faces `direction`.
    pub fn new_circle(
        window: &Window,
        radius: f32,
        pos: Vector2,
        direction: f32,
        color: Color,
    ) -> Self {
        let vertex_buffer =
            glium::VertexBuffer::new(&window.display, &create_circle_vertices(radius, 32, color))
                .unwrap();
        Self {
            pos,
            direction,
            vertex_buffer,
            indices: glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan),
        }
    }
    /// Create a thin ring, or outline of a circle.
    pub fn new_ring(
        window: &Window,
        radius: f32,
        pos: Vector2,
        direction: f32,
        color: Color,
    ) -> Self {
        let vertex_buffer =
            glium::VertexBuffer::new(&window.display, &create_ring_vertices(radius, 32, color))
                .unwrap();
        Self {
            pos,
            direction,
            vertex_buffer,
            indices: glium::index::NoIndices(glium::index::PrimitiveType::LineLoop),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct ImgVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
    color: [f32; 3],
    tint: u8,
}

implement_vertex!(ImgVertex, position, tex_coords, color, tint);

/// An image that can be drawn using the `Window.draw()` method.  Currently only PNG format is
/// supported.
///
/// If you are looking at an image in Photoshop, the "right" direction is the "front" of the
/// image.  `direction` is the angle in radians that the image will be rotated.
///
/// If you want your image to have transparency without getting white borders, export as a PNG-8
/// with Transparency checked, and Matte set to None.  See `media/png-settings-screenshot.png` in
/// the repository for a screenshot of the Photoshop "Export > Save for Web" settings that are known
/// to work.  Or just exporting as a 24-bit PNG might work.
#[derive(Debug)]
pub struct Img {
    pub pos: Vector2,
    pub direction: f32,
    vertex_buffer: glium::vertex::VertexBuffer<ImgVertex>,
    index_buffer: IndexBuffer<u16>,
    texture: glium::texture::CompressedTexture2d,
}

impl Img {
    /// Create a new image.  `filename` is relative to the root of the project you are running from.
    /// For example, if you created a `media` subdirectory in the root of your project and then put
    /// `soldier.png` in it, then your filename would be `media/soldier.png`.
    pub fn new(window: &Window, pos: Vector2, direction: f32, color: Option<Color>, filename: &str) -> Self {
        let file = std::fs::File::open(filename).unwrap();
        let reader = std::io::BufReader::new(file);
        let image = image::load(reader, image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image =
            glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        let texture = glium::texture::CompressedTexture2d::new(&window.display, image).unwrap();

        let tint = if color.is_some() { 1 } else { 0 };
        let color = color.unwrap_or_else(|| Color::new(1.0, 1.0, 1.0));

        let vertex_buffer = {
            let scale = 0.1;
            glium::VertexBuffer::new(
                &window.display,
                &[
                    ImgVertex {
                        position: [-scale, -scale],
                        tex_coords: [0.0, 0.0],
                        color: [color.r, color.g, color.b],
                        tint,
                    },
                    ImgVertex {
                        position: [-scale, scale],
                        tex_coords: [0.0, 1.0],
                        color: [color.r, color.g, color.b],
                        tint,
                    },
                    ImgVertex {
                        position: [scale, scale],
                        tex_coords: [1.0, 1.0],
                        color: [color.r, color.g, color.b],
                        tint,
                    },
                    ImgVertex {
                        position: [scale, -scale],
                        tex_coords: [1.0, 0.0],
                        color: [color.r, color.g, color.b],
                        tint,
                    },
                ],
            )
            .unwrap()
        };
        let index_buffer = glium::IndexBuffer::new(
            &window.display,
            glium::index::PrimitiveType::TriangleStrip,
            &[1 as u16, 2, 0, 3],
        )
        .unwrap();
        Self {
            pos,
            direction,
            vertex_buffer,
            index_buffer,
            texture,
        }
    }
}

/// An OpenGL window for displaying graphics. Also the object through which you'll receive input
/// events (mouse, keyboard, etc.)
pub struct Window {
    events_loop: glutin::EventsLoop,
    display: glium::Display,
    shape_program: glium::Program,
    img_program: glium::Program,
    screen_to_opengl: Box<dyn Fn((f64, f64)) -> Vector2>,
    target: Option<Frame>,
}

impl Window {
    /// By default, this will be a square window with a dimension of `1024px` or
    /// `(monitor height - 100px)`, whichever is smaller.  You can override the dimension by
    /// providing a value for override_dimension, for example: `Some(2048)`.
    ///
    /// `window_title` is for the OS to use on the bar above your window.
    pub fn new(override_dimension: Option<u32>, window_title: &str) -> Self {
        let events_loop = glutin::EventsLoop::new();
        let primary_monitor = events_loop.get_primary_monitor();
        let physical_size = primary_monitor.get_dimensions();
        let screen_height = physical_size
            .to_logical(primary_monitor.get_hidpi_factor())
            .height;
        let dimension = match override_dimension {
            Some(x) => f64::from(x),
            None => f64::from(min(screen_height as u32 - 100, 1024)),
        };
        let logical_size = glutin::dpi::LogicalSize::new(dimension, dimension);
        let window = glutin::WindowBuilder::new()
            .with_dimensions(logical_size)
            .with_title(window_title);
        let context = glutin::ContextBuilder::new();
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        // Create a closure that captures current screen information to use to
        // do local screen coordinate conversion for us.
        let screen_to_opengl = Box::new(move |screen_coord: (f64, f64)| -> Vector2 {
            let x = (screen_coord.0 as f32 / (0.5 * dimension) as f32) - 1.0;
            let y = 1.0 - (screen_coord.1 as f32 / (0.5 * dimension) as f32);
            Vector2 { x, y }
        });

        // For drawing shapes
        let shape_vertex_shader = r#"
        #version 140

        in vec2 position;
        in vec3 color;
        out vec3 v_color;

        uniform mat4 matrix;

        void main() {
            v_color = color;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
        "#;

        let shape_fragment_shader = r#"
            #version 140

            in vec3 v_color;
            out vec4 color;

            void main() {
                color = vec4(v_color, 1.0);
            }
        "#;

        let program = glium::Program::new(
            &display,
            glium::program::ProgramCreationInput::SourceCode {
                vertex_shader: shape_vertex_shader,
                tessellation_control_shader: None,
                tessellation_evaluation_shader: None,
                geometry_shader: None,
                fragment_shader: shape_fragment_shader,
                transform_feedback_varyings: None,
                outputs_srgb: true,
                uses_point_size: true,
            },
        )
        .unwrap();

        // For drawing images
        let vertex_shader_img = r#"
            #version 140
            uniform mat4 matrix;
            in vec2 position;
            in vec2 tex_coords;
            in vec3 color;
            in uint tint;
            out vec3 v_color;
            out vec2 v_tex_coords;
            flat out uint u_tint;
            void main() {
                u_tint = tint;
                v_color = color;
                gl_Position = matrix * vec4(position, 0.0, 1.0);
                v_tex_coords = tex_coords;
            }
        "#;

        let fragment_shader_img = r#"
            #version 140
            uniform sampler2D tex;
            in vec2 v_tex_coords;
            in vec3 v_color;
            flat in uint u_tint;
            out vec4 f_color;
            void main() {
                if ((texture(tex, v_tex_coords).a < 0.9) || (u_tint == 0u)) {
                    f_color = texture(tex, v_tex_coords);
                } else {
                    f_color = mix(texture(tex, v_tex_coords), vec4(v_color, 1.0), 0.5);
                }
            }
        "#;

        let program_img = glium::Program::new(
            &display,
            glium::program::ProgramCreationInput::SourceCode {
                vertex_shader: vertex_shader_img,
                tessellation_control_shader: None,
                tessellation_evaluation_shader: None,
                geometry_shader: None,
                fragment_shader: fragment_shader_img,
                transform_feedback_varyings: None,
                outputs_srgb: true,
                uses_point_size: true,
            },
        )
        .unwrap();

        Self {
            events_loop,
            display,
            shape_program: program,
            img_program: program_img,
            screen_to_opengl,
            target: None,
        }
    }

    /// Call `drawstart()` when you are ready to draw a new frame. It will initialize the next
    /// off-screen framebuffer and clear it to black.
    pub fn drawstart(&mut self) {
        self.target = Some(self.display.draw());
        if let Some(ref mut target) = self.target {
            target.clear_color(0.0, 0.0, 0.0, 1.0);
        }
    }

    /// You must call `.drawstart()` before calling this method.  `draw_shape()` will draw your
    /// shape to the current off-screen framebuffer.  After the first time a given shape value is
    /// drawn it stays on the GPU and during subsequent calls it only sends updated
    /// position/rotation, which is super efficient, so don't destroy and recreate shapes every
    /// frame! Draw calls draw to the framebuffer in the order that they occur, so the last shape
    /// you draw will be on top.
    pub fn draw_shape(&mut self, shape: &Shape) {
        if let Some(ref mut target) = self.target {
            let uniforms = uniform! {
                        // CAUTION: The inner arrays are COLUMNS not ROWS (left to right actually is top to bottom)
                            matrix: [
                                [shape.direction.cos() as f32, shape.direction.sin() as f32, 0.0, 0.0],
                                [-shape.direction.sin() as f32, shape.direction.cos() as f32, 0.0, 0.0],
                                [0.0, 0.0, 1.0, 0.0],
                                [shape.pos.x, shape.pos.y, 0.0, 1.0f32],
                            ]
            // Failed attempt at adding scaling into the mix
            //                let sx = 1.0f32;
            //                let sy = 1.0f32;
            //                matrix: [
            //                    [sx*shape.direction.cos() as f32, sx*shape.direction.sin() as f32, 0.0, 0.0],
            //                    [-sy * shape.direction.sin() as f32, sy *shape.direction.cos() as f32, 0.0, 0.0],
            //                    [0.0, 0.0, 1.0, 0.0],
            //                    [shape.pos.x*shape.direction.cos()-shape.pos.y*shape.direction.sin(), shape.pos.x*shape.direction.sin()+shape.pos.y*shape.direction.cos(), 0.0, 1.0f32],
            //                ]
                        };

            // These options don't seem to have any effect at all :-(
            let draw_parameters = glium::DrawParameters {
                blend: glium::Blend::alpha_blending(),
                line_width: Some(5.0),
                point_size: Some(5.0),
                smooth: Some(glium::draw_parameters::Smooth::Nicest),
                ..Default::default()
            };

            target
                .draw(
                    &shape.vertex_buffer,
                    &shape.indices,
                    &self.shape_program,
                    &uniforms,
                    &draw_parameters,
                )
                .unwrap();
        }
    }

    /// You must call `.drawstart()` before calling this method.  `draw()` will draw your
    /// image to the current off-screen framebuffer.  After the first time a given image value is
    /// drawn it stays on the GPU and during subsequent calls it only sends updated
    /// position/rotation, which is super efficient, so don't destroy and recreate images every
    /// frame! Draw calls draw to the framebuffer in the order that they occur, so the last image
    /// you draw will be on top.
    pub fn draw(&mut self, img: &Img) {
        if let Some(ref mut target) = self.target {
            let uniforms = uniform! {
            // CAUTION: The inner arrays are COLUMNS not ROWS (left to right actually is top to bottom)
                matrix: [
                    [img.direction.cos() as f32, img.direction.sin() as f32, 0.0, 0.0],
                    [-img.direction.sin() as f32, img.direction.cos() as f32, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [img.pos.x, img.pos.y, 0.0, 1.0f32],
                ],
                tex: &img.texture
            };

            // These options don't seem to have any effect at all :-(
            let draw_parameters = glium::DrawParameters {
                blend: glium::Blend::alpha_blending(),
                line_width: Some(5.0),
                point_size: Some(5.0),
                smooth: Some(glium::draw_parameters::Smooth::Nicest),
                ..Default::default()
            };

            target
                .draw(
                    &img.vertex_buffer,
                    &img.index_buffer,
                    &self.img_program,
                    &uniforms,
                    &draw_parameters,
                )
                .unwrap();
        }
    }

    /// Call `drawfinish()` when you are ready to finalize the frame and show it.  You will need to
    /// call `drawstart()` again before you can `draw()` any shapes in a new frame.  I _think_ this
    /// method blocks until the hardware is ready for a frame (vsync), so an unconstrained loop
    /// (that runs fast enough) should run at 60fps on most displays.
    pub fn drawfinish(&mut self) {
        self.target.take().unwrap().finish().unwrap();
    }

    /// For convenience this method abstracts all the possible mouse and keyboard events to
    /// `GameEvent`s, which are the events we care about for the game.
    /// The WASD and arrow keys map to directions, mouse clicks and space bar map to attacks, and
    /// the Escape key maps to quitting.  Any number of events could have occurred since we last
    /// looked, so a `Vec<GameEvent>` is returned.
    pub fn poll_game_events(&mut self) -> Vec<GameEvent> {
        let screen_to_opengl = &mut (self.screen_to_opengl);
        let mut events = Vec::<GameEvent>::new();
        self.events_loop.poll_events(|ev| {
            if let glium::glutin::Event::WindowEvent { event, .. } = ev {
                match event {
                    // Time to close the app?
                    glutin::WindowEvent::CloseRequested => events.push(GameEvent::Quit),
                    // Mouse moved
                    glutin::WindowEvent::CursorMoved { position, .. } => {
                        let mouse_pos = screen_to_opengl(position.into());
                        events.push(GameEvent::MouseMoved {
                            position: mouse_pos,
                        });
                    }
                    // Keyboard button
                    glutin::WindowEvent::KeyboardInput { input, .. } => {
                        let button_state = match input.state {
                            ElementState::Pressed => ButtonState::Pressed,
                            ElementState::Released => ButtonState::Released,
                        };
                        use glium::glutin::VirtualKeyCode::*;
                        if let Some(vkey) = input.virtual_keycode {
                            match vkey {
                                W | Up | Comma => events.push(GameEvent::Button {
                                    button_state,
                                    button_value: ButtonValue::Up,
                                }),
                                S | Down | O => events.push(GameEvent::Button {
                                    button_state,
                                    button_value: ButtonValue::Down,
                                }),
                                A | Left => events.push(GameEvent::Button {
                                    button_state,
                                    button_value: ButtonValue::Left,
                                }),
                                D | Right | E => events.push(GameEvent::Button {
                                    button_state,
                                    button_value: ButtonValue::Right,
                                }),
                                Escape => events.push(GameEvent::Quit),
                                Space | Delete => events.push(GameEvent::Button {
                                    button_state,
                                    button_value: ButtonValue::Attack,
                                }),
                                _ => (),
                            }
                        }
                    }
                    glutin::WindowEvent::MouseInput { state, button, .. } => {
                        if button == glium::glutin::MouseButton::Left {
                            let button_state = match state {
                                ElementState::Pressed => ButtonState::Pressed,
                                ElementState::Released => ButtonState::Released,
                            };
                            events.push(GameEvent::Button {
                                button_state,
                                button_value: ButtonValue::Attack,
                            });
                        }
                    }
                    _ => (),
                }
            }
        });
        events
    }
}
