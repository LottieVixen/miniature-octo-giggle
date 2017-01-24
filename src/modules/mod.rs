pub struct Object {
  name: String,
  position: [f32; 2],
  colour: (f32, f32, f32),
  width: f32,
  height: f32,
  velocity: [f32; 2],
}

pub struct Wheel {
  angle: f32,
  max_angle: f32,
}

pub struct Truck {
  info: Object,

  acceleration: f32,
  speed: f32,
  max_speed: f32,
  
  front_wheel: Wheel,
  back_wheel: Wheel,
}

impl Object {
  pub fn new(name:String) -> Object {
    Object { name: String::from(name), position: [0.0, 0.0], colour: (0.509, 0.098, 0.819), width: 0.5, height: 0.5, velocity: [0.1,  0.1] }
  }
  
  pub fn clone(&mut self, copy: &Object) {
    self.name = String::from(copy.get_name());
    self.position = copy.get_position();
    self.colour = copy.get_colour();
    self.width = copy.get_width();
    self.height = copy.get_height();
    self.velocity = copy.get_velocity();
  }
  
  pub fn get_name(&self) -> &str {
    &self.name
  }
  
  pub fn get_velocity(&self) -> [f32; 2] {
    self.velocity
  }
  
  pub fn get_position(&self) -> [f32; 2] {
    self.position
  }
  
  pub fn get_colour(&self) -> (f32, f32, f32) {
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
  
  pub fn add_x(&mut self, diff: f32) {
    self.position[0] += diff;
  }

  pub fn add_y(&mut self, diff: f32) {
    self.position[1] += diff;
  }
}

impl Wheel {
  pub fn new(angle: f32, max_angle: f32) -> Wheel {
    Wheel { angle: angle, max_angle: max_angle }
  }
}

impl Truck {
  pub fn new() -> Truck {
    Truck { info: Object::new(String::from("Truck")), acceleration: 0.2, speed: 0.0, max_speed: 4.0, front_wheel: Wheel::new(0.0, 45.0), back_wheel: Wheel::new(0.0, 0.0) }
  }
  
  pub fn get_object(&self) -> &Object {
    &self.info
  }
  
  pub fn get_front_wheel(&self) -> &Wheel {
    &self.front_wheel
  }
  
  pub fn get_back_wheel(&self) -> &Wheel {
    &self.back_wheel
  }
  
  pub fn get_velocity(&self) -> [f32; 2] {
    self.info.velocity
  }
  
  pub fn get_position(&self) -> [f32; 2] {
    self.info.position
  }
  
  pub fn get_colour(&self) -> (f32, f32, f32) {
    self.info.colour
  }
  
  pub fn get_width(&self) -> f32 {
    self.info.width
  }
  
  pub fn get_height(&self) -> f32 {
    self.info.height
  }
  
  pub fn add_x(&mut self, diff: f32) {
    self.info.add_x(diff);
  }

  pub fn add_y(&mut self, diff: f32) {
    self.info.add_y(diff);
  }
  
}
