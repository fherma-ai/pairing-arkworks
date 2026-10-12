//! The optimal ate pairing, through arkworks' own `Bn254`.
use ark_bn254::{Bn254, Fq, Fq12, Fq2, G1Affine, G2Affine};
use ark_ec::pairing::Pairing;
use ark_ff::{BigInt, PrimeField};

use crate::fherma::{Inputs, Outputs, Point, Tensor};

/// The `i`-th row of a limb tensor as an element of the field `F`.
fn fe<F: PrimeField<BigInt = BigInt<4>>>(data: &[u64], i: usize, limbs: usize) -> F {
    let mut words = [0u64; 4];
    words[..limbs].copy_from_slice(&data[i * limbs..(i + 1) * limbs]);
    F::from_bigint(BigInt::new(words)).expect("a field element below the modulus")
}

/// Rows `i` and `i+1` as an element of Fp2, c0 first.
fn fe2(data: &[u64], i: usize, limbs: usize) -> Fq2 {
    Fq2::new(fe::<Fq>(data, i, limbs), fe::<Fq>(data, i + 1, limbs))
}

/// Fp12 flattened the way the specification says: twelve base-field elements,
/// index `((w-degree) * 3 + (v-degree)) * 2 + (u-degree)`.
///
/// The tower is arkworks' own and the order is read off it rather than
/// rearranged, so there is nothing here to get backwards except the writing.
fn flatten(e: Fq12, limbs: usize) -> Tensor<u64> {
    let mut data = Vec::with_capacity(12 * limbs);
    for six in [e.c0, e.c1] {
        for two in [six.c0, six.c1, six.c2] {
            for one in [two.c0, two.c1] {
                data.extend_from_slice(&one.into_bigint().0[..limbs]);
            }
        }
    }
    Tensor { shape: vec![12, limbs as i64], data }
}

#[derive(Default)]
pub struct State {
    limbs: usize,
}

pub fn init(p: &Point) -> State {
    State { limbs: p.L as usize }
}

pub fn run(state: &mut State, inp: &Inputs) -> Outputs {
    let limbs = state.limbs;

    // `new_unchecked` for both: the generator takes the G1 point from a square
    // root of the curve equation and the G2 point from a multiple of the
    // standard generator, so each is in its prime-order group by construction.
    let p = G1Affine::new_unchecked(
        fe::<Fq>(&inp.px.data, 0, limbs),
        fe::<Fq>(&inp.py.data, 0, limbs),
    );
    let q = G2Affine::new_unchecked(fe2(&inp.qx.data, 0, limbs), fe2(&inp.qy.data, 0, limbs));

    Outputs { e: flatten(Bn254::pairing(p, q).0, limbs) }
}

pub fn free(state: State) {
    drop(state);
}
