use std::fmt;

use self::error::ModelError;

pub mod error;
pub mod regmachine;

pub enum ModelType {
    RegisterMachine,
    TuringMachine,
    MarkovAlgorithm,
    PostTuringMachine,
    PartialRecursiveFunc,
    LambdaExpr,
}

#[derive(Debug)]
pub struct Args {
    inner: Vec<String>,
}

impl Args {
    pub fn new(args: &[&str]) -> Args {
        Args {
            inner: args.iter().map(|s| (*s).to_owned()).collect()
        }
    }
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();

        for (i, arg) in self.inner.iter().enumerate() {
            s.push_str(arg);

            if i < self.inner.len() {
                s.push_str(", ")
            }
        }

        f.write_str(&s)
    }
}

pub trait Model {
    type Step: Step;

    fn model_type(&self) -> ModelType;

    fn execute(&self, args: &Args) -> Result<(), ModelError>;
}

pub trait Step {

}

