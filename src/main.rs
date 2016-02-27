#[macro_use]
extern crate glium;

#[path = "teapot.rs"]
mod teapot;

fn main() {
    use glium::{DisplayBuild, Surface};
    let display = glium::glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .build_glium().unwrap();
    
    let mut t: f32 = 0.0;

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display,
                                          glium::index::PrimitiveType::TrianglesList,
                                          &teapot::INDICES).unwrap();

    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;
        
        uniform mat4 rotation;
        uniform mat4 scaling;
        uniform mat4 perspective;

        void main() {
            mat4 matrix = rotation * scaling;
            v_normal = transpose(inverse(mat3(matrix))) * normal;
            gl_Position = perspective * matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        
        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

    let program = glium::Program::from_source(&display,
                                              vertex_shader_src,
                                              fragment_shader_src,
                                              None).unwrap();

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };

    loop {

        t += 0.004;
        if t > 2.0 * std::f32::consts::PI {
            t = 0.0;
        }

        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        let perspective = {
            let (width, height) = target.get_dimensions();
            let aspect_ratio = height as f32 / width as f32;

            let fov: f32 = std::f32::consts::PI / 3.0;
            let far = 1024.0;
            let near = 0.1;

            let f = 1.0 / (fov / 2.0).tan();

            [
                [f * aspect_ratio, 0.0,                       0.0, 0.0],
                [             0.0,  f ,                       0.0, 0.0],
                [             0.0, 0.0,     (far+near)/(far-near), 1.0],
                [             0.0, 0.0,-(2.0*far*near)/(far-near), 0.0],
            ]
        };

        let uniforms = uniform! {
            rotation: [
                [ t.cos(), t.sin(), 0.0 , 0.0],
                [-t.sin(), t.cos(), 0.0 , 0.0],
                [    0.0 ,    0.0 , 1.0 , 0.0],
                [    0.0 ,    0.0 , 0.0 , 1.0f32],
            ],
            scaling: [
                [    0.01,    0.0 , 0.0 , 0.0],
                [    0.0 ,    0.01, 0.0 , 0.0],
                [    0.0 ,    0.0 , 0.01, 0.0],
                [    0.0 ,    0.0 , 2.0 , 1.0f32],
            ],
            u_light: [-1.0, 0.4, 0.9f32],
            perspective: perspective,
        };


        target.draw((&positions, &normals), &indices, &program, &uniforms,
                    &params).unwrap();
        target.finish().unwrap();

        for event in display.poll_events() {
            match event {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
