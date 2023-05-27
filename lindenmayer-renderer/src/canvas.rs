use crate::{LSystemRenderer, Operation};
use meval::ContextProvider;
use std::collections::HashMap;

use crate::expressions;

pub trait Canvas {
    fn move_to(&self, x: f64, y: f64);

    fn line_to(&self, x: f64, y: f64);

    fn stroke(&self);

    fn save(&self);

    fn restore(&self);

    fn set_line_width(&self, thickness: f64);

    fn rectangle(&self, x: f64, y: f64, width: f64, height: f64);

    fn set_color(&self, r: f64, g: f64, b: f64, a: f64);

    fn arc(&self, x: f64, y: f64, r: f64);

    fn fill(&self);

}

pub type ExprContext<'a> = meval::Context<'a>;

impl dyn Canvas {
    pub fn draw_fractal<'a>(
        &self,
        fractal: &LSystemRenderer,
        variables: &'a mut ExprContext,
    ) -> Result<(), meval::Error> {
        // Stack of variables
        let mut stack1: HashMap<String, Vec<f64>> = HashMap::new();
        let mut stack2 = vec![]; // (pos, rot, thickness)

        // Current variables
        let mut pos = fractal.initial_pos;
        let mut rot = fractal.initial_rot;
        let mut thickness = fractal.initial_thickness;
        let mut color = fractal.initial_color;

        let mut depth = 0;

        // Update length variable
        variables.var("LENGTH", fractal.expression.len() as f64);
        // Update depth variable
        variables.var("DEPTH", 0f64);

        // Add random function to context
        let rng = fractal.rng.clone();
        variables.func2("rand", move |x, y| {
            let mut rng = (*rng).borrow_mut();
            expressions::rand(&mut rng, x, y)
        });

        // Add initial values and keys
        for (key, value) in fractal.variables.iter() {
            variables.var(key, *value);

            stack1.insert(key.to_string(), vec![*value]);
        }
        stack2.push((pos, rot, thickness, color));

        let mut ignore_counter = 0;

        // Fill background
        let bg_col = fractal.background_color;
        self.set_color(bg_col.0, bg_col.1, bg_col.2, bg_col.3);
        self.rectangle(0.0, 0.0, fractal.canvas.0 as f64, fractal.canvas.1 as f64);
        self.fill();

        // Execute each operation
        for (index, c) in fractal.expression.chars().enumerate() {
            // Skip chars if an ignore action has been called
            if ignore_counter > 0 {
                ignore_counter -= 1;
                continue;
            }

            // Update index variable
            variables.var("INDEX", index as f64);

            let op = fractal.operations.get(&c);

            if let Some(op) = op {
                match op {
                    Operation::Forward(expr) => {
                        let length = expr.eval_with_context(&variables)?;
                        let start_pos = pos;
                        pos = (pos.0 - length * rot.sin(), pos.1 - length * rot.cos());

                        self.set_line_width(thickness);
                        self.set_color(color.0, color.1, color.2, color.3);
                        self.move_to(start_pos.0, start_pos.1);
                        self.line_to(pos.0, pos.1);
                        self.stroke();
                    }
                    Operation::Jump(expr) => {
                        let length = expr.eval_with_context(&variables)?;
                        pos = (pos.0 - length * rot.sin(), pos.1 - length * rot.cos());
                    }
                    Operation::Dot(expr) => {
                        let radius = expr.eval_with_context(&variables)?;

                        self.set_color(color.0, color.1, color.2, color.3);
                        self.arc(pos.0, pos.1, radius);
                        self.fill();
                    }
                    Operation::Rotate(expr) => {
                        rot += expr.eval_with_context(&variables)?;
                    }
                    Operation::Thickness(expr) => {
                        thickness = expr.eval_with_context(&variables)?;
                    }
                    Operation::Ignore(expr) => {
                        let v = expr.eval_with_context(&variables)?;
                        ignore_counter = v as u32;
                    }
                    Operation::PushStack => {
                        for (key, value) in &mut stack1 {
                            if let Some(v) = variables.get_var(key) {
                                value.push(v);
                            }
                        }
                        stack2.push((pos, rot, thickness, color));

                        // Update depth value
                        depth += 1;
                        variables.var("DEPTH", depth as f64);

                        self.save();
                    }
                    Operation::PopStack => {
                        if let Some((old_pos, old_rot, old_thickness, old_color)) = stack2.pop() {
                            // restore variables
                            pos = old_pos;
                            rot = old_rot;
                            thickness = old_thickness;
                            color = old_color;
                            for (key, value) in &mut stack1 {
                                if let Some(v) = value.pop() {
                                    variables.var(key, v);
                                }
                            }

                            // Update depth variable
                            depth -= 1;
                            variables.var("DEPTH", depth as f64);

                            self.restore();
                        }
                    }
                    Operation::SetColor(color_value) => {
                        color = *color_value;
                    }

                    Operation::SetVar(name, expr) => {
                        let v = expr.eval_with_context(&variables)?;
                        variables.var(name, v);
                    }
                }
            }
        }

        Ok(())
    }

}
