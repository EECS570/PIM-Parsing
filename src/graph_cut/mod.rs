use crate::{base_type::{NamedBlock, Size}, sem_type::SemanticNodeInst};
use std::rc::Rc;
use z3::{
    ast::{Bool, Int, BV, Ast},
    Config, Context, Optimize, SatResult,
};
pub struct PIMCoreAssignment {
    pub nodes: Vec<Rc<SemanticNodeInst>>,
}

fn assign_with_z3(unions: Vec<Rc<SemanticNodeInst>>, core_size: i64, core_num: i64) -> () {
    let config = Config::new();
    let context = Context::new(&config);
    let optimizer = Optimize::new(&context);
    let union_size = unions.len();
    let xs: Vec<Int> = (0..union_size)
        .map(|i| Int::new_const(&context, format! {"x_{}", i}))
        .collect();

    for x in &xs {
        optimizer.assert(&x.ge(&Int::from_i64(&context, 0)));
        optimizer.assert(&x.lt(&Int::from_i64(&context, core_num)));
    }

    
    for j in 0..core_num {
        // For bag j, compute total weight.
        // We build an Int expression representing the total weight in bag j.
        let mut total_weight = Int::from_i64(&context, 0);
        for i in 0..union_size {
            // Convert the boolean decision (true/false) to an integer 1/0.
            // This is done via an if-then-else.

            let node_in_core = xs[i]._eq(&Int::from_i64(&context, j));
            let node_size_in_core = node_in_core.ite(&Int::from_i64(&context, unions[i].node_type.size_byte()), &Int::from_i64(&context, 0));
            // Multiply by the weight of the item.
            // let weighted = node.mul(&[&Int::from_i64(&ctx, weights[i] as i64)]);
            total_weight = total_weight + node_size_in_core;
        }
        // The total weight must be <= capacity of bag j.
        let capacity_expr = Int::from_i64(&context, core_size);
        let capacity_constraint = total_weight.le(&capacity_expr);
        optimizer.assert(&capacity_constraint);
    }
    

    let total_weight = Int::from_i64(&context, 0);
    optimizer.maximize(&total_weight);
    match optimizer.check(&[]) {
        SatResult::Sat => {
            let model = optimizer.get_model().unwrap();
            for (i, x) in xs.iter().enumerate() {
                let val = model.eval(x, true).unwrap().as_i64().unwrap();
                println!("Node {} assigned to core {}", i, val);
            }
        }
        _ => println!("No mathing result found."),
    }

}

#[test]

fn test_z3() {
    let nb = Rc::new(NamedBlock {
        name: String::from(""),
        fields: vec![],
    });
    let u: Vec<Rc<SemanticNodeInst>> = vec![
        Rc::new(SemanticNodeInst {
            varname: String::from("n1"),
            node_type: nb.clone(),
        }),
        Rc::new(SemanticNodeInst {
            varname: String::from("n2"),
            node_type: nb.clone(),
        }),
        Rc::new(SemanticNodeInst {
            varname: String::from("n3"),
            node_type: nb.clone(),
        }),
    ];
    assign_with_z3(u, 1, 2);
}

