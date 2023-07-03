#[cfg(test)]
mod test {
    use ark_poly::{DenseMultilinearExtension, MultilinearExtension};
    use bls::Fr;

    #[test]
    fn multilinear_extension() {
        let num_vars = 2_usize;
        let evaluations = vec![Fr::from(1), Fr::from(2), Fr::from(1), Fr::from(4)];

        let extension_poly = DenseMultilinearExtension::from_evaluations_vec(num_vars, evaluations);

        assert_eq!(extension_poly.evaluate(&[Fr::from(0), Fr::from(0)]), Some(Fr::from(1)));
        assert_eq!(extension_poly.evaluate(&[Fr::from(1), Fr::from(0)]), Some(Fr::from(2)));
        assert_eq!(extension_poly.evaluate(&[Fr::from(0), Fr::from(1)]), Some(Fr::from(1)));
        assert_eq!(extension_poly.evaluate(&[Fr::from(1), Fr::from(1)]), Some(Fr::from(4)));

        assert_eq!(extension_poly.evaluate(&[Fr::from(2), Fr::from(0)]), Some(Fr::from(3)));
        assert_eq!(extension_poly.evaluate(&[Fr::from(3), Fr::from(0)]), Some(Fr::from(4)));
        
    }
}
