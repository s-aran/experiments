use std::{fs::File, io::Read, path::Path};

use rustpython_parser::{
    ast::{self, StmtClassDef, StmtFunctionDef, StmtImport},
    Parse,
};

fn main() {
    let test_py_base_path = Path::new("test_files");
    let test_py_path = test_py_base_path.join("test_simple.py");

    let mut test_file = match File::open(test_py_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let mut buf = String::new();
    match test_file.read_to_string(&mut buf) {
        Ok(s) => {
            println!("{} bytes read.", s);
        }
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let result = match ast::Suite::parse(
        buf.as_str(),
        test_py_base_path.file_name().unwrap().to_str().unwrap(),
    )
    .map_err(|e| e.to_string())
    {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    };

    for e in result.iter() {
        println!("{:?}", e);
    }

    println!("");

    // let rf = result.get(0).unwrap();
    // let import_stmt = rf.as_import_stmt().unwrap();
    // let name_first = import_stmt.names.get(0).unwrap();
    // println!("{} as {:?}", name_first.name, name_first.asname);

    // let rl = result.get(result.len() - 1).unwrap();
    // let class_deco = rl.as_class_def_stmt().unwrap();
    // for d in class_deco.decorator_list.iter() {
    //     println!("{:?}", d);
    // }

    let mut states = States::default();
    let tests: Vec<String> = vec![];

    let mut test_classes = Vec::<&StmtClassDef>::new();
    let mut test_methods = Vec::<&StmtFunctionDef>::new();
    for rs in result.iter() {
        match rs {
            ast::Stmt::Import(stmt) => import_stmt(&mut states, stmt),
            ast::Stmt::ClassDef(stmt) => {
                test_classes.push(stmt);
                class_def_stmt(&mut states, &stmt);
            }
            ast::Stmt::FunctionDef(stmt) => {
                test_methods.push(stmt);
                func_def_stmt(&mut states, &stmt);
            }

            _ => {}
        }
    }

    println!("classes: {}", test_classes.len());
    for stmt in test_classes.iter() {
        println!("{:?}", stmt.name);

        println!("* {} in methods: {}", stmt.name, stmt.body.len());
        for b in stmt.body.iter() {
            if !b.is_function_def_stmt() {
                continue;
            }

            let func = b.as_function_def_stmt().unwrap();
            println!("* {}", func.name);
        }
    }

    println!("methods: {}", test_methods.len());
    for stmt in test_methods.iter() {
        println!("{:?}", stmt.name);
    }
}

#[derive(Debug, Default)]
struct States {
    has_unittest: bool,
    imported_skip: bool,
}

fn import_stmt(states: &mut States, stmt: &StmtImport) {}

fn class_def_stmt(states: &mut States, stmt: &StmtClassDef) {}

fn func_def_stmt(states: &mut States, stmt: &StmtFunctionDef) {}
