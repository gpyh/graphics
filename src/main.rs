#[macro_use]
extern crate glium;

mod teapot;

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();
    
    let mut t: f32 = 0.0;

    #[derive(Copy, Clone)]
    struct Vertex {
        v: [f32; 3],
    }

    implement_vertex!(Vertex, v);
    let v1 = Vertex { v: [-0.5 , -0.5 ,  0.0 ] };
    let v2 = Vertex { v: [ 0.0 ,  0.5 ,  0.0 ] };
    let v3 = Vertex { v: [ 0.5 , -0.25,  0.0 ] };
    let shape = vec![v1, v2, v3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::
        NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec3 v;
        out vec3 pos;
        uniform mat4 matrix;
        void main() {
            pos = v;
            gl_Position = matrix * vec4(v, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec3 pos;
        out vec4 color;
        void main() {
            color = vec4(pos, 1.0);
        }
    "#;

    let program = glium::Program:: from_source(&display,
                                               vertex_shader_src,
                                               fragment_shader_src, None)
        .unwrap();


    loop {

        t += 0.004;
        if t > 2.0 * std::f32::consts::PI {
            t = 0.0;
        }

        let uniforms = uniform! {
            matrix: [
                [ t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [    0.0 ,    0.0 , 1.0, 0.0],
                [    0.0 ,    0.0 , 0.0, 1.0f32],
            ]
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program,
                    &uniforms,
                    &Default::default())
            .unwrap();
        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
