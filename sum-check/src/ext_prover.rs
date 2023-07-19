use crate::MultiExt;
use ark_poly::{DenseMVPolynomial, MultilinearExtension, Polynomial};
use bls::Fr;

pub struct Prover {
    poly: MultiExt,
    challenges: Vec<Fr>,
}


