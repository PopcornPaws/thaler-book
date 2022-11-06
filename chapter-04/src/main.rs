use ark_poly::polynomial::multivariate::SparsePolynomial as MultiPoly;
use bls::Fr;

struct Prover {
	poly: MultiPoly,
	r_vec: Vec<Fr>,
}

impl Prover {
	fn new(poly: MultiPoly) -> Self {
		Self {
			poly,
			r_vec: Vec::new(),
		}
	}
}

fn main() {

}
