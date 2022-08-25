#[derive(Clone)]
pub struct Controlls {
    pub left: bool,
    pub right: bool,
    pub forward: bool,
    pub reverse: bool,
    pub active: bool,
}

impl Controlls {
    pub fn new(is_main_car: bool) -> Self {
        Self {
            active: is_main_car,
            left: false,
            right: false,
            forward: !is_main_car,
            reverse: false,
        }
    }
}
