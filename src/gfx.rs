extern crate glium;
use glium::Surface;
use glium::glutin::{self, Event, ElementState};
use std::f64::consts::PI;
use std;

#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 2],
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    x : f32,
    y : f32,
}

pub type Angle = f32;

implement_vertex!(Vertex, position);

pub fn angle_between(pos : Position, target_pos : Position) -> Angle {
    (pos.x - target_pos.x).atan2(pos.y - target_pos.y)
}

fn create_circle_vertices(radius : f64, num_vertices : usize) -> Vec<Vertex> {
    let mut v = Vec::<Vertex>::with_capacity(num_vertices+1);
    for x in 0..=num_vertices {
        let inner : f64 = 2.0 * PI / num_vertices as f64 * x as f64;
        v.push(Vertex { position: [(inner.cos()*radius) as f32, (inner.sin()*radius) as f32] });
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
//

pub struct Color (f32, f32, f32);

pub struct Circle {
    pos : Position,
    direction : Angle,
    color : Color,
}


pub struct Display {
    events_loop : glutin::EventsLoop,
    display : glium::Display,
    vertex_buffer : glium::vertex::VertexBuffer<Vertex>,
    program : glium::Program,
    pos : Position,
    mouse_pos : Position,
    horiz_axis : f32,
    vert_axis : f32,
    screen_to_opengl : Box<FnMut((f64, f64)) -> Position>,
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


        //let vertex1 = Vertex { position: [-0.5, -0.5] };
        //let vertex2 = Vertex { position: [ 0.0,  0.5] };
        //let vertex3 = Vertex { position: [ 0.5, -0.25] };
        //let shape = vec![vertex1, vertex2, vertex3];
        let shape = create_circle_vertices(0.2, 30);

        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();


        let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        out vec2 my_attr;

        uniform mat4 matrix;

        void main() {
            my_attr = position;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
        "#;

        let fragment_shader_src = r#"
            #version 140

            in vec2 my_attr;
            out vec4 color;

            void main() {
                color = vec4(my_attr, 0.3, 1.0);
            }
        "#;

        let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
        Self {
            events_loop,
            display,
            vertex_buffer,
            program,
            pos : Position { x: 0.0, y: 0.0 },
            mouse_pos : Position { x: 0.0, y: 0.0 },
            horiz_axis : 0.0,
            vert_axis : 0.0,
            screen_to_opengl,
        }
    }

    pub fn draw(&self) {
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);
        let angle = angle_between(self.pos, self.mouse_pos);


        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [angle.cos() as f32, -angle.sin() as f32, 0.0, 0.0],
                [angle.sin() as f32, angle.cos() as f32, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [self.pos.x, self.pos.y, 0.0, 1.0f32],
            ]
        };
        target.draw(&self.vertex_buffer, &indices, &self.program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
    }

    pub fn update(self : &mut Self) {

        let movement_speed : f32 = 0.002;
        // Handle all events
        let mut events = Vec::new();
        self.events_loop.poll_events(|ev| { events.push(ev) });
        for ev in events {
            if let Event::WindowEvent {event, ..} = ev {
                match event {
                    // Time to close the app?
                    glutin::WindowEvent::Closed => std::process::exit(0), //closed = true,
                    // Mouse moved
                    glutin::WindowEvent::CursorMoved { device_id : _, position, modifiers : _ } => {
                        self.mouse_pos = (self.screen_to_opengl)(position);
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

        // Modify position
        self.pos.x += movement_speed * self.horiz_axis;
        self.pos.y += movement_speed * self.vert_axis;
    }
}



