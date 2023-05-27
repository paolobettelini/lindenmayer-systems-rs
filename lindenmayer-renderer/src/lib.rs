pub use lindenmayer_engine::*;
pub use meval;
pub use rand_pcg::Pcg64;
use meval::*;
use std::{rc::Rc, cell::RefCell};

use std::collections::HashMap;

pub mod canvas;
pub(crate) mod expressions;

#[derive(Debug)]
pub enum Operation {
    Forward(Expr),
    Jump(Expr),
    Dot(Expr),
    Rotate(Expr),
    Thickness(Expr),
    Ignore(Expr),
    PushStack,
    PopStack,
    SetColor((f64, f64, f64, f64)),
    SetVar(String, Expr),
}

#[derive(Debug)]
pub struct LSystemRenderer {
    pub lsystem: LSystem,
    pub iter: usize,
    pub initial_pos: (f64, f64),
    pub initial_rot: f64,
    pub initial_thickness: f64,
    pub background_color: (f64, f64, f64, f64),
    pub initial_color: (f64, f64, f64, f64),
    pub canvas: (i32, i32),
    pub seed: String,
    pub injections: Vec<(u32, String)>,
    pub variables: HashMap<String, f64>,
    pub operations: HashMap<char, Vec<Operation>>,
    // Cache for re-use
    pub expression: String,
    pub rng: Rc<RefCell<Pcg64>>,
}

impl Default for LSystemRenderer {
    fn default() -> Self {
        let axiom = String::new();
        let rules = [];
        let lsystem = LSystem::new(&axiom, &rules);
        let iter = 1;
        let initial_pos = (375f64, 560f64);
        let initial_rot = 0f64;
        let initial_thickness = 1f64;
        let background_color = (1f64, 1f64, 1f64, 1f64);
        let initial_color = (0f64, 0f64, 0f64, 1f64);
        let canvas = (750, 750);
        let seed = "Default Seed".to_string();
        let injections = vec![];
        let variables = HashMap::new();
        let operations = HashMap::new();

        let expression = String::from(""); // Nothing to render
        let rng = expressions::get_rng(&seed);
        let rng = Rc::new(RefCell::new(rng));

        LSystemRenderer {
            lsystem,
            iter,
            initial_pos,
            initial_rot,
            initial_thickness,
            background_color,
            initial_color,
            canvas,
            seed,
            injections,
            variables,
            operations,
            expression,
            rng,
        }
    }
}

impl LSystemRenderer {
    pub fn new(
        lsystem: LSystem,
        iter: usize,
        initial_pos: (f64, f64),
        initial_rot: f64,
        initial_thickness: f64,
        background_color: (f64, f64, f64, f64),
        initial_color: (f64, f64, f64, f64),
        canvas: (i32, i32),
        seed: String,
        injections: Vec<(u32, String)>,
        variables: HashMap<String, f64>,
        operations: HashMap<char, Vec<Operation>>,
    ) -> Self {
        let expression = lsystem.expand(iter);
        let rng = expressions::get_rng(&seed);
        let rng = Rc::new(RefCell::new(rng));

        Self {
            lsystem,
            iter,
            initial_pos,
            initial_rot,
            initial_thickness,
            background_color,
            initial_color,
            canvas,
            seed,
            injections,
            variables,
            operations,
            expression,
            rng,
        }
    }

    pub fn update_expr(&mut self) {
        let expression = self.lsystem.expand(self.iter);

        // Calculate the total length of all injections
        let total_injection_length: usize = self
            .injections
            .iter()
            .filter(|injection| injection.0 <= expression.len() as u32)
            .map(|injection| injection.1.len())
            .sum();

        // Create a buffer with enough capacity to hold the final injected expression
        let mut injected_expression = String::with_capacity(expression.len() + total_injection_length);

        // Iterate over the expression and injections simultaneously
        let mut expr_index = 0;
        let mut injection_iter = self.injections.iter().filter(|injection| injection.0 <= expression.len() as u32);

        while let Some((injection_index, injection_value)) = injection_iter.next() {
            // Append the characters from the expression until the injection index
            injected_expression.push_str(&expression[expr_index..(*injection_index as usize)]);

            // Append the injection value
            injected_expression.push_str(injection_value);

            // Update the expression index
            expr_index = *injection_index as usize;
        }

        // Append the remaining characters from the expression
        injected_expression.push_str(&expression[expr_index..]);

        self.expression = injected_expression;
    }

    pub fn update_rng(&mut self) {
        *self.rng.borrow_mut() = expressions::get_rng(&self.seed);
    }
}