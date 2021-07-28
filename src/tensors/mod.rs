#[macro_use]
mod macros;

use crate::consts::*;

type Tensor0 = f64;

pub struct Tensor1 {
    pub vals: [f64; DIMENSIONS],
}
impl Default for Tensor1 {
    fn default() -> Tensor1 {
        Tensor1 {
            vals: [0.0; DIMENSIONS],
        }
    }
}
#[derive(Debug)]
pub struct Tensor2 {
    pub vals: [[f64; DIMENSIONS]; DIMENSIONS],
}
impl Default for Tensor2 {
    fn default() -> Tensor2 {
        Tensor2 {
            vals: [[0.0; DIMENSIONS]; DIMENSIONS],
        }
    }
}
pub struct Tensor3 {
    pub vals: [[[f64; DIMENSIONS]; DIMENSIONS]; DIMENSIONS],
}
impl Default for Tensor3 {
    fn default() -> Tensor3 {
        Tensor3 {
            vals: [[[0.0; DIMENSIONS]; DIMENSIONS]; DIMENSIONS],
        }
    }
}
pub struct Tensor4 {
    pub vals: [[[[f64; DIMENSIONS]; DIMENSIONS]; DIMENSIONS]; DIMENSIONS],
}
impl Default for Tensor4 {
    fn default() -> Tensor4 {
        Tensor4 {
            vals: [[[[0.0; DIMENSIONS]; DIMENSIONS]; DIMENSIONS]; DIMENSIONS],
        }
    }
}

const_assert_eq!(DIMENSIONS, 4); // inverse currently functions only for 4x4 matrices
impl Tensor2 {
    pub fn inv(&self) -> Option<Tensor2> {
        let mut inv = [
            [
                // row 0:
                // 0,0:
                self.vals[1][1] * self.vals[2][2] * self.vals[3][3]
                    - self.vals[1][1] * self.vals[2][3] * self.vals[3][2]
                    - self.vals[2][1] * self.vals[1][2] * self.vals[3][3]
                    + self.vals[2][1] * self.vals[1][3] * self.vals[3][2]
                    + self.vals[3][1] * self.vals[1][2] * self.vals[2][3]
                    - self.vals[3][1] * self.vals[1][3] * self.vals[2][2],
                // 0,1:
                -self.vals[0][1] * self.vals[2][2] * self.vals[3][3]
                    + self.vals[0][1] * self.vals[2][3] * self.vals[3][2]
                    + self.vals[2][1] * self.vals[0][2] * self.vals[3][3]
                    - self.vals[2][1] * self.vals[0][3] * self.vals[3][2]
                    - self.vals[3][1] * self.vals[0][2] * self.vals[2][3]
                    + self.vals[3][1] * self.vals[0][3] * self.vals[2][2],
                // 0,2:
                self.vals[0][1] * self.vals[1][2] * self.vals[3][3]
                    - self.vals[0][1] * self.vals[1][3] * self.vals[3][2]
                    - self.vals[1][1] * self.vals[0][2] * self.vals[3][3]
                    + self.vals[1][1] * self.vals[0][3] * self.vals[3][2]
                    + self.vals[3][1] * self.vals[0][2] * self.vals[1][3]
                    - self.vals[3][1] * self.vals[0][3] * self.vals[1][2],
                // 0,3:
                -self.vals[0][1] * self.vals[1][2] * self.vals[2][3]
                    + self.vals[0][1] * self.vals[1][3] * self.vals[2][2]
                    + self.vals[1][1] * self.vals[0][2] * self.vals[2][3]
                    - self.vals[1][1] * self.vals[0][3] * self.vals[2][2]
                    - self.vals[2][1] * self.vals[0][2] * self.vals[1][3]
                    + self.vals[2][1] * self.vals[0][3] * self.vals[1][2],
            ],
            [
                // row 1
                // 1,0:
                -self.vals[1][0] * self.vals[2][2] * self.vals[3][3]
                    + self.vals[1][0] * self.vals[2][3] * self.vals[3][2]
                    + self.vals[2][0] * self.vals[1][2] * self.vals[3][3]
                    - self.vals[2][0] * self.vals[1][3] * self.vals[3][2]
                    - self.vals[3][0] * self.vals[1][2] * self.vals[2][3]
                    + self.vals[3][0] * self.vals[1][3] * self.vals[2][2],
                // 1,1:
                self.vals[0][0] * self.vals[2][2] * self.vals[3][3]
                    - self.vals[0][0] * self.vals[2][3] * self.vals[3][2]
                    - self.vals[2][0] * self.vals[0][2] * self.vals[3][3]
                    + self.vals[2][0] * self.vals[0][3] * self.vals[3][2]
                    + self.vals[3][0] * self.vals[0][2] * self.vals[2][3]
                    - self.vals[3][0] * self.vals[0][3] * self.vals[2][2],
                // 1,2:
                -self.vals[0][0] * self.vals[1][2] * self.vals[3][3]
                    + self.vals[0][0] * self.vals[1][3] * self.vals[3][2]
                    + self.vals[1][0] * self.vals[0][2] * self.vals[3][3]
                    - self.vals[1][0] * self.vals[0][3] * self.vals[3][2]
                    - self.vals[3][0] * self.vals[0][2] * self.vals[1][3]
                    + self.vals[3][0] * self.vals[0][3] * self.vals[1][2],
                // 1,3:
                self.vals[0][0] * self.vals[1][2] * self.vals[2][3]
                    - self.vals[0][0] * self.vals[1][3] * self.vals[2][2]
                    - self.vals[1][0] * self.vals[0][2] * self.vals[2][3]
                    + self.vals[1][0] * self.vals[0][3] * self.vals[2][2]
                    + self.vals[2][0] * self.vals[0][2] * self.vals[1][3]
                    - self.vals[2][0] * self.vals[0][3] * self.vals[1][2],
            ],
            [
                // row 2
                // 2,0:
                self.vals[1][0] * self.vals[2][1] * self.vals[3][3]
                    - self.vals[1][0] * self.vals[2][3] * self.vals[3][1]
                    - self.vals[2][0] * self.vals[1][1] * self.vals[3][3]
                    + self.vals[2][0] * self.vals[1][3] * self.vals[3][1]
                    + self.vals[3][0] * self.vals[1][1] * self.vals[2][3]
                    - self.vals[3][0] * self.vals[1][3] * self.vals[2][1],
                // 2,1:
                -self.vals[0][0] * self.vals[2][1] * self.vals[3][3]
                    + self.vals[0][0] * self.vals[2][3] * self.vals[3][1]
                    + self.vals[2][0] * self.vals[0][1] * self.vals[3][3]
                    - self.vals[2][0] * self.vals[0][3] * self.vals[3][1]
                    - self.vals[3][0] * self.vals[0][1] * self.vals[2][3]
                    + self.vals[3][0] * self.vals[0][3] * self.vals[2][1],
                // 2,2:
                self.vals[0][0] * self.vals[1][1] * self.vals[3][3]
                    - self.vals[0][0] * self.vals[1][3] * self.vals[3][1]
                    - self.vals[1][0] * self.vals[0][1] * self.vals[3][3]
                    + self.vals[1][0] * self.vals[0][3] * self.vals[3][1]
                    + self.vals[3][0] * self.vals[0][1] * self.vals[1][3]
                    - self.vals[3][0] * self.vals[0][3] * self.vals[1][1],
                // 2,3:
                -self.vals[0][0] * self.vals[1][1] * self.vals[2][3]
                    + self.vals[0][0] * self.vals[1][3] * self.vals[2][1]
                    + self.vals[1][0] * self.vals[0][1] * self.vals[2][3]
                    - self.vals[1][0] * self.vals[0][3] * self.vals[2][1]
                    - self.vals[2][0] * self.vals[0][1] * self.vals[1][3]
                    + self.vals[2][0] * self.vals[0][3] * self.vals[1][1],
            ],
            [
                // row 3
                // 3,0:
                -self.vals[1][0] * self.vals[2][1] * self.vals[3][2]
                    + self.vals[1][0] * self.vals[2][2] * self.vals[3][1]
                    + self.vals[2][0] * self.vals[1][1] * self.vals[3][2]
                    - self.vals[2][0] * self.vals[1][2] * self.vals[3][1]
                    - self.vals[3][0] * self.vals[1][1] * self.vals[2][2]
                    + self.vals[3][0] * self.vals[1][2] * self.vals[2][1],
                // 3,1:
                self.vals[0][0] * self.vals[2][1] * self.vals[3][2]
                    - self.vals[0][0] * self.vals[2][2] * self.vals[3][1]
                    - self.vals[2][0] * self.vals[0][1] * self.vals[3][2]
                    + self.vals[2][0] * self.vals[0][2] * self.vals[3][1]
                    + self.vals[3][0] * self.vals[0][1] * self.vals[2][2]
                    - self.vals[3][0] * self.vals[0][2] * self.vals[2][1],
                // 3,2:
                -self.vals[0][0] * self.vals[1][1] * self.vals[3][2]
                    + self.vals[0][0] * self.vals[1][2] * self.vals[3][1]
                    + self.vals[1][0] * self.vals[0][1] * self.vals[3][2]
                    - self.vals[1][0] * self.vals[0][2] * self.vals[3][1]
                    - self.vals[3][0] * self.vals[0][1] * self.vals[1][2]
                    + self.vals[3][0] * self.vals[0][2] * self.vals[1][1],
                // 3,3:
                self.vals[0][0] * self.vals[1][1] * self.vals[2][2]
                    - self.vals[0][0] * self.vals[1][2] * self.vals[2][1]
                    - self.vals[1][0] * self.vals[0][1] * self.vals[2][2]
                    + self.vals[1][0] * self.vals[0][2] * self.vals[2][1]
                    + self.vals[2][0] * self.vals[0][1] * self.vals[1][2]
                    - self.vals[2][0] * self.vals[0][2] * self.vals[1][1],
            ],
        ];

        let det = self.vals[0][0] * inv[0][0]
            + self.vals[0][1] * inv[1][0]
            + self.vals[0][2] * inv[2][0]
            + self.vals[0][3] * inv[3][0];
        if det == 0. {
            return None;
        }

        let det_inv = 1. / det;

        for row in &mut inv {
            for elem in row.iter_mut() {
                *elem *= det_inv;
            }
        }

        Some(Tensor2 { vals: inv })
    }
}
