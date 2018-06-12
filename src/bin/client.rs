extern crate rusty_sword_arena;
#[macro_use]
extern crate glium;
use glium::{glutin, Surface};

use rusty_sword_arena as rsa;
use rsa::{GameControlMsg, GameSettings};
use rsa::net::{ServerConnection};

fn main() {

    let mut server_conn = ServerConnection::new("localhost");

    let msg = GameControlMsg::Join {name : "bob".to_string()};
    if let Ok(game_settings) = server_conn.game_control(msg) {
        println!("Got game settings! {:?}", game_settings);
    }


    // network code commented out and stashed in lib.rs for now
    use glium::glutin;

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

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

    let mut t: f32 = -0.5;
    let mut closed = false;
    while !closed {
        // we update `t`
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [t,   0.0, 0.0, 1.0f32],
            ]
        };
        target.draw(&vertex_buffer, &indices, &program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();

        // Handle network events

        // Time to close the app?
        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent {event, ..} => match event {
                    glutin::WindowEvent::Closed => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
