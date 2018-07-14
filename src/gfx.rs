extern crate glium;
use glium::glutin::{self, ElementState};
use glium::Surface;
use std::cmp::min;
use std::f64::consts::PI;

use super::game::{ButtonState, ButtonValue, Color, InputEvent, Vector2};
use glium::Frame;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

fn create_circle_vertices(radius: f32, num_vertices: usize, color: Color) -> Vec<Vertex> {
    let mut v = Vec::<Vertex>::with_capacity(num_vertices + 2);
    // The center of the circle/fan
    v.push(Vertex {
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
        v.push(Vertex {
            position: [inner.cos() as f32 * radius, inner.sin() as f32 * radius],
            color: [color.r, color.g, color.b],
        });
    }
    v
}

fn create_ring_vertices(radius: f32, num_vertices: usize, color: Color) -> Vec<Vertex> {
    let mut v = Vec::<Vertex>::with_capacity(num_vertices + 1);
    for x in 0..=num_vertices {
        let inner: f64 = 2.0 * PI / num_vertices as f64 * x as f64;
        v.push(Vertex {
            position: [inner.cos() as f32 * radius, inner.sin() as f32 * radius],
            color: [color.r, color.g, color.b],
        });
    }
    v
}

/// A `Shape` can be drawn by a [Window](gfx/struct.Window.html).  Use the provided `new_*` methods
/// to make a `Shape`.
pub struct Shape {
    pub pos: Vector2,
    pub direction: f32,
    vertex_buffer: glium::vertex::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
}

impl Shape {
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

/// An OpenGL window for displaying graphics. Also the object through which you'll receive input
/// events (mouse, keyboard, etc.)
pub struct Window {
    events_loop: glutin::EventsLoop,
    display: glium::Display,
    program: glium::Program,
    screen_to_opengl: Box<FnMut((f64, f64)) -> Vector2>,
    target: Option<Frame>,
}

impl Window {
    /// By default, this will be a square window with a dimension of `1024px` or
    /// `(monitor height - 100px)`, whichever is smaller.  You can override the dimension by
    /// providing a value for override_dimension, for example: `Some(2048)`.  Note that on hi-dpi
    /// displays (like Apple Retina Displays) 1 "pixel" is often a square of 4 hi-dpi pixels
    /// (depending on whether the monitor is set to be scaled or not).
    pub fn new(override_dimension: Option<u32>) -> Self {
        let events_loop = glutin::EventsLoop::new();
        let (_, monitor_height) = events_loop.get_primary_monitor().get_dimensions();
        let dimension: u32 = match override_dimension {
            Some(x) => x,
            None => min(monitor_height - 100, 1024),
        };
        let window = glutin::WindowBuilder::new()
            .with_dimensions(dimension, dimension)
            .with_title("Rusty Sword Arena!");
        let context = glutin::ContextBuilder::new();
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        // Create a closure that captures the hidpi_factor to do local screen coordinate conversion
        // for us.
        let hidpi_factor = display.gl_window().window().hidpi_factor();
        let screen_to_opengl = Box::new(move |screen_coord: (f64, f64)| -> Vector2 {
            let x = (screen_coord.0 as f32 / (0.5 * hidpi_factor * dimension as f32)) - 1.0;
            let y = 1.0 - (screen_coord.1 as f32 / (0.5 * hidpi_factor * dimension as f32));
            Vector2 { x, y }
        });

        let vertex_shader = r#"
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

        let fragment_shader = r#"
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
                vertex_shader: vertex_shader,
                tessellation_control_shader: None,
                tessellation_evaluation_shader: None,
                geometry_shader: None,
                fragment_shader: fragment_shader,
                transform_feedback_varyings: None,
                outputs_srgb: true,
                uses_point_size: true,
            },
        ).unwrap();

        Self {
            events_loop,
            display,
            program,
            screen_to_opengl,
            target: None,
        }
    }

    /// Call `drawstart()` when you are ready to draw a new frame. It will create the new frame and
    /// clear it to black.
    pub fn drawstart(&mut self) {
        self.target = Some(self.display.draw());
        if let Some(ref mut target) = self.target {
            target.clear_color(0.0, 0.0, 0.0, 1.0);
        }
    }

    /// Pass `draw()` every shape that you would like to draw.  After the first time they are drawn,
    /// shapes stay on the GPU and only send updated position/rotation, which is super efficient,
    /// so keep your shape objects around!  Don't recreate them every frame.  Shapes are drawn in
    /// order, so the last shape you draw will be on top.
    pub fn draw(&mut self, shape: &Shape) {
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
                    &self.program,
                    &uniforms,
                    &draw_parameters,
                )
                .unwrap();
        }
    }

    /// Call `drawfinish()` when you are ready to finalize the frame and show it.  You will need to
    /// call `drawstart()` again before you can `draw()` any shapes in a new frame.  I think this
    /// method blocks until the hardware is ready for a frame. 60fps on most displays.
    pub fn drawfinish(&mut self) {
        self.target.take().unwrap().finish().unwrap();
    }

    /// Get [input events](game/enum.InputEvent.html) that the graphics system may have seen
    /// (window, keyboard, mouse) and translate them into our own, simpler events.
    pub fn poll_input_events(&mut self) -> Vec<InputEvent> {
        let screen_to_opengl = &mut (self.screen_to_opengl);
        let mut events = Vec::<InputEvent>::new();
        self.events_loop.poll_events(|ev| {
            if let glium::glutin::Event::WindowEvent { event, .. } = ev {
                match event {
                    // Time to close the app?
                    glutin::WindowEvent::Closed => events.push(InputEvent::WindowClosed),
                    // Mouse moved
                    glutin::WindowEvent::CursorMoved {
                        device_id: _,
                        position,
                        modifiers: _,
                    } => {
                        let mouse_pos = screen_to_opengl(position);
                        events.push(InputEvent::MouseMoved {
                            position: mouse_pos,
                        });
                    }
                    // Keyboard button
                    glutin::WindowEvent::KeyboardInput {
                        device_id: _,
                        input,
                    } => {
                        let button_state = match input.state {
                            ElementState::Pressed => ButtonState::Pressed,
                            ElementState::Released => ButtonState::Released,
                        };
                        use glium::glutin::VirtualKeyCode::*;
                        if let Some(vkey) = input.virtual_keycode {
                            match vkey {
                                W | Up | Comma => events.push(InputEvent::Button {
                                    button_state,
                                    button_value: ButtonValue::Up,
                                }),
                                S | Down | O => events.push(InputEvent::Button {
                                    button_state,
                                    button_value: ButtonValue::Down,
                                }),
                                A | Left => events.push(InputEvent::Button {
                                    button_state,
                                    button_value: ButtonValue::Left,
                                }),
                                D | Right | E => events.push(InputEvent::Button {
                                    button_state,
                                    button_value: ButtonValue::Right,
                                }),
                                Escape => events.push(InputEvent::Button {
                                    button_state,
                                    button_value: ButtonValue::Quit,
                                }),
                                Space | Delete => events.push(InputEvent::Button {
                                    button_state,
                                    button_value: ButtonValue::Attack,
                                }),
                                _ => (),
                            }
                        }
                    }
                    glutin::WindowEvent::MouseInput {
                        device_id: _,
                        state,
                        button,
                        modifiers: _,
                    } => {
                        if button == glium::glutin::MouseButton::Left {
                            let button_state = match state {
                                ElementState::Pressed => ButtonState::Pressed,
                                ElementState::Released => ButtonState::Released,
                            };
                            events.push(InputEvent::Button {
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
