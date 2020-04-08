#[derive(Debug)]
pub struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    #[allow(dead_code)]
    pub fn new(real: f64, imag: f64) -> Complex {
        Complex {
            real: real,
            imag: imag,
        }
    }
    pub fn clone(&self) -> Complex {
        Complex {
            real: self.real,
            imag: self.imag,
        }
    }
    #[allow(dead_code)]
    pub fn conj(&self) -> Complex {
        Complex {
            real: self.real,
            imag: -1.0 * self.imag,
        }
    }
    #[allow(dead_code)]
    pub fn magnintude(&self) -> f64 {
        self.real * self.real + self.imag * self.imag
    }
    #[allow(dead_code)]
    pub fn add(&self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
    #[allow(dead_code)]
    pub fn sub(&self, other: Complex) -> Complex {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
    #[allow(dead_code)]
    pub fn mul(&self, other: Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.imag * other.real + self.real * other.imag,
        }
    }
    #[allow(dead_code)]
    pub fn div(c1: Complex, c2: Complex) -> Complex {
        let ans = c1.mul(c2.conj());
        let c2_mag = c2.magnintude();
        Complex {
            real: ans.real / c2_mag,
            imag: ans.imag / c2_mag,
        }
    }
    #[allow(dead_code)]
    pub fn view(&self) {
        if self.imag > 0.0 {
            println!("{} + i{}", self.real, self.imag)
        } else if self.imag < 0.0 {
            println!("{} - i{}", self.real, -1.0 * self.imag)
        }
    }
}
