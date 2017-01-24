#[macro_use]

extern crate glium;
extern crate glutin;

mod modules;

use modules::Object;
use std::time::Instant;
use glium::Surface;
use glium::DisplayBuild;
use glium::glutin::Event;
use glium::glutin::ElementState;
//use glium::glutin::VirtualKeyCode;

#[derive(Copy, Clone)] // Some random thing to define the struct to do stuff? i think?
struct Vertex {
  position: [f32; 2],
}

implement_vertex!(Vertex, position);



fn square(x: f32, y: f32, width: f32, height: f32) -> Vec<Vertex> {
  let w = width/2.0;
  let h = height/2.0;
  vec![Vertex { position: [x-w, y-h] }, Vertex { position: [ x+w,  y-h] }, Vertex { position: [ x+w, y+h] },
       Vertex { position: [x-w, y-h] }, Vertex { position: [ x-w,  y+h] }, Vertex { position: [ x+w, y+h] } ]
}

fn triangle() -> Vec<Vertex> {
  vec![Vertex { position: [-0.25, -0.25] }, Vertex { position: [ 0.25,  -0.25] }, Vertex { position: [ 0.25, 0.25] }]
}

fn keyboard(key_pressed: [bool; 255], dt: f32, objects: &mut [Object; 1]) {  
  let w = 25;
  let a = 38;
  let s = 39;
  let d = 40;
  //let esc = 9;
  
  if key_pressed[w] == true {
    let vely = objects[0].get_velocity()[1] *dt as f32;
    objects[0].change_y(vely);
  }
  
  if key_pressed[a] == true {
    let velx = objects[0].get_velocity()[0] *dt as f32;
    objects[0].change_x(-velx);
  }
  
  if key_pressed[s] == true {
    let vely = objects[0].get_velocity()[1] *dt as f32;
    objects[0].change_y(-vely);
  }
  
  if key_pressed[d] == true {
    let velx = objects[0].get_velocity()[0] *dt as f32;
    objects[0].change_x(velx);
  }
}

// That useful drawing function
fn draw(display: &glium::backend::glutin_backend::GlutinFacade, shaders: &glium::Program, objects: &[Object; 1]) {
  let mut render = display.draw();
  
  // Set render background colour
  render.clear_color(0.0, 0.0, 1.0, 1.0);
  
  let vertex_buffer = glium::VertexBuffer::new(display, &square(objects[0].get_position()[0], objects[0].get_position()[1], objects[0].get_width(), objects[0].get_height())).unwrap();
  let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
  
  render.draw(&vertex_buffer, &indices, &shaders, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
  
  // Draw screeb
  render.finish().unwrap();
}

fn main() {
  println!("Hello, world!");
  
  let display = glium::glutin::WindowBuilder::new().with_dimensions(1024, 768).
                                                    with_title(format!("TrucksTrucksandMoreTrucks")).
                                                    build_glium().unwrap();
  let shaders = glium::Program::from_source(&display, include_str!("shader.vs"), include_str!("shader.frag"), None).unwrap();
  
  let mut key_pressed: [bool; 255] = [false; 255];
  
  let mut objects: [Object; 1] = [Object::new()];
  
  objects[0].set_name(String::from("Truck"));
  
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
    
    keyboard(key_pressed, delta_time, &mut objects);
    
    // Draw them things on the render
    draw(&display, &shaders, &objects);

  }
}
