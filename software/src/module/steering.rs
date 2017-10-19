/// Steering Wheel
pub struct SteeringWheel {}
impl SteeringWheel {
    pub fn set(&self, _steering: &Steering) {}
}

pub enum Steering {
    Center,
    Left,
    Right,
}
