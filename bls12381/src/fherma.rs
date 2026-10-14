// GENERATED from pairing/bls12381@1.0.0. Do not edit — `--update` rewrites it.
//
// The types your answer is written against, derived from the signature: one
// field per value parameter, per argument, per result. A tensor is typed,
// because the signature already settled what its elements are.
//
// Fields are named as the signature names them (`N`, not `n`), and a harness
// may have no use for one of these types; neither is worth a warning.
#![allow(non_snake_case, dead_code)]

#[derive(Debug, Clone, Default)]
pub struct Tensor<T> {
    pub shape: Vec<i64>,
    pub data: Vec<T>,          // row-major
}

impl<T> Tensor<T> {
    pub fn count(&self) -> usize {
        self.shape.iter().product::<i64>() as usize
    }
}

#[derive(Debug, Clone, Default)]
pub struct Point {
    pub L: u32,   // u32
}

#[derive(Debug, Clone, Default)]
pub struct Inputs {
    pub px: Tensor<u64>,   // tensor<L x u64>
    pub py: Tensor<u64>,   // tensor<L x u64>
    pub qx: Tensor<u64>,   // tensor<2 x L x u64>
    pub qy: Tensor<u64>,   // tensor<2 x L x u64>
}

#[derive(Debug, Clone, Default)]
pub struct Outputs {
    pub e: Tensor<u64>,   // tensor<12 x L x u64>
}
