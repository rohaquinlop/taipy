use pyo3::{exceptions::PyTypeError, prelude::*};
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Type {
    PyInt,
    PyFloat,
    PyString,
    PyBool,
    PyNone,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::PyInt => write!(f, "int"),
            Type::PyFloat => write!(f, "float"),
            Type::PyString => write!(f, "str"),
            Type::PyBool => write!(f, "bool"),
            Type::PyNone => write!(f, "None"),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Variable {
    pub name: String,
    pub type_: Vec<Type>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<Type>,
    pub mapped_args: HashMap<String, Type>,
    pub return_type: Vec<Type>,
}

#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub methods: Vec<Function>,
    pub attributes: Vec<Variable>,
}

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub classes: Vec<Class>,
    pub functions: Vec<Function>,
    pub variables: Vec<Variable>,
}

#[derive(Debug)]
pub struct Context {
    pub modules: Vec<Module>,
    pub classes: Vec<Class>,
    pub functions: Vec<Function>,
    pub variables: Vec<Variable>,
}

impl Context {
    pub fn init() -> Self {
        Context {
            modules: Vec::new(),
            classes: Vec::new(),
            functions: Vec::new(),
            variables: Vec::new(),
        }
    }

    // Exists

    fn exists_variable(&self, variable: Variable) -> PyResult<bool> {
        // Check if a variable exists in the context
        // Complexity: O(log(n))

        if self.variables.len() == 0 {
            return Ok(false);
        }

        let mut lo = 0;
        let mut hi = self.variables.len();
        let mut mid;

        while hi - lo > 1 {
            mid = lo + (hi - lo) / 2;

            if variable.name < self.variables[mid].name {
                hi = mid;
            } else if variable.name > self.variables[mid].name {
                lo = mid;
            } else {
                if variable == self.variables[mid] {
                    return Ok(true);
                } else {
                    let error_msg = format!(
                        "Variable '{:}' type mismatch. Expected: {:}, got: {:}",
                        variable.name.as_str(),
                        self.variables[mid]
                            .type_
                            .iter()
                            .map(|t| t.to_string())
                            .collect::<Vec<String>>()
                            .join(" | "),
                        variable
                            .type_
                            .iter()
                            .map(|t| t.to_string())
                            .collect::<Vec<String>>()
                            .join(" | "),
                    );
                    return Err(PyTypeError::new_err(error_msg));
                }
            }
        }

        if variable.name == self.variables[lo].name {
            if variable != self.variables[lo] {
                let error_msg = format!(
                    "Variable '{:}' type mismatch. Expected: {:}, got: {:}",
                    variable.name.to_string(),
                    self.variables[lo]
                        .type_
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<String>>()
                        .join(" | "),
                    variable
                        .type_
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<String>>()
                        .join(" | "),
                );
                return Err(PyTypeError::new_err(error_msg));
            } else {
                return Ok(true);
            }
        }

        Ok(false)
    }

    // Add

    pub fn add_variable(&mut self, variable: Variable) -> PyResult<()> {
        // Insert the variable in a sorted vector
        // Complexity: O(log(n))

        match self.exists_variable(variable.clone()) {
            Ok(true) => return Ok(()),
            Ok(false) => {}
            Err(e) => return Err(e),
        }

        if self.variables.len() == 0 {
            self.variables.push(variable);
        } else {
            let mut lo = 0;
            let mut hi = self.variables.len();
            let mut mid;

            while hi - lo > 1 {
                mid = lo + (hi - lo) / 2;

                if variable.name < self.variables[mid].name {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }

            self.variables.insert(hi, variable);
        }

        Ok(())
    }
}
