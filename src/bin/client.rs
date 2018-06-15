extern crate rusty_sword_arena;
#[macro_use]
extern crate glium;
use glium::{glutin, Surface};
use glium::glutin::{Event, ElementState, VirtualKeyCode};

use rusty_sword_arena as rsa;
use rsa::{GameControlMsg, GameSettings};
use rsa::net::{ServerConnection};
use std::f64::consts::PI;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}


fn create_circle_vertices(radius : f64, num_vertices : usize) -> Vec<Vertex> {
    let mut v = Vec::<Vertex>::with_capacity(num_vertices+1);
    for x in 0..=num_vertices {
        let inner : f64 = 2.0 * PI / num_vertices as f64 * x as f64;
        v.push(Vertex { position: [(inner.cos()*radius) as f32, (inner.sin()*radius) as f32] });
    }
    v
}

fn main() {

    let mut server_conn = ServerConnection::new("localhost");

    let msg = GameControlMsg::Join {name : "bob".to_string()};
    if let Ok(game_settings) = server_conn.game_control(msg) {
        println!("Got game settings! {:?}", game_settings);
    }


    // network code commented out and stashed in lib.rs for now
    use glium::glutin;

    let mut events_loop = glutin::EventsLoop::new();
    let window_width = 1024;
    let window_height = 1024;
    let window = glutin::WindowBuilder::new()
        .with_dimensions(window_width, window_height)
        .with_title("Rusty Sword Arena!");
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    //let shape = vec![vertex1, vertex2, vertex3];
    let shape = create_circle_vertices(0.2, 30);

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleFan);

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

    let mut ox : f32 = 0.0;
    let mut oy : f32 = 0.0;

    let mut closed = false;
    let mut mousex : f32 = 0.0;
    let mut mousey : f32 = 0.0;
    let movement_speed : f32 = 0.002;
    let mut horiz_axis : f32 = 0.0;
    let mut vert_axis : f32 = 0.0;
    while !closed {
        // Poll events
        events_loop.poll_events(|ev| {
            match ev {
                Event::WindowEvent {event, ..} => match event {
                    // Time to close the app?
                    glutin::WindowEvent::Closed => closed = true,
                    // Mouse moved
                    glutin::WindowEvent::CursorMoved { device_id, position, modifiers } => {
                        mousex = ((position.0 / window_width as f64) - 1.0) as f32;
                        mousey = (1.0 - (position.1 / window_height as f64)) as f32;
                    },
                    // Keyboard button
                    glutin::WindowEvent::KeyboardInput { device_id, input } => {
                        match input {
                            glium::glutin::KeyboardInput { scancode, state, virtual_keycode, modifiers } => {
                                let amount : f32;
                                match state {
                                    ElementState::Pressed => { amount = 1.0 },
                                    ElementState::Released => { amount = 0.0 },
                                }
                                use glutin::VirtualKeyCode::*;
                                if let Some(vkey) = virtual_keycode {
                                    match vkey {
                                        W | Up | Comma => { vert_axis  = amount },
                                        S | Down | O   => { vert_axis  = -amount },
                                        A | Left       => { horiz_axis = -amount },
                                        D | Right | E  => { horiz_axis = amount },
                                        _ => (),
                                    }
                                }
                            },
                            _ => (),
                        }
                    },
                    _ => (),
                },
                _ => (),
            }
        });

        // Modify movement
        ox += movement_speed * horiz_axis;
        oy += movement_speed * vert_axis;

        fn angle_between(pos_x : f32, pos_y : f32, target_x : f32, target_y : f32) -> f32 {
            (pos_x - target_x).atan2(pos_y - target_y)
        }


        let x = ox - (mousex as f32);
        let y = oy - (mousey as f32);

        //let angle = x.atan2(y);
        let angle = angle_between(ox, oy, mousex, mousey);


        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [angle.cos() as f32, -angle.sin() as f32, 0.0, 0.0],
                [angle.sin() as f32, angle.cos() as f32, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [ox, oy, 0.0, 1.0f32],
            ]
        };
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        // Handle network events

    }
}
