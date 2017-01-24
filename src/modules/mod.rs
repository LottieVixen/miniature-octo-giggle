pub struct Object {
  name: String,
  position: [f32; 2],
  colour: [f32; 3],
  width: f32,
  height: f32,
  velocity: [f32; 2],
}

impl Object {
  pub fn new() -> Object {
    Object { name: String::from("Type"), position: [0.0, 0.0], colour: [0.509, 0.098, 0.819], width: 0.5, height: 0.5, velocity: [0.1,  0.1] }
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
  
  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }
  
  pub fn change_x(&mut self, diff: f32) {
    self.position[0] += diff;
  }

  pub fn change_y(&mut self, diff: f32) {
    self.position[1] += diff;
  }
}