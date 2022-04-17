// Max depth of the Alpha beta search
#[derive(Debug, Clone)]
pub struct Depth(pub u8);

impl Depth {
    pub fn add(&mut self) {
        if self.0 + 1 <= 12 {
            self.0 += 1;
        }
    }

    pub fn sub(&mut self) {
        if self.0 - 1 >= 2 {
            self.0 -= 1;
        }
    }
}

impl Default for Depth {
    fn default() -> Self {
        Self(6)
    }
}

impl ToString for Depth {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
