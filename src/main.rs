mod base_type;
mod code_gen;
mod parser;
mod sem_type;
mod semantics_analysis;
mod graph_cut;
use crate::parser::parse_str;
use anyhow::Result;
use base_type::NamedBlock;
use clap::Parser;
use code_gen::TypeCodeGen;
use sem_type::SemanticGlobal;
use semantics_analysis::semantic_analysis;
use std::fs;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
    #[arg(short, long, default_value_t = 1)]
    count: u8,
    #[arg(short, long, default_value_t = String::from("generated_code.cpp"))]
    output: String,
}

fn print_info(sem: SemanticGlobal) -> (){
    println!("------------Generating Codes--------------");
    if sem.edges.is_empty() {
        println!("No edges found");
    } else {
        println!("Generated Edges Code:");
        for (edge_name, edge_rc) in &sem.edges {
            let edge_from = edge_rc.from.as_ref().clone().type_code();
            let edge_to = edge_rc.to.as_ref().clone().type_code();

            println!(
                "Edge '{}':\nFrom: {}\nTo: {}",
                edge_name, edge_from, edge_to
            );
        }
    }

    if sem.walkers.is_empty() {
        println!("No walkers found");
    } else {
        println!("Generated Walkers Code:");
        for (walker_name, walker_rc) in &sem.walkers {
            let walker_node_type = walker_rc.node_type.as_ref().clone().type_code();

            println!("Walker '{}':\nNode Type: {}", walker_name, walker_node_type);
        }
    }

    if sem.graphs.is_empty() {
        println!("No graphs found");
    } else {
        println!("Generated Graphs Code:");
        for graph in &sem.graphs {
            println!("Graph:");
            for node_inst in &graph.node_insts {
                let node_inst_node_type = node_inst.node_type.as_ref().clone().type_code();
                let node_inst_varname = node_inst.varname.clone();
                println!(
                    "----Node Instance:----\nVarname: {}\nNode Type: {}",
                    node_inst_varname, node_inst_node_type
                );
            }
            for edge_inst in &graph.edge_insts {
                let edge_inst_edge_type = edge_inst.edge_type.as_ref().type_code();
                let edge_inst_from_var = edge_inst.from_var.as_ref().varname.clone();
                let edge_inst_to_var = edge_inst.to_var.as_ref().varname.clone();
                let edge_inst_weight = edge_inst.weight;
                println!(
                    "----Edge Instance:----\nFrom Node: {}\nTo Node: {}\nEdge Type: {}\nWeight: {}",
                    edge_inst_from_var, edge_inst_to_var, edge_inst_edge_type, edge_inst_weight
                );
            }
            for walker_inst in &graph.walker_insts {
                let walker_inst_walker_type = walker_inst
                    .walker_type
                    .as_ref()
                    .node_type
                    .as_ref()
                    .clone()
                    .type_code();
                let walker_inst_start_node = walker_inst.start_node.as_ref().varname.clone();
                println!(
                    "----Walker Instance:----\nStart Node: {}\nWalker Type: {}",
                    walker_inst_start_node, walker_inst_walker_type
                );
            }
        }
    }
}

fn write_to_file(file_name: &str, sem: &SemanticGlobal) -> Result<()> {
    let mut output_file = fs::File::create(file_name)?;

    writeln!(output_file, "// Generated C++ code")?;
    writeln!(output_file, "#include <cstdint>")?;
    writeln!(output_file, "\nusing namespace std;\n")?;


    writeln!(output_file, "// Struct definitions for nodes")?;
    for graph in &sem.graphs {
        for node_inst in &graph.node_insts {
            writeln!(output_file, "{};\n", node_inst.node_type.type_code())?;
        }
    }

    writeln!(output_file, "// Struct definitions for edges")?;
    for (_edge_name, edge) in &sem.edges {
        writeln!(output_file, "{};\n", edge.type_code())?;
    }

    writeln!(output_file, "// Struct definitions for walkers")?;
    for (_walker_name, walker) in &sem.walkers {
        writeln!(output_file, "using {} = {};", walker.name, walker.node_type.name)?;
    }

    writeln!(output_file, "\nint main() {{")?;

    
    for graph in &sem.graphs {
        writeln!(output_file, "// Instantiate nodes")?;
        for node_inst in &graph.node_insts {
            writeln!(
                output_file,
                "\t{} {};",
                node_inst.node_type.name, node_inst.varname
            )?;
        }

        writeln!(output_file)?;

        writeln!(output_file, "// Instantiate edges")?;
        for edge_inst in &graph.edge_insts {
            let edge_name = format!("{}_{}", edge_inst.from_var.varname, edge_inst.to_var.varname);
            let edge_type_name = &edge_inst.edge_type.named_block.name;
            let from_var = &edge_inst.from_var.varname;
            let to_var = &edge_inst.to_var.varname;
            let weight = edge_inst.weight;
            
            writeln!(output_file, "\t{} {};", edge_type_name, edge_name)?;
            writeln!(output_file, "\t{}.weight = {};", edge_name, weight)?;
            writeln!(output_file, "\t{}.from = {};", edge_name, from_var)?;
            writeln!(output_file, "\t{}.to = {};", edge_name, to_var)?;
            writeln!(output_file)?;
        }


        writeln!(output_file, "// Instantiate walkers")?;
        for walker_inst in &graph.walker_insts {
            writeln!(
                output_file,
                "\t{} walker_on_{};",
                walker_inst.walker_type.name, walker_inst.start_node.varname
            )?;
        }
    }

    writeln!(output_file, "\treturn 0;")?;
    writeln!(output_file, "}}")?;

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let file_content = fs::read_to_string(args.file)?;
    let sem = semantic_analysis(parse_str(&file_content)?)?;

    print_info(sem.clone());

    write_to_file(&args.output, &sem).ok();

    Ok(())
}
