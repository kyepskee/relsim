use crate::tensors::Tensor1;

pub struct AstroObj {
    pub schwarzschild_radius: f64, //someone spell check this
    pub position: Tensor1,
    pub radius: f64,
}

pub const OBJS: [AstroObj; 2] = [
    AstroObj {
        schwarzschild_radius: 0.001f64,
        position: Tensor1 {
            vals: [0f64, -3f64, 0f64, 0f64],
        },
        radius: 0.8f64,
    },
    AstroObj {
        schwarzschild_radius: 0.001f64,
        position: Tensor1 {
            vals: [0f64, 3f64, 0f64, 0f64],
        },
        radius: 0.3f64,
    },
];
