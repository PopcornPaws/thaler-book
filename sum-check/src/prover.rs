use crate::{MultiPoly, UniPoly};

use ark_ff::{One, Zero};
use ark_poly::polynomial::multivariate::Term;
use ark_poly::Polynomial;
use bls::Fr;

pub struct Prover {
    /// Original polynomial known to the prover
    poly: MultiPoly,
    /// Random challenges sent by the verifier
    r_vec: Vec<Fr>,
}

impl Prover {
    pub fn new(poly: MultiPoly) -> Self {
        Self {
            poly,
            r_vec: Vec::new(),
        }
    }

    pub fn init_poly_sum(&self) -> Fr {
        crate::utils::sum_binary_evals(&self.poly)
    }

    pub fn next_unipoly(&self) -> UniPoly {
        let mut unipoly = UniPoly::from_coefficients_vec(vec![]);
        let twos_exponent = self.poly.num_vars - self.r_vec.len() - 1;
        for i in 0..2usize.pow(twos_exponent as u32) {
            for (coeff, term) in &self.poly.terms {
                let mut power: Option<usize> = None;
                let mut prod = *coeff;
                for (pow, var) in term.powers().iter().zip(term.vars()) {
                    if var == self.r_vec.len() {
                        power = Some(*pow);
                    } else if let Some(rand) = self.r_vec.get(var) {
                        let tmp_poly = UniPoly::from_coefficients_vec(vec![(*pow, Fr::one())]);
                        let eval = tmp_poly.evaluate(rand);
                        prod *= eval;
                    } else if (i & (1 << (var - self.r_vec.len() - 1))) == 0 {
                        prod = Fr::zero();
                        break; // don't even multiply the rest
                    }
                }
                let pow = power.unwrap_or_default();
                unipoly += &UniPoly::from_coefficients_vec(vec![(pow, prod)]);
            }
        }
        unipoly
    }

    pub fn evaluate(&self) -> Fr {
        self.poly.evaluate(&self.r_vec)
    }

    pub fn register_challenge(&mut self, challenge: Fr) {
        self.r_vec.push(challenge);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ark_poly::DenseMVPolynomial;
    use ark_poly::polynomial::multivariate::SparseTerm;

    #[test]
    fn simple_prover() {
        // 3-variate polynomial
        // g(x, y, z) = 2x³+ xz + yz
        // x - 0 input index
        // y - 1 input index
        // z - 2 input index
        let poly = MultiPoly::from_coefficients_vec(
            3,
            vec![
                (Fr::from(2), SparseTerm::new(vec![(0, 3)])),
                (Fr::from(1), SparseTerm::new(vec![(0, 1), (2, 1)])),
                (Fr::from(1), SparseTerm::new(vec![(1, 1), (2, 1)])),
            ],
        );
        let mut prover = Prover::new(poly);

        assert_eq!(Fr::from(12), prover.init_poly_sum());

        let unipoly = prover.next_unipoly();

        assert_eq!(Fr::one(), unipoly.evaluate(&Fr::zero()));
        assert_eq!(Fr::from(11), unipoly.evaluate(&Fr::one()));
        assert_eq!(Fr::from(69), unipoly.evaluate(&Fr::from(2)));

        prover.register_challenge(Fr::from(2));

        let unipoly = prover.next_unipoly();
        assert_eq!(Fr::from(34), unipoly.evaluate(&Fr::zero()));
        assert_eq!(Fr::from(35), unipoly.evaluate(&Fr::one()));

        prover.register_challenge(Fr::from(3));

        let unipoly = prover.next_unipoly();
        assert_eq!(Fr::from(16), unipoly.evaluate(&Fr::zero()));
        assert_eq!(Fr::from(21), unipoly.evaluate(&Fr::one()));

        prover.register_challenge(Fr::from(6));
        assert_eq!(Fr::from(46), prover.evaluate());
    }
}
