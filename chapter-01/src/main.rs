use ark_bls12_381 as bls;

use ark_ff::{Field, One, Zero};
use bls::fq::Fq;
use nalgebra::{SMatrix as Matrix, SVector as Vector};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

const N: usize = 5;
fn main() {
    let mut rng = StdRng::from_seed([3; 32]);
    let a_mat = Matrix::<Fq, N, N>::from_fn(|_, _| Fq::from(rng.gen::<u128>()));
    let b_mat = Matrix::<Fq, N, N>::from_fn(|_, _| Fq::from(rng.gen::<u128>()));
    // valid c_matrix
    let c_mat = a_mat * b_mat;

    let mut random_bytes = [0u8; N];
    rng.fill(&mut random_bytes);
    let x = Fq::from_random_bytes(&random_bytes).expect("failed to generate random field element");
    let mut test_vec = vec![Fq::one(); N];
    // this generates 1, x, x^2, ... x^{N - 1}
    for i in 1..N {
        test_vec[i] = x * test_vec[i - 1];
    }

    let test_vector = Vector::<Fq, N>::from_vec(test_vec);
    let zero_vector = Vector::<Fq, N>::from_fn(|_, _| Fq::zero());

    let bt = b_mat * test_vector;
    let ct = c_mat * test_vector;
    assert_eq!(a_mat * bt - ct, zero_vector);

    // invalid c_matrix
    let c_mat = Matrix::<Fq, N, N>::from_fn(|_, _| Fq::from(rng.gen::<u128>()));
    let ct = c_mat * test_vector;
    assert_ne!(a_mat * bt - ct, zero_vector);
}
