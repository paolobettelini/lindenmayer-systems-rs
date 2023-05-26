use crate::*;
use lindenmayer_renderer::{LSystemRenderer, Operation};
use std::str::FromStr;

pub type Result<T> = std::result::Result<T, ParsingError>;

pub trait Updatable<T> {
    fn update(&mut self, conf: T) -> Result<LineType>;

    fn get_line_type(&self, conf: T) -> LineType;
}

#[derive(Debug)]
pub enum ParsingError {
    UnknownAction,
    InvalidFormat,
    InvalidInteger,
    InvalidFloatingPoint,
    InvalidOperation,
    InvalidExpression,
    InvalidTuple,
    InvalidColor,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LineType {
    Config,
    Variable,
    Rule,
    Operation,
    Nothing,
    Unknown,
}

impl Updatable<&str> for LSystemRenderer {
    fn update(&mut self, line: &str) -> Result<LineType> {
        let line = line.trim();

        if line.is_empty() {
            return Ok(LineType::Nothing);
        }

        if line.starts_with(AXIOM) {
            self.lsystem.axiom = parse_axiom(line)?;
            return Ok(LineType::Config);
        }

        if line.starts_with(ITER) {
            self.iter = parse_iter(line)?;
            return Ok(LineType::Config);
        }

        if line.starts_with(INITIAL_ROT) {
            self.initial_rot = parse_initial_rot(line)?;
            return Ok(LineType::Config);
        }

        if line.starts_with(INITIAL_POS) {
            self.initial_pos = parse_initial_pos(line)?;
            return Ok(LineType::Config);
        }

        if line.starts_with(INITIAL_THICKNESS) {
            self.initial_thickness = parse_initial_thickness(line)?;
            return Ok(LineType::Config);
        }

        if line.starts_with(BACKGROUND) {
            self.background_color = parse_background_color(line)?;
            return Ok(LineType::Config);
        }

        if line.starts_with(INITIAL_COLOR) {
            self.initial_color = parse_initial_color(line)?;
            return Ok(LineType::Config);
        }

        if line.starts_with(CANVAS) {
            self.canvas = parse_canvas(line)?;
            return Ok(LineType::Config);
        }

        if line.starts_with(SEED) {
            self.seed = parse_seed(line)?;
            return Ok(LineType::Config);
        }

        if line.starts_with(INJECT) {
            self.injections = parse_injections(line)?;
            return Ok(LineType::Config);
        }

        if line.contains(RULE_DECLARATION) {
            let (character, rule) = parse_rule(line)?;
            self.lsystem.rules.insert(character, rule);
            return Ok(LineType::Rule);
        }

        if line.contains(OP_DECLARATION) {
            let (character, operation) = parse_char_operation(line)?;
            self.operations.insert(character, operation);
            return Ok(LineType::Operation);
        }

        if line.contains(VAR_DECLARATION) {
            let (name, value) = parse_variable(line)?;
            self.variables.insert(name, value);
            return Ok(LineType::Variable);
        }

        Err(ParsingError::UnknownAction)
    }

    fn get_line_type(&self, line: &str) -> LineType {
        if line.is_empty() {
            LineType::Nothing
        } else if line.starts_with(AXIOM)
            || line.starts_with(ITER)
            || line.starts_with(INITIAL_ROT)
            || line.starts_with(INITIAL_POS)
            || line.starts_with(INITIAL_THICKNESS)
            || line.starts_with(BACKGROUND)
            || line.starts_with(INITIAL_COLOR)
            || line.starts_with(CANVAS)
            || line.starts_with(SEED)
            || line.starts_with(INJECT)
        {
            LineType::Config
        } else if line[1..].trim().starts_with(RULE_DECLARATION) {
            LineType::Rule
        } else if line[1..].trim().starts_with(OP_DECLARATION) {
            LineType::Operation
        } else if line.contains(VAR_DECLARATION) {
            LineType::Variable
        } else {
            LineType::Unknown
        }
    }
}

fn parse_axiom(line: &str) -> Result<String> {
    let parts = line.split(' ').collect::<Vec<_>>();
    let axiom = parts.get(1).ok_or(ParsingError::InvalidFormat)?;
    Ok((*axiom).to_owned())
}

fn parse_iter(line: &str) -> Result<usize> {
    let parts = line.split(' ').collect::<Vec<_>>();
    let iter = parts
        .get(1)
        .ok_or(ParsingError::InvalidFormat)?
        .parse::<usize>()
        .map_err(|_| ParsingError::InvalidInteger)?;
    Ok(iter)
}

fn parse_initial_rot(line: &str) -> Result<f64> {
    let parts = line.split(' ').collect::<Vec<_>>();
    let rot = parts
        .get(1)
        .ok_or(ParsingError::InvalidFormat)?
        .parse::<f64>()
        .map_err(|_| ParsingError::InvalidFloatingPoint)?;
    Ok(rot)
}

fn parse_initial_pos(line: &str) -> Result<(f64, f64)> {
    let start = INITIAL_POS.len() + 1;
    let line = line.get(start..).ok_or(ParsingError::InvalidFormat)?;
    let initial_pos = parse_tuple_arg::<f64, f64>(line)?;

    Ok(initial_pos)
}

fn parse_initial_thickness(line: &str) -> Result<f64> {
    let parts = line.split(' ').collect::<Vec<_>>();
    let thickness = parts
        .get(1)
        .ok_or(ParsingError::InvalidFormat)?
        .parse::<f64>()
        .map_err(|_| ParsingError::InvalidFloatingPoint)?;
    Ok(thickness)
}

fn parse_seed(line: &str) -> Result<String> {
    let seed = line
        .get((SEED.len() + 1)..)
        .ok_or(ParsingError::InvalidFormat)?;

    Ok(seed.to_string())
}

fn parse_injections(line: &str) -> Result<Vec<(u32, String)>> {
    // There is no value
    if line.len() == INJECT.len() {
        return Ok(Vec::new());
    }

    let parts = line
        .get((INJECT.len() + 1)..)
        .ok_or(ParsingError::InvalidFormat)?
        .split(' ');

    let mut result = vec![];

    for part in parts {
        let tuple: (u32, String) = parse_tuple_arg(&part)?;
        if tuple.1.is_empty() {
            return Err(ParsingError::InvalidTuple);
        }
        result.push(tuple);
    }

    Ok(result)
}

fn parse_char_operation(line: &str) -> Result<(char, Operation)> {
    let character = line.chars().next().ok_or(ParsingError::InvalidFormat)?;
    let op = line.split_at(2).1.trim().to_string();
    let op = parse_operation(&op)?;
    Ok((character, op))
}

fn parse_rule(line: &str) -> Result<(char, String)> {
    let parts: Vec<&str> = line.split(RULE_DECLARATION).map(|s| s.trim()).collect();
    let character = parts
        .first()
        .ok_or(ParsingError::InvalidFormat)?
        .chars()
        .next()
        .ok_or(ParsingError::InvalidFormat)?;
    let rule = parts.get(1).ok_or(ParsingError::InvalidFormat)?.to_string();

    Ok((character, rule))
}

fn parse_variable(line: &str) -> Result<(String, f64)> {
    let parts: Vec<&str> = line.split(VAR_DECLARATION).map(|s| s.trim()).collect();
    let name = parts
        .first()
        .ok_or(ParsingError::InvalidFormat)?
        .to_string();
    let value: f64 = parts
        .get(1)
        .ok_or(ParsingError::InvalidFormat)?
        .parse()
        .map_err(|_| ParsingError::InvalidFloatingPoint)?;
    Ok((name, value))
}

fn parse_background_color(line: &str) -> Result<(f64, f64, f64, f64)> {
    let color = line
        .get((BACKGROUND.len() + 1)..)
        .ok_or(ParsingError::InvalidFormat)?;
    let color = csscolorparser::parse(color).map_err(|_| ParsingError::InvalidColor)?;

    Ok((color.r, color.g, color.b, color.a))
}

fn parse_initial_color(line: &str) -> Result<(f64, f64, f64, f64)> {
    let color = line
        .get((INITIAL_COLOR.len() + 1)..)
        .ok_or(ParsingError::InvalidFormat)?;
    let color = csscolorparser::parse(color).map_err(|_| ParsingError::InvalidColor)?;

    Ok((color.r, color.g, color.b, color.a))
}

fn parse_canvas(line: &str) -> Result<(i32, i32)> {
    let start = CANVAS.len() + 1;
    let line = line.get(start..).ok_or(ParsingError::InvalidFormat)?;
    let size = parse_tuple_arg::<i32, i32>(line)?;

    if size.0 < 0 || size.1 < 0 {
        return Err(ParsingError::InvalidInteger);
    }

    Ok(size)
}

fn parse_operation(operation: &str) -> Result<Operation> {
    let parts: Vec<&str> = operation.split(' ').collect();
    let part0 = parts.first().ok_or(ParsingError::InvalidFormat)?;

    macro_rules! parse_expr {
        () => {
            operation
                .get((part0.len() + 1)..)
                .ok_or(ParsingError::InvalidFormat)?
                .parse()
                .map_err(|_| ParsingError::InvalidExpression)?
        };
    }

    match *part0 {
        FORWARD => Ok(Operation::Forward(parse_expr!())),
        JUMP => Ok(Operation::Jump(parse_expr!())),
        DOT => Ok(Operation::Dot(parse_expr!())),
        ROTATE => Ok(Operation::Rotate(parse_expr!())),
        THICKNESS => Ok(Operation::Thickness(parse_expr!())),
        IGNORE => Ok(Operation::Ignore(parse_expr!())),
        PUSH => Ok(Operation::PushStack),
        POP => Ok(Operation::PopStack),
        COLOR => Ok(Operation::SetColor({
            let color = operation
                .get((part0.len() + 1)..)
                .ok_or(ParsingError::InvalidFormat)?;
            let color = csscolorparser::parse(color).map_err(|_| ParsingError::InvalidColor)?;

            (color.r, color.g, color.b, color.a)
        })),
        _ => {
            if operation.contains(VAR_DECLARATION) {
                // set var
                let var_name = parts
                    .first()
                    .ok_or(ParsingError::InvalidFormat)?
                    .to_string();
                let expr_string = parts.get(2..).ok_or(ParsingError::InvalidFormat)?.join(" ");
                Ok(Operation::SetVar(
                    var_name,
                    expr_string
                        .parse()
                        .map_err(|_| ParsingError::InvalidExpression)?,
                ))
            } else {
                Err(ParsingError::InvalidOperation)
            }
        }
    }
}

fn parse_tuple_arg<T1: FromStr, T2: FromStr>(line: &str) -> Result<(T1, T2)> {
    let parts = line.split(TUPLE_SEPARATOR).collect::<Vec<_>>();

    let v1 = parts
        .first()
        .ok_or(ParsingError::InvalidFormat)?
        .trim()
        .parse::<T1>()
        .map_err(|_| ParsingError::InvalidTuple)?;

    let v2 = parts
        .get(1)
        .ok_or(ParsingError::InvalidFormat)?
        .trim()
        .parse::<T2>()
        .map_err(|_| ParsingError::InvalidTuple)?;

    Ok((v1, v2))
}
