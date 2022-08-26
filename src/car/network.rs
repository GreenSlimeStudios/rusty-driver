use macroquad::prelude::*;
// use utils::math::sigmoid;

#[derive(Clone, Debug)]
pub struct Network {
    layers: Vec<Layer>,
}
impl Network {
    pub fn new(layer_count: i8, layer_neuron_count: i8, input_neuron_count: i8) -> Self {
        let mut network = Self { layers: Vec::new() };
        for i in 0..layer_count {
            if i == layer_count - 1 {
                network.layers.push(Layer::new(4)); // 4 becouse there are for buttons the car can press
            } else {
                network.layers.push(Layer::new(layer_neuron_count));
            }
        }
        for i in 0..network.layers.len() {
            let input_len = if i == 0 {
                input_neuron_count
            } else {
                network.layers[i - 1].neurons.len() as i8
            };
            network.layers[i].generate_new_weights(input_len);
        }
        network
    }
}
#[derive(Clone, Debug)]
struct Layer {
    weights: Vec<Vec<f32>>,
    neurons: Vec<Neouron>,
}
impl Layer {
    fn new(neuron_count: i8) -> Self {
        let mut layer = Self {
            weights: Vec::new(),
            neurons: Vec::new(),
        };
        for i in 0..neuron_count {
            layer.neurons.push(Neouron::new());
        }
        layer
    }
    fn update_neurons(&mut self, input_neurons: &Vec<Neouron>) {
        for i in 0..self.neurons.len() {
            self.neurons[i].calculate(&self.weights[i], input_neurons);
        }
    }
    fn generate_new_weights(&mut self, in_neuron_count: i8) {
        for i in 0..self.neurons.len() {
            self.weights.push(Vec::new());
            for j in 0..in_neuron_count {
                self.weights[i].push(0.5);
            }
        }
    }
}
#[derive(Clone, Debug)]
pub struct Neouron {
    // inputs: Vec,
    value: f32,
}
impl Neouron {
    fn new() -> Self {
        Self { value: 0.5 }
    }
    pub fn from_val(value: f32) -> Self {
        Self { value }
    }
    fn calculate(&mut self, weights: &Vec<f32>, input_neurons: &Vec<Neouron>) {
        let mut val: f32 = 0.0;
        for i in 0..input_neurons.len() {
            val += input_neurons[i].value * weights[i];
        }
        self.value = val;
        // self.value = sigmoid(val);
    }
}
