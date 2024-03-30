use crate::classes;

pub mod utils;

use pyo3::prelude::*;
use rustpython_parser::{
    ast::{self, Stmt},
    Parse,
};

#[pyfunction]
pub fn type_check_file(file_path: &str) -> PyResult<bool> {
    let code = std::fs::read_to_string(file_path)?;
    let ast = ast::Suite::parse(&code, "<embedded>").unwrap();

    let mut context = classes::Context::init();

    for stmt in ast.iter() {
        match type_check_stmt(stmt.clone(), &mut context) {
            Ok(_) => {}
            Err(e) => {
                println!("{:}", e.to_string());
                return Ok(false);
            }
        }
    }

    println!("{:#?}", context);

    Ok(true)
}

fn type_check_stmt(statement: Stmt, context: &mut classes::Context) -> PyResult<bool> {
    // println!("{:#?}", statement);

    let variables: Vec<classes::Variable> =
        match utils::create_variables_from_assign(statement.clone()) {
            Ok(vars) => vars,
            Err(e) => return Err(e.into()),
        };

    for variable in variables {
        match context.add_variable(variable.clone()) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
    }

    Ok(true)
}
