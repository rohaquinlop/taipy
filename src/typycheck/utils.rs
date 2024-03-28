use crate::classes::{
    Type,
    Type::{PyBool, PyFloat, PyInt, PyNone, PyString},
    Variable,
};
use pyo3::PyResult;
use rustpython_parser::ast::{self, Stmt};

pub fn create_variables_from_assign(assign: Stmt) -> PyResult<Vec<Variable>> {
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
        Stmt::AnnAssign(a) => {
            let var_name = match *a.target {
                ast::Expr::Name(name) => name.id.to_string(),
                _ => "".to_string(),
            };

            let types = get_types(*a.annotation);

            // Check if the value matches the annotation
            let value_type = match a.value {
                Some(expr) => get_types(*expr),
                None => {
                    return Err(pyo3::exceptions::PyTypeError::new_err(
                        "Variable must have a value",
                    ))
                }
            };

            // Check if the value is contained in the annotation
            for t in value_type {
                if !types.contains(&t) {
                    return Err(pyo3::exceptions::PyTypeError::new_err(
                        "Value type does not match annotation",
                    ));
                }
            }

            variables.push(Variable {
                name: var_name,
                type_: types,
            });
        }
        _ => {}
    }

    Ok(variables)
}

fn get_types(expr: ast::Expr) -> Vec<Type> {
    let mut types: Vec<Type> = Vec::new();

    match expr {
        ast::Expr::Constant(c) => match c.value {
            ast::Constant::Int(_) => {
                types.push(PyInt);
            }
            ast::Constant::Float(_) => {
                types.push(PyFloat);
            }
            ast::Constant::Bool(_) => {
                types.push(PyBool);
            }
            ast::Constant::Str(_) => {
                types.push(PyString);
            }
            ast::Constant::None => {
                types.push(PyNone);
            }
            _ => {}
        },
        ast::Expr::Name(n) => match n.id.as_str() {
            "int" => {
                types.push(PyInt);
            }
            "float" => {
                types.push(PyFloat);
            }
            "str" => {
                types.push(PyString);
            }
            "bool" => {
                types.push(PyBool);
            }
            _ => {}
        },
        ast::Expr::BinOp(b) => {
            let left = get_types(*b.left);
            let right = get_types(*b.right);

            types.append(&mut left.clone());
            types.append(&mut right.clone());
        }
        _ => {}
    }

    types
}
