mod base_type;
mod code_gen;
mod parser;
mod sem_type;
mod semantics_analysis;
mod graph_cut;
use crate::parser::parse_str;
use anyhow::Result;
use base_type::PIMType;
use base_type::NamedBlock;
use clap::Parser;
use code_gen::TypeCodeGen;
use sem_type::SemanticGlobal;
use semantics_analysis::semantic_analysis;
use std::clone;
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

    writeln!(output_file, "// Generated C code")?;
    writeln!(output_file, "#include <stdint.h>")?;
    writeln!(output_file, "#include <stdio.h>")?;
    writeln!(output_file, "#include <kernel.c>")?;
    writeln!(output_file, "#include <string.h>\n")?;


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


fn write_to_app(file_name: &str, sem: &SemanticGlobal) -> Result<()> {
    let mut output_file = fs::File::create(file_name)?;

    // Write header includes.
    writeln!(output_file, "// Generated C code")?;
    writeln!(output_file, "#include <stdint.h>")?;
    writeln!(output_file, "#include <stdio.h>")?;
    writeln!(output_file, "#include <string.h>")?;
    writeln!(output_file, "#include <stdlib.h>")?;
    writeln!(output_file, "#include <stdbool.h>")?;
    writeln!(output_file, "#include <dpu.h>")?;
    writeln!(output_file, "#include <dpu_log.h>")?;
    writeln!(output_file, "#include <unistd.h>")?;
    writeln!(output_file, "#include <getopt.h>")?;
    writeln!(output_file, "#include <assert.h>")?;
    writeln!(output_file, "#include \"kernel.c\"\n")?;

    writeln!(output_file, "#include <common.h>")?;
    writeln!(output_file, "#include <timer.h>")?;
    writeln!(output_file, "#include <params.h>\n")?;

    // Define the DPU Binary path as DPU_BINARY here
    writeln!(output_file, "#ifndef DPU_BINARY")?;
    writeln!(output_file, "#define DPU_BINARY \"./dpu_binary.bin\"\n")?;
    writeln!(output_file, "#endif\n")?;

    // Struct definitions for nodes, edges, and walkers
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

    // Instantiate the graph
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

    // Pointer declaration
    writeln!(output_file, "// Pointer declaration")?;
    for node_inst in &sem.graphs[0].node_insts {
        for field in &node_inst.node_type.fields {
            if let PIMType::Array(base_type, _) = &field.pim_type {
                writeln!(output_file, "static {}* {}_{};", base_type.type_code(), node_inst.varname ,field.varname)?;
            }
        }
    }

    writeln!(output_file, "// Create input arrays TBD")?;

    // Generate the main function.
    writeln!(output_file, "\nint main(int argc, char **argv) {{\n")?;
    writeln!(output_file, "\tstruct Params p = input_params(argc, argv);\n")?;
    writeln!(output_file, "\tstruct dpu_set_t dpu_set, dpu;")?;
    writeln!(output_file, "\tuint32_t nr_of_dpus;\n")?;

    // Allocate DPUs and load binary
    writeln!(output_file, "\tDPU_ASSERT(dpu_alloc(NR_DPUS, NULL, &dpu_set));")?;
    writeln!(output_file, "\tDPU_ASSERT(dpu_load(dpu_set, DPU_BINARY, NULL));")?;
    writeln!(output_file, "\tDPU_ASSERT(dpu_get_nr_dpus(dpu_set, &nr_of_dpus));")?;
    writeln!(output_file, "\tprintf(\"Allocated %d DPU(s)\\n\", nr_of_dpus);")?;
    writeln!(output_file, "\tunsigned int i = 0;\n")?;
    writeln!(output_file, "\tconst unsigned int input_size_8bytes = p.input_size;")?;
    writeln!(output_file, "\tconst unsigned int input_size_dpu_8bytes = divceil(input_size, nr_of_dpus);")?;
    writeln!(output_file, "\n")?;

    writeln!(output_file, "\t// Input/output allocation and initialization")?;
    for node_inst in &sem.graphs[0].node_insts {
        for field in &node_inst.node_type.fields {
            if let PIMType::Array(base_type, _) = &field.pim_type {
                let node_name = node_inst.varname.clone();
                let field_name = field.varname.clone();
                let type_name = base_type.type_code().clone();
                let nr_elements = if let PIMType::Array(_, size) = &field.pim_type {
                    *size
                } else {
                    0
                };
                writeln!(output_file, "\t{}_{} = malloc({} * nr_of_dpus * sizeof({}));", node_name, field_name, nr_elements, type_name)?;
                writeln!(output_file, "\t{}* buffer_{}_{} = {}_{};", type_name, node_name, field_name, node_name, field_name)?;
            }
        }
    }

    // Input data
    for node_inst in &sem.graphs[0].node_insts {
        for field in &node_inst.node_type.fields {
            if let PIMType::Array(_, _) = &field.pim_type {
                let node_name = node_inst.varname.clone();
                let field_name = field.varname.clone();
                let nr_elements = if let PIMType::Array(_, size) = &field.pim_type {
                    *size
                } else {
                    0
                };
                writeln!(output_file, "\tread_input({}_{}, {});", node_name, field_name, nr_elements)?;
            }
        }
    }

    // Timer declaration
    writeln!(output_file, "\t// Timer declaration")?;
    writeln!(output_file, "\tTimer timer;\n")?;

    writeln!(output_file, "\tprintf(\"NR_TASKLETS\\t%d\\tBL\\t%d\\n\", NR_TASKLETS, BL);\n")?;
    

    // Loop over main kernel
    writeln!(output_file, "\tfor(int rep = 0; rep < p.n_warmup + p.n_reps; rep++) {{")?;
    writeln!(output_file, "\t\tprintf(\"Load input data\\n\");")?;
    writeln!(output_file, "\t\tif(rep >= p.n_warmup)")?;
    writeln!(output_file, "\t\t\tstart(&timer, 1, rep - p.n_warmup);")?;
    writeln!(output_file, "\t\t// Input arguments")?;
    writeln!(output_file, "\t\tunsigned int kernel = 0;")?;
    writeln!(output_file, "\t\tdpu_arguments_t input_arguments[NR_DPUS];\n")?;
    writeln!(output_file, "\t\tfor(i=0; i<nr_of_dpus-1; i++) {{")?;
    writeln!(output_file, "\t\t\t")?;
    writeln!(output_file, "\t\t\t")?;
    writeln!(output_file, "\t\t\t")?;
    writeln!(output_file, "\t\t}}")?;
    writeln!(output_file, "\t\t")?;
    writeln!(output_file, "\t\t")?;
    writeln!(output_file, "\t\t")?;
    writeln!(output_file, "\t\t")?;
    writeln!(output_file, "\t\t")?;
    writeln!(output_file, "\t\t")?;
    writeln!(output_file, "\t\t")?;
    writeln!(output_file, "\t\t")?;
    writeln!(output_file, "\t}}")?;
    

    writeln!(output_file, "\t// Instantiate node Alice")?;
    writeln!(output_file, "\tAlice alice;")?;
    writeln!(output_file, "\t// Initialize alice.vec1 and alice.vec2 (example: set all elements to 0)")?;
    writeln!(output_file, "\tmemset(alice.vec1, 0, sizeof(alice.vec1));")?;
    writeln!(output_file, "\tmemset(alice.vec2, 0, sizeof(alice.vec2));\n")?;
    writeln!(output_file, "\n")?;
    
    writeln!(output_file, "\t// Call kernel_host to compute: alice.vec2 = alice.vec1 + alice.vec2")?;
    writeln!(output_file, "\tkernel_host(alice.vec2, alice.vec1, alice.vec2, 128);\n")?;

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
    write_to_app("./examples/app.c", &sem).ok();
    Ok(())
}
