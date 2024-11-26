use std::fmt::{Display, Formatter, Result};
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "row {} col{}: {}", self.row + 1, self.col + 1, self.msg)
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

    pub fn check(&self) {
        if self.problems.is_empty() {
            return;
        }

        let mut msg = vec![format!("ERROR: {}", self.err_msg)];
        for p in self.problems.iter() {
            msg.push(p.to_string());
        }
        eprintln!("{}\n\n", msg.join("\n\n"));
        process::exit(1);
    }
}

impl Extend<Box<dyn Display>> for Problems {
    fn extend<T: IntoIterator<Item = Box<dyn Display>>>(&mut self, iter: T) {
        self.problems.extend(iter);
    }
}
