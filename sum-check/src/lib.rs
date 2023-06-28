mod prover;
mod utils;

pub use prover::Prover;

use ark_poly::polynomial::multivariate::SparseTerm;
use bls::Fr;

pub type MultiPoly = ark_poly::polynomial::multivariate::SparsePolynomial<Fr, SparseTerm>;
pub type UniPoly = ark_poly::polynomial::univariate::SparsePolynomial<Fr>;
