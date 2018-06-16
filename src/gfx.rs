extern crate glium;
use glium::Surface;
use glium::glutin::{self, Event, ElementState};
use std::f64::consts::PI;
use std;

use super::{Color, Angle};

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

#[derive(Copy, Clone, Debug)]
pub struct Position {
    x : f32,
    y : f32,
}

impl Position {
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}


pub fn angle_between(pos : Position, target_pos : Position) -> Angle {
    (pos.x - target_pos.x).atan2(pos.y - target_pos.y)
}

fn create_circle_vertices(radius : f64, num_vertices : usize, color : Color) -> Vec<Vertex> {
    let mut v = Vec::<Vertex>::with_capacity(num_vertices+1);
    for x in 0..=num_vertices {
        let inner : f64 = 2.0 * PI / num_vertices as f64 * x as f64;
        v.push(Vertex {
            position: [(inner.cos()*radius) as f32, (inner.sin()*radius) as f32],
            color: [color.r, color.g, color.b],
        });
    }
    v
}

//pub struct GlThingy<I, U>
//    where I : glium::index::Index,
//          U : glium::uniforms::Uniforms {
//    vertex_buffer : glium::VertexBuffer,
//    index : I,
//    program : glium::Program,
//    uniforms : U,
//}

pub struct Shape {
    pos : Position,
    direction : Angle,
    vertex_buffer : glium::vertex::VertexBuffer<Vertex>,
}

impl Shape {
    pub fn new_circle(display : &glium::Display, radius : f64, pos : Position, direction : Angle, color : Color) -> Self {
        let vertex_buffer = glium::VertexBuffer::new(display, &create_circle_vertices(radius, 30, color)).unwrap();
        Self {
            pos,
            direction,
            vertex_buffer,
        }
    }
}


pub struct Display {
    events_loop : glutin::EventsLoop,
    display : glium::Display,
    program : glium::Program,
    horiz_axis : f32,
    vert_axis : f32,
    screen_to_opengl : Box<FnMut((f64, f64)) -> Position>,
    shapes : Vec<Shape>,
}


impl Display {
    pub fn new(width : u32, height : u32) -> Self {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_dimensions(width, height)
            .with_title("Rusty Sword Arena!");
        let context = glutin::ContextBuilder::new();
        let display = glium::Display::new(window, context, &events_loop).unwrap();

        // Create a closure that captures the hidpi_factor to do local screen coordinate conversion
        // for us.
        let hidpi_factor = display.gl_window().window().hidpi_factor();
        let screen_to_opengl = Box::new(move |screen_coord : (f64, f64)| -> Position {
            let x = (screen_coord.0 as f32 / (0.5 * hidpi_factor * width as f32)) - 1.0;
            let y = 1.0 - (screen_coord.1 as f32 / (0.5 * hidpi_factor * height as f32));
            Position { x, y }
        });

        let shapes = vec![
            Shape::new_circle(&display, 0.2, Position::new(), 0.0, Color { r : 0.1, g : 0.2, b : 1.0 }),
            Shape::new_circle(&display, 0.2, Position { x : 0.5, y : 0.5 }, 0.0, Color { r : 1.0, g : 0.1, b : 0.1 }),
        ];

        let vertex_shader_src = r#"
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

        let fragment_shader_src = r#"
            #version 140

            in vec3 v_color;
            out vec4 color;

            void main() {
                color = vec4(v_color, 1.0);
            }
        "#;

        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
        Self {
            events_loop,
            display,
            program,
            horiz_axis : 0.0,
            vert_axis : 0.0,
            screen_to_opengl,
            shapes,
        }
    }

    pub fn draw(&self) {
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);


        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        for shape in &self.shapes {
            let uniforms = uniform! {
                matrix: [
                    [shape.direction.cos() as f32, -shape.direction.sin() as f32, 0.0, 0.0],
                    [shape.direction.sin() as f32, shape.direction.cos() as f32, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [shape.pos.x, shape.pos.y, 0.0, 1.0f32],
                ]
            };
            target.draw(&shape.vertex_buffer, &indices, &self.program, &uniforms,
                        &Default::default()).unwrap();
        }
        target.finish().unwrap();
    }

    pub fn update(self : &mut Self) {
        // Handle all events
        let mut events = Vec::new();
        self.events_loop.poll_events(|ev| { events.push(ev) });
        let mut mouse_pos_opt : Option<Position> = None;
        for ev in events {
            if let Event::WindowEvent {event, ..} = ev {
                match event {
                    // Time to close the app?
                    glutin::WindowEvent::Closed => std::process::exit(0), //closed = true,
                    // Mouse moved
                    glutin::WindowEvent::CursorMoved { device_id : _, position, modifiers : _ } => {
                        mouse_pos_opt = Some((self.screen_to_opengl)(position));
                    },
                    // Keyboard button
                    glutin::WindowEvent::KeyboardInput { device_id : _, input } => {
                        let amount : f32;
                        match input.state {
                            ElementState::Pressed => { amount = 1.0 },
                            ElementState::Released => { amount = 0.0 },
                        }
                        use glium::glutin::VirtualKeyCode::*;
                        if let Some(vkey) = input.virtual_keycode {
                            match vkey {
                                W | Up | Comma => { self.vert_axis  = amount },
                                S | Down | O   => { self.vert_axis  = -amount },
                                A | Left       => { self.horiz_axis = -amount },
                                D | Right | E  => { self.horiz_axis = amount },
                                _ => (),
                            }
                        }
                    },
                    _ => (),
                }
            }
        }

        // Update position
        let movement_speed : f32 = 0.002;
        if self.shapes.len() > 0 {
            if let Some(mouse_pos) = mouse_pos_opt {
                self.shapes[0].direction = angle_between(self.shapes[0].pos, mouse_pos);
            }
            self.shapes[0].pos.x += movement_speed * self.horiz_axis;
            self.shapes[0].pos.y += movement_speed * self.vert_axis;
        }
    }
}



