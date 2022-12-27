use std::ops;
use rand::Rng;



#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub e: [f64; 3]
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2]
        }  
    }

    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 {
            e: [rng.gen::<f64>()*rng.gen::<f64>(), rng.gen::<f64>()*rng.gen::<f64>(), rng.gen::<f64>()*rng.gen::<f64>()]
        }
    }

    pub fn length(self) -> f64 {
        (self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]).sqrt()
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
       *v / v.length()
    }

    pub fn x(self) -> f64 {
        self.e[0]
    }

    pub fn y(self) -> f64 {
        self.e[1]
    }

    pub fn z(self) -> f64 {
        self.e[2]
    }

    pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
        a.e[0] * b.e[0] + a.e[1] * b.e[1] + a.e[2] * b.e[2]
    }

    pub fn squared_length(self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3 {
            e: [a.y() * b.z() - a.z() * b.y(),
            a.z() * b.x() - a.x() * b.z(),
            a.x() * b.y() - a.y() * b.x()]
        }
    }
    

    /// Converts floats to ints, colors are
    /// supposed to be integer values between 0 and 255.
    pub fn colorize(&mut self) -> () {
        self.e[0] = self.e[0].floor();
        self.e[1] = self.e[1].floor();
        self.e[2] = self.e[2].floor();
    }
}


impl std::ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            e: [rhs.e[0] * self, rhs.e[1] * self, rhs.e[2] * self],
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let k = 1.0 / rhs;

        Vec3 {
            e: [self.e[0] * k, self.e[1] * k, self.e[2] * k],
        }
    }
}

impl ops::AddAssign for Vec3 {

    fn add_assign(&mut self, rhs: Vec3) {
        self.e = [self.e[0] + rhs.e[0], self.e[1] + rhs.e[1], self.e[2] + rhs.e[2]]
    }
}