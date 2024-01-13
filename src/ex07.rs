use crate::propositional_formula::PropositionalFormula;

impl PropositionalFormula {
    pub fn sat(&self) -> bool {
        let mask: u32 = 1 << self.variables.len();
        let mut result = false;

        for m in 0..mask {
            let variable_map = self.get_variable_map(m);
            if self.evaluate(&variable_map) {
                result = true;
                break;
            }
        }
        result
    }
}

pub fn sat(formula: &str) -> bool {
    if let Ok(pf) = PropositionalFormula::try_from(formula) {
        pf.sat()
    } else {
        println!("Error");
        false
    }
}

#[test]
fn test_sat() {
    assert_eq!(sat("AB|"), true);
    assert_eq!(sat("AB&"), true);
    assert_eq!(sat("AA!&"), false);
    assert_eq!(sat("AA^"), false);
}
