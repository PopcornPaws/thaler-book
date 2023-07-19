mod ext_prover;
mod prover;
mod utils;

pub use ext_prover::Prover as ExtProver;
pub use prover::Prover;

use ark_poly::polynomial::multivariate::SparseTerm;
use bls::Fr;

pub type MultiExt = ark_poly::SparseMultilinearExtension<Fr>;
pub type MultiPoly = ark_poly::polynomial::multivariate::SparsePolynomial<Fr, SparseTerm>;
pub type UniPoly = ark_poly::polynomial::univariate::SparsePolynomial<Fr>;
