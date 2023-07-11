#[cfg(test)]
mod test {
    use ark_poly::polynomial::multivariate::{SparsePolynomial, SparseTerm, Term};
    use ark_poly::{
        DenseMVPolynomial, DenseMultilinearExtension, MultilinearExtension, Polynomial,
    };
    use bls::Fr;

    #[test]
    fn multilinear_extension() {
        let num_vars = 2_usize;
        let evaluations = vec![Fr::from(1), Fr::from(2), Fr::from(1), Fr::from(4)];

        let extension_poly = DenseMultilinearExtension::from_evaluations_vec(num_vars, evaluations);

        // columns, rows
        assert_eq!(
            extension_poly.evaluate(&[Fr::from(0), Fr::from(0)]),
            Some(Fr::from(1))
        );
        assert_eq!(
            extension_poly.evaluate(&[Fr::from(1), Fr::from(0)]),
            Some(Fr::from(2))
        );
        assert_eq!(
            extension_poly.evaluate(&[Fr::from(0), Fr::from(1)]),
            Some(Fr::from(1))
        );
        assert_eq!(
            extension_poly.evaluate(&[Fr::from(1), Fr::from(1)]),
            Some(Fr::from(4))
        );

        assert_eq!(
            extension_poly.evaluate(&[Fr::from(2), Fr::from(0)]),
            Some(Fr::from(3))
        );
        assert_eq!(
            extension_poly.evaluate(&[Fr::from(3), Fr::from(0)]),
            Some(Fr::from(4))
        );
    }

    #[test]
    fn another_multilinear_extension() {
        let num_vars = 2_usize;
        // NOTE MUST be in LITTLE ENDIAN FORM
        // 0 = 00 => w(0,0) = 3
        // 1 = 10 => w(1,0) = 2
        // 2 = 01 => w(0,1) = 3
        // 3 = 11 => w(1,1) = 1
        let evaluations = vec![Fr::from(3), Fr::from(3), Fr::from(2), Fr::from(1)];
        let extension_poly = DenseMultilinearExtension::from_evaluations_vec(num_vars, evaluations);

        // g(x, y) = 3(1 - x)(1 - y) + 2(1 - x)y + 3x(1 - y) + xy
        // g(x, y) = 3 - y - xy
        let minus_one = Fr::from(0) - Fr::from(1);
        let poly = SparsePolynomial::<Fr, SparseTerm>::from_coefficients_vec(
            2,
            vec![
                (Fr::from(3), SparseTerm::new(vec![(0, 0)])),
                (minus_one, SparseTerm::new(vec![(1, 1)])),
                (minus_one, SparseTerm::new(vec![(0, 1), (1, 1)])),
            ],
        );

        assert_eq!(poly.evaluate(&vec![Fr::from(0), Fr::from(0)]), Fr::from(3));
        assert_eq!(poly.evaluate(&vec![Fr::from(0), Fr::from(1)]), Fr::from(2));
        assert_eq!(poly.evaluate(&vec![Fr::from(1), Fr::from(0)]), Fr::from(3));
        assert_eq!(poly.evaluate(&vec![Fr::from(1), Fr::from(1)]), Fr::from(1));
        assert_eq!(
            poly.evaluate(&vec![Fr::from(2), Fr::from(3)]),
            Fr::from(0) - Fr::from(6)
        );

        // poly and it's extension representation should evaluate to the same value
        let input = vec![Fr::from(1), Fr::from(2)];
        assert_eq!(poly.evaluate(&input), extension_poly.evaluate(&input).unwrap());
        let input = vec![Fr::from(2), Fr::from(1)];
        assert_eq!(poly.evaluate(&input), extension_poly.evaluate(&input).unwrap());
        let input = vec![Fr::from(9), Fr::from(111)];
        assert_eq!(poly.evaluate(&input), extension_poly.evaluate(&input).unwrap());
    }
}
