pub mod export;
pub use import::*;
mod import;

#[derive(Default, Debug)]
pub struct ConfEditor {
    pub configurations: Vec<String>,
    pub rules: Vec<String>,
    pub operations: Vec<String>,
    pub variables: Vec<String>,
}

// Commands
pub const AXIOM: &str = "axiom";
pub const ITER: &str = "iter";
pub const INITIAL_ROT: &str = "initial_rot";
pub const INITIAL_POS: &str = "initial_pos";
pub const INITIAL_THICKNESS: &str = "initial_thickness";
pub const INITIAL_COLOR: &str = "initial_color";
pub const BACKGROUND: &str = "background";
pub const CANVAS: &str = "canvas";
pub const SEED: &str = "seed";
pub const INJECT: &str = "inject";
pub const COMMENT: &str = ";";

// Symbols
pub const OP_DECLARATION: &str = ":";
pub const RULE_DECLARATION: &str = "->";
pub const VAR_DECLARATION: &str = "=";
pub const TUPLE_SEPARATOR: &str = ",";

// Operations
pub const FORWARD: &str = "forward";
pub const JUMP: &str = "jump";
pub const DOT: &str = "dot";
pub const ROTATE: &str = "rotate";
pub const THICKNESS: &str = "thickness";
pub const IGNORE: &str = "ignore";
pub const PUSH: &str = "push";
pub const POP: &str = "pop";
pub const COLOR: &str = "color";