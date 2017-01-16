#[macro_use]

extern crate glium;
use glium::Surface;

// That useful drawing function
fn draw(mut screen: glium::Frame) {
  // Set screen background colour
  screen.clear_color(0.0, 0.0, 1.0, 1.0);
  
  // Draw screeb
  screen.finish().unwrap();
}

fn main() {
  println!("Hello, world!");
  
  use glium::DisplayBuild;
  
  let display = glium::glutin::WindowBuilder::new().with_dimensions(800, 600).
                                                    with_title(format!("TrucksTrucksandMoreTrucks")).
                                                    build_glium().unwrap();

  loop {
    let mut screen = display.draw();
  
    // Them helpful poll things ya know
    for ev in display.poll_events() {
      match ev {
        glium::glutin::Event::Closed => return, _ => ()
      }
    }
    
    // Draw them things on the screen
    draw(screen);
  }
}
