pub struct Controlls {
    pub left: bool,
    pub right: bool,
    pub forward: bool,
    pub reverse: bool,
}

impl Controlls {
    pub fn new() -> Self {
        Self {
            left: false,
            right: false,
            forward: false,
            reverse: false,
        }
    }
}
