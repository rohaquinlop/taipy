use crate::classes::{
    Type,
    Type::{PyBool, PyFloat, PyInt, PyNone, PyString},
    Variable,
};
use rustpython_parser::ast::{self, Stmt};

pub fn create_variables_from_assign(assign: Stmt) -> Vec<Variable> {
    let mut variables: Vec<Variable> = Vec::new();

    match assign {
        Stmt::Assign(a) => {
            let mut var_names: Vec<String> = Vec::new();
            let mut var_values: Vec<Type> = Vec::new();

            for target in a.targets {
                match target {
                    ast::Expr::Name(name) => {
                        var_names.push(name.id.to_string());
                    }
                    _ => {}
                }
            }

            match *a.value {
                ast::Expr::Constant(c) => match c.value {
                    ast::Constant::Int(_) => {
                        var_values.push(PyInt);
                    }
                    ast::Constant::Float(_) => {
                        var_values.push(PyFloat);
                    }
                    ast::Constant::Bool(_) => {
                        var_values.push(PyBool);
                    }
                    ast::Constant::Str(_) => {
                        var_values.push(PyString);
                    }
                    ast::Constant::None => {
                        var_values.push(PyNone);
                    }
                    _ => {}
                },
                _ => {}
            }

            for i in 0..var_names.len() {
                variables.push(Variable {
                    name: var_names[i].to_string(),
                    type_: vec![var_values[i]],
                });
            }
        }
        _ => {}
    }

    variables
}
