use crate::{
    base_type::{NamedBlock, Size},
    parser::parse_str,
    sem_type::{SemanticEdgeInst, SemanticNodeInst},
    semantics_analysis::semantic_analysis,
};
use anyhow::Result;
use std::rc::Rc;
use std::{collections::HashMap, f32::consts::E};
use thiserror::Error;
use z3::{
    ast::{Ast, Bool, Int, BV},
    Config, Context, Optimize, SatResult,
};

use std::fs;

#[derive(Error, Debug)]
pub enum DataMappingError {
    #[error("Node `{0}` not found when data mapping")]
    NodeNotFound(String),
    #[error("No solution found")]
    NoSolutionFound,

    #[error("Unknown error.")]
    Unknown,
}

pub struct PIMCoreAssignment {
    pub nodes: Vec<Rc<SemanticNodeInst>>,
}

fn find_node_with_name(nodes: &Vec<(Rc<SemanticNodeInst>, Int)>, target: String) -> Result<usize> {
    for i in 0..nodes.len() {
        if nodes[i].0.varname == target {
            return Ok(i);
        }
    }
    Err(DataMappingError::NodeNotFound(target).into())
}

fn construct_result(
    optimizer: &Optimize,
    xs: &Vec<(Rc<SemanticNodeInst>, Int)>,
    core_num: i64,
) -> Result<Vec<Vec<Rc<SemanticNodeInst>>>> {
    let mut result: Vec<Vec<Rc<SemanticNodeInst>>> = vec![vec![]; core_num as usize];

    match optimizer.check(&[]) {
        SatResult::Sat => {
            let model = optimizer.get_model().unwrap();
            for (i, x) in xs.iter().enumerate() {
                let val = model.eval(&x.1, true).unwrap().as_i64().unwrap();
                result[val as usize].extend([x.0.clone()]);
                println!("Node {} assigned to core {}", i, val);
            }
        }
        _ => return Err(DataMappingError::NoSolutionFound.into()),
    };
    return Ok(result);
}

fn assign_with_z3(
    unions: &Vec<Rc<SemanticNodeInst>>,
    edges: &Vec<Rc<SemanticEdgeInst>>,
    core_size: i64,
    core_num: i64,
) -> Result<Vec<Vec<Rc<SemanticNodeInst>>>> {
    let config = Config::new();
    let context = Context::new(&config);
    let optimizer = Optimize::new(&context);
    let union_size = unions.len();
    let xs: Vec<(Rc<SemanticNodeInst>, Int)> = (0..union_size)
        .map(|i| {
            (
                unions[i].clone(),
                Int::new_const(&context, format! {"x_{}", i}),
            )
        })
        .collect();

    let mut var_map: HashMap<String, Int> = HashMap::new();

    for x in &xs {
        optimizer.assert(&x.1.ge(&Int::from_i64(&context, 0)));
        optimizer.assert(&x.1.lt(&Int::from_i64(&context, core_num)));
    }

    for j in 0..core_num {
        // For bag j, compute total weight.
        // We build an Int expression representing the total weight in bag j.
        let mut total_weight = Int::from_i64(&context, 0);
        for i in 0..union_size {
            // Convert the boolean decision (true/false) to an integer 1/0.
            // This is done via an if-then-else.

            let node_in_core = xs[i].1._eq(&Int::from_i64(&context, j));
            let node_size_in_core = node_in_core.ite(
                &Int::from_i64(&context, unions[i].node_type.size_byte()),
                &Int::from_i64(&context, 0),
            );
            // Multiply by the weight of the item.
            // let weighted = node.mul(&[&Int::from_i64(&ctx, weights[i] as i64)]);
            total_weight = total_weight + node_size_in_core;
        }

        // Calculate all edges
        for edge in edges {
            let from_id = find_node_with_name(&xs, edge.from_var.varname.clone())?;
            let to_id = find_node_with_name(&xs, edge.to_var.varname.clone())?;
            let list = [
                &Int::from_i64(&context, j)._eq(&xs[from_id].1),
                &Int::from_i64(&context, j)._eq(&xs[to_id].1),
            ];
            let edge_in_core = Bool::or(&context, &list);
            let edge_weight = edge_in_core.ite(
                &Int::from_i64(&context, edge.edge_type.named_block.size_byte()),
                &Int::from_i64(&context, 0),
            );
            total_weight = total_weight + edge_weight;
        }
        // The total weight must be <= capacity of bag j.
        let capacity_expr = Int::from_i64(&context, core_size);
        let capacity_constraint = total_weight.le(&capacity_expr);
        optimizer.assert(&capacity_constraint);
    }

    let mut evaluation = Int::from_i64(&context, 0);
    for edge in edges {
        let from_id = find_node_with_name(&xs, edge.from_var.varname.clone())?;
        let to_id = find_node_with_name(&xs, edge.to_var.varname.clone())?;
        let to_x = &xs[to_id].1;
        let weight = Int::from_i64(&context, edge.weight);
        evaluation = evaluation
            + xs[from_id]
                .1
                ._eq(to_x)
                .ite(&weight, &Int::from_i64(&context, 0));
    }

    optimizer.maximize(&evaluation);
    construct_result(&optimizer, &xs, core_num)
}

#[test]

fn test_z3() -> Result<()> {
    let file_context = fs::read_to_string(String::from("examples/test_dm.dspim"))?;
    let sm = semantic_analysis(parse_str(&file_context)?)?;
    let g = sm.graphs[0].clone();
    assign_with_z3(&g.node_insts, &g.edge_insts, 3, 3)?;
    Ok(())
}
