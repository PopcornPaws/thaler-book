use bls::Fr;

pub type GateDescription = ([usize; 2], Op);
pub type LayerDescription = Vec<GateDescription>;
pub type CircuitDescription = Vec<LayerDescription>;

pub enum Gate<'a> {
    Input(Fr),
    Op { inputs: [&'a Gate<'a>; 2], op: Op },
}

pub enum Op {
    Add,
    Mul,
}

// each layer should have a multilinear extension that's a log2(S_i)-variate polynomial
pub struct Layer<'a>(Vec<Gate<'a>>);

pub struct Circuit<'a>(Vec<Layer<'a>>);

impl<'a> Gate<'a> {
    pub fn eval(&self) -> Fr {
        match self {
            Self::Input(scalar) => *scalar,
            Self::Op { inputs: [a, b], op } => match op {
                Op::Add => a.eval() + b.eval(),
                Op::Mul => a.eval() * b.eval(),
            },
        }
    }

    pub fn new_input(scalar: Fr) -> Self {
        Self::Input(scalar)
    }

    pub fn new_mid(a: &'a Gate, b: &'a Gate, op: Op) -> Self {
        Self::Op { inputs: [a, b], op }
    }
}

impl<'a> Layer<'a> {
    pub fn new_input(scalars: Vec<Fr>) -> Self {
        Self(scalars.into_iter().map(Gate::new_input).collect())
    }

    // unwraps are fine because we are initializing, and if the representation
    // is invalid, we don't want to proceed.
    pub fn new_mid(desc: LayerDescription, layer: &'a Self) -> Self {
        Self(
            desc.into_iter()
                .map(|([i, j], op)| {
                    Gate::new_mid(layer.0.get(i).unwrap(), layer.0.get(j).unwrap(), op)
                })
                .collect(),
        )
    }

    pub fn eval(&self) -> Vec<Fr> {
        self.0.iter().map(Gate::eval).collect()
    }
}

impl<'a> Circuit<'a> {
    pub fn new(inputs: Vec<Fr>) -> Self {
        Self(vec![Layer::new_input(inputs)])
    }

    pub fn add_layers(&mut self, desc: CircuitDescription) {
        for (i, layer_desc) in desc.into_iter().enumerate() {
            let previous_layer = self.0.get(i).unwrap();
            self.0.push(Layer::new_mid(layer_desc, previous_layer))
        }
    }

    pub fn eval(&self) -> Vec<Vec<Fr>> {
        self.0.iter().map(Layer::eval).collect()
    }
}
