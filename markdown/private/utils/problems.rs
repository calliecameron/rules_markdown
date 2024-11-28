use std::fmt::{Display, Formatter};
use std::process;

#[derive(Clone, Debug)]
pub struct RowProblem {
    row: usize,
    msg: String,
}

impl RowProblem {
    pub fn new(row: usize, msg: &str) -> RowProblem {
        RowProblem {
            row,
            msg: String::from(msg),
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }
}

impl Display for RowProblem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "row {}: {}", self.row + 1, self.msg)
    }
}

#[derive(Clone, Debug)]
pub struct ColProblem {
    col: usize,
    msg: String,
}

impl ColProblem {
    pub fn new(col: usize, msg: &str) -> ColProblem {
        ColProblem {
            col,
            msg: String::from(msg),
        }
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }

    pub fn add_row(self, row: usize) -> RowColProblem {
        RowColProblem::new(row, self.col, &self.msg)
    }
}

#[derive(Clone, Debug)]
pub struct RowColProblem {
    row: usize,
    col: usize,
    msg: String,
}

impl RowColProblem {
    pub fn new(row: usize, col: usize, msg: &str) -> RowColProblem {
        RowColProblem {
            row,
            col,
            msg: String::from(msg),
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }
}

impl Display for RowColProblem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "row {} col {}: {}", self.row + 1, self.col + 1, self.msg)
    }
}

pub struct Problems {
    err_msg: String,
    problems: Vec<Box<dyn Display>>,
}

impl Problems {
    pub fn new(err_msg: &str) -> Problems {
        Problems {
            err_msg: String::from(err_msg),
            problems: Vec::new(),
        }
    }

    pub fn push(&mut self, p: Box<dyn Display>) {
        self.problems.push(p);
    }

    fn check_internal(&self) -> Option<String> {
        if self.problems.is_empty() {
            return None;
        }

        let mut msg = vec![format!("ERROR: {}", self.err_msg)];
        for p in self.problems.iter() {
            msg.push(format!("  {}", p));
        }
        Some(format!("{}\n\n", msg.join("\n\n")))
    }

    pub fn check(&self) {
        if let Some(msg) = self.check_internal() {
            eprintln!("{}", msg);
            process::exit(1);
        }
    }
}

impl Extend<Box<dyn Display>> for Problems {
    fn extend<T: IntoIterator<Item = Box<dyn Display>>>(&mut self, iter: T) {
        self.problems.extend(iter);
    }
}

#[cfg(test)]
mod test_problems {
    use super::{ColProblem, Display, Problems, RowColProblem, RowProblem};

    #[test]
    fn test_row_problem() {
        let p = RowProblem::new(2, "foo");
        assert_eq!(p.row(), 2);
        assert_eq!(p.msg(), "foo");
        assert_eq!(format!("{p}"), "row 3: foo");
    }

    #[test]
    fn test_col_problem() {
        let p = ColProblem::new(2, "foo");
        assert_eq!(p.col(), 2);
        assert_eq!(p.msg(), "foo");
        let p2 = p.add_row(5);
        assert_eq!(p2.row(), 5);
        assert_eq!(p2.col(), 2);
        assert_eq!(p2.msg(), "foo");
    }

    #[test]
    fn test_row_col_problem() {
        let p = RowColProblem::new(2, 5, "foo");
        assert_eq!(p.row(), 2);
        assert_eq!(p.col(), 5);
        assert_eq!(p.msg(), "foo");
        assert_eq!(format!("{p}"), "row 3 col 6: foo");
    }

    #[test]
    fn test_problems() {
        let mut p = Problems::new("foo");
        assert!(p.check_internal().is_none());
        p.push(Box::new(String::from("bar")));
        let v: Vec<Box<dyn Display>> = vec![
            Box::new(String::from("baz")),
            Box::new(String::from("quux")),
        ];
        p.extend(v);
        assert_eq!(
            p.check_internal().unwrap(),
            "ERROR: foo

  bar

  baz

  quux

"
        );
    }
}
