#[macro_use]
extern crate glium;
use glium::Surface;

mod teapot;

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new()
        .with_title(String::from("LearnOpenGL"))
        .with_dimensions(600, 600)
        .with_depth_buffer(24)
        .build_glium()
        .expect("Failed to build glium window");

    let vs = r#"
        #version 330

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;

        uniform mat4 matrix;

        // Called once for each vertex in our geometry
        void main() {
            v_normal = transpose(inverse(mat3(matrix))) * normal;
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

    let fs = r#"
        #version 330

        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;

        // Called once per pixel
        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
        }
    "#;

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display,
                                          glium::index::PrimitiveType::TrianglesList,
                                          &teapot::INDICES)
        .unwrap();
    let program = glium::Program::from_source(&display, vs, fs, None)
        .expect("Failed to link program");

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let light = [-1.0, 0.4, 0.8f32];

    let uniforms = uniform! {
            matrix: [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
            u_light: light,
        };

    loop {
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        target.draw((&positions, &normals),
                  &indices,
                  &program,
                  &uniforms,
                  &params)
            .expect("Failed to draw");

        target.finish().expect("Failed to clear target");

        for event in display.poll_events() {
            match event {
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Escape)) => return,
                glium::glutin::Event::Closed => return,
                _ => (),
            }
        }
    }
}
