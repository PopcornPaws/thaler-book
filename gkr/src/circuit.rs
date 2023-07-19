use bls::Fr;
use std::rc::Rc;

pub type GateDescription = ([usize; 2], Op);
pub type LayerDescription = Vec<GateDescription>;
pub type CircuitDescription = Vec<LayerDescription>;

pub enum Op {
    Add,
    Mul,
}

pub enum Gate {
    Input(Fr),
    Op { inputs: [Rc<Gate>; 2], op: Op },
}

impl Gate {
    pub fn eval(&self) -> Fr {
        match self {
            Self::Input(scalar) => *scalar,
            Self::Op { inputs: [a, b], op } => match op {
                Op::Add => a.as_ref().eval() + b.as_ref().eval(),
                Op::Mul => a.as_ref().eval() * b.as_ref().eval(),
            },
        }
    }

    pub fn new_input(scalar: Fr) -> Self {
        Self::Input(scalar)
    }

    pub fn new_mid(a: Rc<Gate>, b: Rc<Gate>, op: Op) -> Self {
        Self::Op {
            inputs: [Rc::clone(&a), Rc::clone(&b)],
            op,
        }
    }
}

// each layer should have a multilinear extension that's a log2(S_i)-variate polynomial
pub struct Layer(Vec<Rc<Gate>>);

impl Layer {
    pub fn new_input(scalars: Vec<Fr>) -> Self {
        Self(
            scalars
                .into_iter()
                .map(|x| Rc::new(Gate::new_input(x)))
                .collect(),
        )
    }

    pub fn new_mid(desc: LayerDescription, layer: Rc<Self>) -> Self {
        Self(
            desc.into_iter()
                .map(|([i, j], op)| {
                    Rc::new(Gate::new_mid(
                        Rc::clone(&layer.as_ref().0[i]),
                        Rc::clone(&layer.as_ref().0[j]),
                        op,
                    ))
                })
                .collect(),
        )
    }

    pub fn eval(&self) -> Vec<Fr> {
        self.0.iter().map(|x| x.as_ref().eval()).collect()
    }
}

pub struct Circuit(Vec<Rc<Layer>>);

impl Circuit {
    pub fn new(inputs: Vec<Fr>, desc: CircuitDescription) -> Self {
        let mut layers = vec![Rc::new(Layer::new_input(inputs))];

        for (i, layer_desc) in desc.into_iter().enumerate() {
            let previous_layer = Rc::clone(&layers[i]);
            layers.push(Rc::new(Layer::new_mid(layer_desc, previous_layer)))
        }
        // reverse order so that the output layer has index 0, just like in the book
        layers.reverse();
        Self(layers)
    }
    pub fn eval(&self) -> Vec<Vec<Fr>> {
        self.0.iter().map(|x| x.as_ref().eval()).collect()
    }

    pub fn eval_layer(&self, index: usize) -> Vec<Fr> {
        self.0[index].as_ref().eval()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 1 addition gate (outputs the sum of squares)
    // 2 addition gates (adding a^2 + b^2 and c^2 + d^2)
    // 4 multiplication gates (squaring each input)
    // 4 input gates
    #[test]
    fn circuit_eval() {
        let desc = vec![
            vec![
                ([0, 0], Op::Mul),
                ([1, 1], Op::Mul),
                ([2, 2], Op::Mul),
                ([3, 3], Op::Mul),
            ],
            vec![([0, 1], Op::Add), ([2, 3], Op::Add)],
            vec![([0, 1], Op::Add)],
        ];
        let inputs = vec![Fr::from(0), Fr::from(1), Fr::from(2), Fr::from(3)];
        let circuit = Circuit::new(inputs.clone(), desc);

        let square_evals = vec![Fr::from(0), Fr::from(1), Fr::from(4), Fr::from(9)];
        let sum_evals = vec![Fr::from(1), Fr::from(13)];
        let outputs = vec![Fr::from(14)];

        let evals = circuit.eval();
        assert_eq!(
            evals,
            vec![
                outputs.clone(),
                sum_evals.clone(),
                square_evals.clone(),
                inputs.clone(),
            ]
        );

        assert_eq!(circuit.eval_layer(0), outputs);
        assert_eq!(circuit.eval_layer(1), sum_evals);
        assert_eq!(circuit.eval_layer(2), square_evals);
        assert_eq!(circuit.eval_layer(3), inputs);
    }
}
