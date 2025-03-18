mod base_type;
mod code_gen;
mod parser;
mod sem_type;
mod semantics_analysis;
use crate::parser::parse_str;
use anyhow::Result;
use base_type::NamedBlock;
use clap::Parser;
use code_gen::TypeCodeGen;
use semantics_analysis::semantic_analysis;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Reading from: {}", args.file);
    let file_content = fs::read_to_string(args.file)?;
    println!("File content: {}", file_content);
    let sem = semantic_analysis(parse_str(&file_content)?)?;

    println!("------------Generating Codes--------------");
    if sem.edges.is_empty() {
        println!("No edges found");
    }
    else {
        println!("Generated Edges Code:");
        for (edge_name, edge_rc) in &sem.edges {
            let edge_from = edge_rc.from.as_ref().clone().type_code(); 
            let edge_to = edge_rc.to.as_ref().clone().type_code();   
        
            println!("Edge '{}':\nFrom: {}\nTo: {}", edge_name, edge_from, edge_to);
        }
    }

    if sem.walkers.is_empty() {
        println!("No walkers found");
    }
    else {
        println!("Generated Walkers Code:");
        for (walker_name, walker_rc) in &sem.walkers {
            let walker_node_type = walker_rc.node_type.as_ref().clone().type_code();   
        
            println!("Walker '{}':\nNode Type: {}", walker_name, walker_node_type);
        }

    }

        
    if sem.graphs.is_empty() {
        println!("No graphs found");
    }
    else {
        println!("Generated Graphs Code:");
        for graph in &sem.graphs {
            println!("Graph:");
            for node_inst in &graph.node_insts {
                let node_inst_node_type = node_inst.node_type.as_ref().clone().type_code(); 
                let node_inst_varname = node_inst.varname.clone();
                println!("----Node Instance:----\nVarname: {}\nNode Type: {}", node_inst_varname, node_inst_node_type);  
            }
            for edge_inst in &graph.edge_insts {
                let edge_inst_edge_type = edge_inst.edge_type.as_ref().type_code(); 
                let edge_inst_from_var = edge_inst.from_var.as_ref().varname.clone();
                let edge_inst_to_var = edge_inst.to_var.as_ref().varname.clone();
                let edge_inst_weight = edge_inst.weight;
                println!("----Edge Instance:----\nFrom Node: {}\nTo Node: {}\nEdge Type: {}\nWeight: {}", edge_inst_from_var, edge_inst_to_var, edge_inst_edge_type, edge_inst_weight); 
            }
            for walker_inst in &graph.walker_insts {
                let walker_inst_walker_type = walker_inst.walker_type.as_ref().node_type.as_ref().clone().type_code(); 
                let walker_inst_start_node = walker_inst.start_node.as_ref().varname.clone();
                println!("----Walker Instance:----\nStart Node: {}\nWalker Type: {}", walker_inst_start_node, walker_inst_walker_type); 
            }
        }
    }

    Ok(())
}