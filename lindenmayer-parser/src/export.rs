use std::fmt::Write;
use crate::*;

pub fn serialize_renderer(editor: &ConfEditor) -> String {
    let mut res = String::from("");

    macro_rules! add_lines {
        ($iter:expr) => {
            for line in $iter {
                if !&line.is_empty() {
                    let _ = writeln!(res, "{}", &line);
                }
            }
            
            let _ = writeln!(res, "");
        }
    }

    add_lines!(&editor.configurations);
    add_lines!(&editor.variables);
    add_lines!(&editor.operations);
    add_lines!(&editor.rules);

    res
}