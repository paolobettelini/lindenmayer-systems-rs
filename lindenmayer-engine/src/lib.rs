use std::collections::HashMap;

#[derive(Debug)]
pub struct LSystem {
    pub axiom: String,
    pub rules: HashMap<char, String>,
}

impl LSystem {
    pub fn new(axiom: &str, rules: &[(char, String)]) -> Self {
        // maybe &[(char, &str)] as parameter ?

        let axiom = axiom.to_string();
        let mut rule_map = HashMap::new();

        for rule in rules {
            rule_map.insert(rule.0, rule.1.to_string());
        }

        LSystem {
            axiom,
            rules: rule_map,
        }
    }

    pub fn expand(&self, iter: usize) -> String {
        self.apply_rules_times(self.axiom.clone(), iter)
    }

    pub fn apply_rules_times(&self, expression: String, iter: usize) -> String {
        let mut result = expression;

        for _ in 0..iter {
            result = self.apply_rules(result);
        }

        result
    }

    pub fn apply_rules(&self, expression: String) -> String {
        let mut result = String::new();

        for c in expression.chars() {
            if let Some(s) = self.rules.get(&c) {
                result.push_str(s)
            } else {
                result.push(c)
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn algae() {
        let axiom = "A";
        let rules = [('A', String::from("AB")), ('B', String::from("A"))];

        let system = LSystem::new(&axiom, &rules);

        assert_eq!(&system.expand(0), "A");
        assert_eq!(&system.expand(1), "AB");
        assert_eq!(&system.expand(2), "ABA");
        assert_eq!(&system.expand(3), "ABAAB");
        assert_eq!(&system.expand(4), "ABAABABA");
        assert_eq!(&system.expand(5), "ABAABABAABAAB");
        assert_eq!(&system.expand(6), "ABAABABAABAABABAABABA");
        assert_eq!(&system.expand(7), "ABAABABAABAABABAABABAABAABABAABAAB");
    }
}
