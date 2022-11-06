use ark_ff::{One, Zero};
use ark_poly::polynomial::multivariate;
use ark_poly::polynomial::multivariate::SparseTerm as MultiTerm;
use ark_poly::polynomial::multivariate::Term;
use ark_poly::polynomial::univariate;
use ark_poly::{MVPolynomial, Polynomial, UVPolynomial};
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
}

fn main() {
    let poly = MultiPoly::from_coefficients_vec(
        3,
        vec![
            (Fr::from(2), MultiTerm::new(vec![(2, 3)])),
            (Fr::from(1), MultiTerm::new(vec![(2, 1), (0, 1)])),
            (Fr::from(1), MultiTerm::new(vec![(1, 1), (0, 1)])),
        ],
    );

    assert_eq!(Fr::from(12), sum_poly(&poly));

    let next = 2;
    let mut unipoly = UniPoly::from_coefficients_vec(vec![]);
    for i in 0..2u32.pow(next as u32) {
        for (coeff, term) in &poly.terms {
            let mut power: Option<usize> = None;
            let mut prod = *coeff;
            for (pow, var) in term.powers().iter().zip(term.vars()) {
                if var == next {
                    power = Some(*pow);
                } else if i.bit(var) {
                    prod *= Fr::one(); // TODO this will be r_vec[i]
                } else {
                    prod = Fr::zero();
                    break; // don't even multiply the rest
                }
            }
            let pow = power.unwrap_or_default();
            unipoly += &UniPoly::from_coefficients_vec(vec![(pow, prod)]);
        }
    }

    assert_eq!(Fr::one(), unipoly.evaluate(&Fr::zero()));
    assert_eq!(Fr::from(11), unipoly.evaluate(&Fr::one()));
    assert_eq!(Fr::from(69), unipoly.evaluate(&Fr::from(2)));
}

fn sum_poly(poly: &MultiPoly) -> Fr {
    let binary_inputs = binary_inputs(poly.num_vars() as u8);
    binary_inputs
        .iter()
        .fold(Fr::zero(), |acc, x| acc + poly.evaluate(x))
}

fn binary_inputs(size: u8) -> Vec<Vec<Fr>> {
    (0..2_u32.pow(u32::from(size)))
        .map(|i| (0..size).map(|j| Fr::from(i.bit(j))).collect::<Vec<Fr>>())
        .collect()
}
