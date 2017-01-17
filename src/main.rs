#[macro_use]

extern crate glium;

use glium::Surface;
use glium::DisplayBuild;

#[derive(Copy, Clone)] // Some random thing to define the struct to do stuff? i think?
struct Vertex {
  position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn triangle() -> Vec<Vertex> {
  vec![Vertex { position: [-0.5, -0.5] }, Vertex { position: [ 0.0,  0.5] }, Vertex { position: [ 0.5, -0.25] }]
}

// That useful drawing function
fn draw(display: &glium::backend::glutin_backend::GlutinFacade, shaders: &glium::Program) {
  let mut render = display.draw();
  
  // Set render background colour
  render.clear_color(0.0, 0.0, 1.0, 1.0);
  
  let vertex_buffer = glium::VertexBuffer::new(display, &triangle()).unwrap();
  let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
  
  render.draw(&vertex_buffer, &indices, &shaders, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
  
  // Draw screeb
  render.finish().unwrap();
}

fn main() {
  println!("Hello, world!");
  
  // Them Shaders
  let vertex_shader = r#"
    #version 140
    
    in vec2 position;
    
    void main() {
      gl_Position = vec4(position, 0.0, 1.0);
    }
  "#;

  let fragment_shader = r#"
    #version 140
    
    out vec4 color;
    
    void main() {
      color = vec4(1.0, 0.0, 0.0, 1.0);
    }
  "#;
  
  let display = glium::glutin::WindowBuilder::new().with_dimensions(800, 600).
                                                    with_title(format!("TrucksTrucksandMoreTrucks")).
                                                    build_glium().unwrap();
  let shaders = glium::Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();
  
  loop {
    // Them helpful poll things ya know
    for ev in display.poll_events() {
      match ev {
        glium::glutin::Event::Closed => return, _ => ()
      }
    }
    
    // Draw them things on the render
    draw(&display, &shaders);
  }
}
