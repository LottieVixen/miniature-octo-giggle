pub struct Object {
  name: String,
  position: [f32; 2],
  colour: (f32, f32, f32),
  width: f32,
  height: f32,
}

pub struct Wheel {
  info: Object,
  angle: f32,
  max_angle: f32,
}

pub struct Truck {
  info: Object,

  velocity: [f32; 2],
  acceleration: f32,
  speed: f32,
  max_speed: f32,
  max_reverse_speed: f32,
  
  front_wheel: Wheel,
  back_wheel: Wheel,
}

impl Object {
  pub fn new(name:String) -> Object {
    Object { name: String::from(name), position: [0.0, 0.0], colour: (0.509, 0.098, 0.819), width: 0.5, height: 0.5 }
  }
  
  pub fn clone(&mut self, copy: &Object) {
    self.name = String::from(copy.get_name());
    self.position = copy.get_position();
    self.colour = copy.get_colour();
    self.width = copy.get_width();
    self.height = copy.get_height();
  }
  
  pub fn get_name(&self) -> &str {
    &self.name
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
    Wheel { info: Object::new(String::from("Wheel")), angle: angle, max_angle: max_angle }
  }
  
  pub fn rotate(&mut self, angle: f32) {
    self.angle += angle;
    if self.angle > self.max_angle {
      self.angle = self.max_angle; 
    } else if self.angle < -self.max_angle {
      self.angle = -self.max_angle;
    }
    
  }
}

impl Truck {
  pub fn new() -> Truck {
    Truck { info: Object::new(String::from("Truck")), velocity: [0.1,  0.1], acceleration: 0.0, speed: 0.0, max_speed: 0.0005, max_reverse_speed: -0.0001, front_wheel: Wheel::new(0.0, 45.0), back_wheel: Wheel::new(0.0, 0.0) }
  }
  
  pub fn get_object(&self) -> &Object {
    &self.info
  }
  
  pub fn get_velocity(&self) -> [f32; 2] {
    self.velocity
  }
  
  pub fn get_front_wheel(&self) -> &Wheel {
    &self.front_wheel
  }
  
  pub fn get_back_wheel(&self) -> &Wheel {
    &self.back_wheel
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
  
  pub fn update(&mut self, delta_time: f32) {
    let friction = 0.00001;
    if self.acceleration != 0.0 {
      self.speed += self.acceleration*delta_time;
      if self.speed > self.max_speed {
        self.speed = self.max_speed;
      } else if self.speed < self.max_reverse_speed {
        self.speed = self.max_reverse_speed;
      }
    } else {
      if self.speed > 0.0 {
        self.speed -= friction*delta_time;
        if self.speed < 0.0 {
         self.speed = 0.0;
        }
      } else if self.speed < 0.0 {
        self.speed += friction*delta_time;
        if self.speed > 0.0 {
          self.speed = 0.0;
        }
      }
    }
    
    self.acceleration = 0.0;
    self.info.add_y(self.speed);
  }
  
  pub fn accelerate(&mut self, delta_time: f32) {
    self.acceleration += 0.02*delta_time;
  }
  
  pub fn decelerate(&mut self, delta_time: f32) {
    self.acceleration -= 0.02*delta_time;
  }
  
  pub fn rotate_front_wheel(&mut self, angle: f32) {
    &self.front_wheel.rotate(angle);
  }
  
  pub fn rotate_back_wheel(&mut self, angle: f32) {
    &self.back_wheel.rotate(angle);
  }
}
