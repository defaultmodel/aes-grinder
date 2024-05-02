mod algo;
mod cli;
mod exhaustive_search;
mod matrix;
mod parser;
mod utils;

use crate::exhaustive_search::{exhaustive_search, random_search};
use clap::Parser as ClapParser;

struct GlobalInfos {
    filename_eq_sys: String,
    sys_name: String,
    polynomial: u16,
}

impl GlobalInfos {
    fn new(filename_eq_sys: String) -> GlobalInfos {
        GlobalInfos {
            filename_eq_sys,
            sys_name: String::new(),
            polynomial: 0x11b,
        }
    }
}

fn main() {
    let mut cli = cli::Cli::parse();
    cli::cli_check(&mut cli);

    let mut globals: GlobalInfos = GlobalInfos::new(cli.equation_system);
    let mut parser_mod = parser::Parser::new(&globals);

    // NEED TO CATCH MATRIX
    let matrix = parser_mod
        .parse_system(&mut globals)
        .expect("Error while parsing system");
    let mut matrix = matrix;
    matrix.set_vars_map(parser_mod.vars_map);

    println!("{}", matrix);
    matrix.drop_linear_variable();
    println!("{}", matrix);

    let graph = exhaustive_search::random_search(&mut matrix);
    let _ = graph.to_dot_debug("/tmp/sac", &matrix);

    // let graph = exhaustive_search(matrix, 6);
    //  assert!(graph.len()==1);
    //println!("LEN GRAPH{:?}",graph);
    // graph.iter().next().unwrap().to_dot_debug("/tmp/algo.dot").unwrap();
}
