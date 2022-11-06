use ark_ff::{One, Zero};
use ark_poly::polynomial::multivariate;
use ark_poly::polynomial::multivariate::SparseTerm as MultiTerm;
use ark_poly::polynomial::multivariate::Term;
use ark_poly::polynomial::univariate;
use ark_poly::{MVPolynomial, Polynomial};
use bls::Fr;
use intbits::Bits;

type MultiPoly = multivariate::SparsePolynomial<Fr, MultiTerm>;
type UniPoly = univariate::SparsePolynomial<Fr>;

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

    fn init_poly_sum(&self) -> Fr {
        let binary_inputs = binary_inputs(self.poly.num_vars() as u8);
        binary_inputs
            .iter()
            .fold(Fr::zero(), |acc, x| acc + self.poly.evaluate(x))
    }

    fn next_unipoly(&self) -> UniPoly {
        let mut unipoly = UniPoly::from_coefficients_vec(vec![]);
        let twos_exponent = self.poly.num_vars() - self.r_vec.len() - 1;
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

    fn evaluate(&self) -> Fr {
        self.poly.evaluate(&self.r_vec)
    }
}

fn binary_inputs(size: u8) -> Vec<Vec<Fr>> {
    (0..2_u32.pow(u32::from(size)))
        .map(|i| (0..size).map(|j| Fr::from(i.bit(j))).collect::<Vec<Fr>>())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_prover() {
        let poly = MultiPoly::from_coefficients_vec(
            3,
            vec![
                (Fr::from(2), MultiTerm::new(vec![(0, 3)])),
                (Fr::from(1), MultiTerm::new(vec![(0, 1), (2, 1)])),
                (Fr::from(1), MultiTerm::new(vec![(1, 1), (2, 1)])),
            ],
        );
        let mut prover = Prover::new(poly);

        assert_eq!(Fr::from(12), prover.init_poly_sum());

        let unipoly = prover.next_unipoly();

        assert_eq!(Fr::one(), unipoly.evaluate(&Fr::zero()));
        assert_eq!(Fr::from(11), unipoly.evaluate(&Fr::one()));
        assert_eq!(Fr::from(69), unipoly.evaluate(&Fr::from(2)));

        prover.r_vec.push(Fr::from(2));

        let unipoly = prover.next_unipoly();
        assert_eq!(Fr::from(34), unipoly.evaluate(&Fr::zero()));
        assert_eq!(Fr::from(35), unipoly.evaluate(&Fr::one()));

        prover.r_vec.push(Fr::from(3));

        let unipoly = prover.next_unipoly();
        assert_eq!(Fr::from(16), unipoly.evaluate(&Fr::zero()));
        assert_eq!(Fr::from(21), unipoly.evaluate(&Fr::one()));

        prover.r_vec.push(Fr::from(6));
        assert_eq!(Fr::from(46), prover.evaluate());
    }
}
