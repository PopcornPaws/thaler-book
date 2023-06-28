use ark_ff::Zero;
use ark_poly::Polynomial;
use bls::Fr;
use intbits::Bits;

/// Generates permutations of inputs depending on the number of arguments (size).
fn binary_inputs(size: u8) -> Vec<Vec<Fr>> {
    (0..2_u32.pow(u32::from(size)))
        .map(|i| (0..size).map(|j| Fr::from(i.bit(j))).collect::<Vec<Fr>>())
        .collect()
}

/// Computes the initial sum of evaluations over each binary input permutation.
pub fn sum_binary_evals(polynomial: &crate::MultiPoly) -> Fr {
    let inputs = binary_inputs(polynomial.num_vars as u8);
    inputs
        .iter()
        .fold(Fr::zero(), |acc, x| acc + polynomial.evaluate(x))
}

#[cfg(test)]
mod test {
    use super::*;
    use ark_ff::{One, Zero};
    use ark_poly::polynomial::multivariate::{SparseTerm, Term};
    use ark_poly::DenseMVPolynomial;

    #[test]
    fn binary_inputs_works() {
        assert_eq!(binary_inputs(0), vec![Vec::<Fr>::new()]);
        assert_eq!(binary_inputs(1), vec![vec![Fr::zero()], vec![Fr::one()]]);
        assert_eq!(
            binary_inputs(2),
            vec![
                vec![Fr::zero(), Fr::zero()],
                vec![Fr::one(), Fr::zero()],
                vec![Fr::zero(), Fr::one()],
                vec![Fr::one(), Fr::one()],
            ]
        );

        assert_eq!(
            binary_inputs(3),
            vec![
                vec![Fr::zero(), Fr::zero(), Fr::zero()], // 0: 000 big endian!
                vec![Fr::one(), Fr::zero(), Fr::zero()],  // 1: 100
                vec![Fr::zero(), Fr::one(), Fr::zero()],  // 2: 010
                vec![Fr::one(), Fr::one(), Fr::zero()],   // 3: 110
                vec![Fr::zero(), Fr::zero(), Fr::one()],  // 4: 001
                vec![Fr::one(), Fr::zero(), Fr::one()],   // 5: 101
                vec![Fr::zero(), Fr::one(), Fr::one()],   // 6: 011
                vec![Fr::one(), Fr::one(), Fr::one()],    // 7: 111
            ]
        );
    }

    #[test]
    fn sum_binary_evals_works() {
        // g(x,y) = 2x + xy + y
        let poly = crate::MultiPoly::from_coefficients_vec(
            2,
            vec![
                (Fr::from(2), SparseTerm::new(vec![(0, 1)])),
                (Fr::from(1), SparseTerm::new(vec![(0, 1), (1, 1)])),
                (Fr::from(1), SparseTerm::new(vec![(1, 1)])),
            ],
        );
        // g(0,0) = 0
        // g(1,0) = 2
        // g(0,1) = 1
        // g(1,1) = 4
        // sum = 0 + 2 + 1 + 4 = 7
        assert_eq!(sum_binary_evals(&poly), Fr::from(7));
    }
}
