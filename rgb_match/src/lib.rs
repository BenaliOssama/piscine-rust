#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
    if self.r == first {
        if self.g == second {
            let temp = self.r;
            self.r = self.g;
            self.g = temp;
        } else if self.b == second {
            let temp = self.r;
            self.r = self.b;
            self.b = temp;
        } else if self.a == second {
            let temp = self.r;
            self.r = self.a;
            self.a = temp;
        }
    } else if self.g == first {
        if self.r == second {
            let temp = self.g;
            self.g = self.r;
            self.r = temp;
        } else if self.b == second {
            let temp = self.g;
            self.g = self.b;
            self.b = temp;
        } else if self.a == second {
            let temp = self.g;
            self.g = self.a;
            self.a = temp;
        }
    } else if self.b == first {
        if self.r == second {
            let temp = self.b;
            self.b = self.r;
            self.r = temp;
        } else if self.g == second {
            let temp = self.b;
            self.b = self.g;
            self.g = temp;
        } else if self.a == second {
            let temp = self.b;
            self.b = self.a;
            self.a = temp;
        }
    } else if self.a == first {
        if self.r == second {
            let temp = self.a;
            self.a = self.r;
            self.r = temp;
        } else if self.g == second {
            let temp = self.a;
            self.a = self.g;
            self.g = temp;
        } else if self.b == second {
            let temp = self.a;
            self.a = self.b;
            self.b = temp;
        }
    }

    self
}
}
