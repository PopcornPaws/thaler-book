use ark_bls12_381 as bls;
use nalgebra as na;

use ark_ff::{One, Zero};
use bls::fq::Fq;
use na::{SMatrix as Matrix, SVector as Vector};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

const N: usize = 2;
fn main() {
    let mut rng = StdRng::from_seed([3; 32]);
    let a_mat = Matrix::<Fq, N, N>::from_fn(|_, _| Fq::from(rng.gen::<u128>()));
    let b_mat = Matrix::<Fq, N, N>::from_fn(|_, _| Fq::from(rng.gen::<u128>()));
    // valid c_matrix
    let c_mat = a_mat * b_mat;

    let test_vector = Vector::<Fq, N>::from_fn(|c, _| if c % 2 == 0 { Fq::one() } else { Fq::zero() });
    let zero_vector = Vector::<Fq, N>::from_fn(|_, _| Fq::zero());

    let bt = b_mat * test_vector;
    let ct = c_mat * test_vector;
    assert_eq!(a_mat * bt - ct, zero_vector);

    // invalid c_matrix
    let c_mat = Matrix::<Fq, N, N>::from_fn(|_, _| Fq::from(rng.gen::<u128>()));
    let ct = c_mat * test_vector;
    assert_ne!(a_mat * bt - ct, zero_vector);
}
