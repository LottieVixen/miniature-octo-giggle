#[macro_use]

extern crate glium;  //import glium
extern crate glutin; //import glutin

mod modules; //import module from modules dir

use modules::Object; //select parts from modules/libs to add to namespace directly
use modules::Truck;
use std::time::Instant;// this makes using parts of mods easier
use glium::Surface;
use glium::DisplayBuild;
use glium::glutin::Event;
use glium::glutin::ElementState;
//use glium::glutin::VirtualKeyCode;

#[derive(Copy, Clone)] // Some random thing to define the struct to do stuff? i think?
struct Vertex {        //make a struct "Vertex", define a data type! :D
  position: [f32; 2],  //2 item array with f32 type, default to 0
}

implement_vertex!(Vertex, position);



fn square(x: f32, y: f32, width: f32, height: f32) -> Vec<Vertex> {
  let w = width/2.0;
  let h = height/2.0;
  vec![Vertex { position: [x-w, y-h] }, Vertex { position: [ x+w,  y-h] },
       Vertex { position: [ x+w, y+h] }, //first 3 vecs
       Vertex { position: [x-w, y-h] }, Vertex { position: [ x-w,  y+h] }, 
       Vertex { position: [ x+w, y+h] } ] //last 3 vecs
}

fn triangle() -> Vec<Vertex> {
  vec![Vertex { position: [-0.25, -0.25] }, Vertex { position: [ 0.25,  -0.25] }, Vertex { position: [ 0.25, 0.25] }]
}

// That useful drawing function
fn draw(display: &glium::backend::glutin_backend::GlutinFacade, shaders: &glium::Program, object_info: &Vec<Object>) {
  let mut render = display.draw();
  
  // Set render background colour
  render.clear_color(0.0, 0.0, 1.0, 1.0);
  
  let vertex_buffer = glium::VertexBuffer::new(display, &square(object_info[0].get_position()[0], object_info[0].get_position()[1], object_info[0].get_width(), object_info[0].get_height())).unwrap();
  let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
  
  render.draw(&vertex_buffer, &indices, &shaders, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap();
  
  // Draw screeb
  render.finish().unwrap();
}

fn main() {
  
  let display = glium::glutin::WindowBuilder::new().with_dimensions(1024, 768).
                                                    with_title(format!("TrucksTrucksandMoreTrucks")).
                                                    build_glium().unwrap();
  let shaders = glium::Program::from_source(&display, include_str!("shader.vs"), include_str!("shader.frag"), None).unwrap();
  
  let mut key_pressed: [bool; 255] = [false; 255];
  
  let mut truck = Truck::new();
  
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
          //println!("P: {}", scan_code);
        },
        Event::KeyboardInput(ElementState::Released, scan_code, _) => {
          key_pressed[scan_code as usize] = false;
          //println!("R: {}", scan_code);
        },
        glium::glutin::Event::Closed => return, _ => (),      
      }
    }
    
    let w = 25;
    let a = 38;
    let s = 39;
    let d = 40;
    //let esc = 9;
    
    // Check Key presses
    if key_pressed[w] == true {
      truck.accelerate(delta_time);
    }
  
    if key_pressed[a] == true {
       truck.rotate_front_wheel(-2.0);
    }
   
    if key_pressed[s] == true {
      truck.decelerate(delta_time);
    }
    
    if key_pressed[d] == true {
      truck.rotate_front_wheel(2.0);
    }
    
    // Update
    truck.update(delta_time);
    
    // Get Info from Objects
    let mut object_info = Vec::new();
    object_info.push(Object::new(String::from("")));
    object_info[0].clone(truck.get_object());
    // Draw them things on the render
    draw(&display, &shaders, &object_info);

  }
}
