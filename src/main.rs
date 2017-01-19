#[macro_use]

extern crate glium;
extern crate glutin;

use glium::Surface;
use glium::DisplayBuild;
use glium::glutin::Event;
use glium::glutin::ElementState;
//use glium::glutin::VirtualKeyCode;
use std::time::Instant;

#[derive(Copy, Clone)] // Some random thing to define the struct to do stuff? i think?
struct Vertex {
  position: [f32; 2],
}

implement_vertex!(Vertex, position);

struct Object {
  position: [f32; 2],
  colour: [f32; 3],
  width: f32,
  height: f32,
  velocity: [f32; 2],
}

impl Object {
  pub fn new() -> Object {
    Object { position: [0.0, 0.0], colour: [0.509, 0.098, 0.819], width: 0.5, height: 0.5, velocity: [0.1,  0.1] }
  }
  
  pub fn get_velocity(&self) -> [f32; 2] {
    self.velocity
  }
  
  pub fn get_position(&self) -> [f32; 2] {
    self.position
  }
  
  pub fn get_colour(&self) -> [f32; 3] {
    self.colour
  }
  
  pub fn get_width(&self) -> f32 {
    self.width
  }
  
  pub fn get_height(&self) -> f32 {
    self.height
  }
  
  pub fn change_x(&mut self, diff: f32) {
    self.position[0] += diff;
  }

  pub fn change_y(&mut self, diff: f32) {
    self.position[1] += diff;
  }
}

fn square(x: f32, y: f32, width: f32, height: f32) -> Vec<Vertex> {
  let w = width/2.0;
  let h = height/2.0;
  vec![Vertex { position: [x-w, y-h] }, Vertex { position: [ x+w,  y-h] }, Vertex { position: [ x+w, y+h] },
       Vertex { position: [x-w, y-h] }, Vertex { position: [ x-w,  y+h] }, Vertex { position: [ x+w, y+h] } ]
}

fn triangle() -> Vec<Vertex> {
  vec![Vertex { position: [-0.25, -0.25] }, Vertex { position: [ 0.25,  -0.25] }, Vertex { position: [ 0.25, 0.25] }]
}

fn keyboard(key_pressed: [bool; 255], dt: f32,  truck: &mut Object) {  
  let w = 25;
  let a = 38;
  let s = 39;
  let d = 40;
  //let esc = 9;
  
  if key_pressed[w] == true {
    let vely = truck.get_velocity()[1] *dt as f32;
    truck.change_y(vely);
  }
  
  if key_pressed[a] == true {
    let velx = truck.get_velocity()[0] *dt as f32;
    truck.change_x(-velx);
  }
  
  if key_pressed[s] == true {
    let vely = truck.get_velocity()[1] *dt as f32;
    truck.change_y(-vely);
  }
  
  if key_pressed[d] == true {
    let velx = truck.get_velocity()[0] *dt as f32;
    truck.change_x(velx);
  }
}

// That useful drawing function
fn draw(display: &glium::backend::glutin_backend::GlutinFacade, shaders: &glium::Program, truck: &Object) {
  let mut render = display.draw();
  
  // Set render background colour
  render.clear_color(0.0, 0.0, 1.0, 1.0);
  
  let vertex_buffer = glium::VertexBuffer::new(display, &square(truck.get_position()[0], truck.get_position()[1], truck.get_width(), truck.get_height())).unwrap();
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
      color = vec4(0.509, 0.098, 0.819, 1.0);
    }
  "#;
  
  let display = glium::glutin::WindowBuilder::new().with_dimensions(800, 600).
                                                    with_title(format!("TrucksTrucksandMoreTrucks")).
                                                    build_glium().unwrap();
  let shaders = glium::Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();
  
  let mut key_pressed: [bool; 255] = [false; 255];
  
  let mut truck = Object::new();// { name: "Truck", position: [0.0, 0.0], colour: [0.509, 0.098, 0.819], width: 0.5, height: 0.5, velocity: [0.0,  0.0] };
  
  //start timer for delta time
  let mut last_time = Instant::now();
  
  loop {
    //get elapsed time!
    let delta_time = last_time.elapsed().subsec_nanos() as f32 / 1000000000.0;
    last_time = Instant::now();
    
    // Them helpful poll things ya know
    for ev in display.poll_events() {
      match ev {
        Event::KeyboardInput(ElementState::Pressed, scan_code, _) => {
          key_pressed[scan_code as usize] = true;
          println!("P: {}", scan_code);
        },
        Event::KeyboardInput(ElementState::Released, scan_code, _) => {
          key_pressed[scan_code as usize] = false;
          println!("R: {}", scan_code);
        },
        glium::glutin::Event::Closed => return, _ => (),      
      }
    }
    
    keyboard(key_pressed, delta_time, &mut truck);
    
    // Draw them things on the render
    draw(&display, &shaders, &truck);

  }
}
