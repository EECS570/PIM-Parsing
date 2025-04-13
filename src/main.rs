mod base_type;
mod code_gen;
mod parser;
mod sem_type;
mod semantics_analysis;
mod graph_cut;
use crate::parser::parse_str;
use anyhow::Result;
use base_type::{Size, PIMField, PIMType, PIMBaseType};
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

    writeln!(output_file, "#include <../support/common.h>")?;
    writeln!(output_file, "#include <../support/timer.h>")?;
    writeln!(output_file, "#include <../support/params.h>\n")?;

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

    let mut node_pointer_list = vec![];
    let mut node_pointer_val_list = vec![];
    let mut size_byte_dict = std::collections::HashMap::new();

    // Pointer declaration
    writeln!(output_file, "// Pointer declaration")?;
    for node_inst in &sem.graphs[0].node_insts {
        for field in &node_inst.node_type.fields {
            if let PIMType::Array(base_type, _) = &field.pim_type {
                writeln!(output_file, "static {}* {}_{};", base_type.type_code(), node_inst.varname ,field.varname)?;
                writeln!(output_file, "//PIM_Size: {} byte", field.size_byte())?;
                let node_name = node_inst.varname.clone();
                let field_name = field.varname.clone();
                let pointer_name = format!("{}_{}", node_name, field_name);
                node_pointer_list.push(format!("{}_{}", node_name, field_name));
                node_pointer_val_list.push(format!("{}_{}_val", node_name, field_name));
                size_byte_dict.insert(pointer_name, field.size_byte());
            }
        }
    }

    // Print the size of each pointer
    for (key, value) in &size_byte_dict {
        println!("Pointer: {}, Size: {} bytes", key, value);
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
    writeln!(output_file, "\tunsigned int i = 0;\n\n")?;

    writeln!(output_file, "\tconst unsigned int input_size = p.exp == 0 ? p.input_size * nr_of_dpus : p.input_size; // Total input size (weak or strong scaling)\n")?;

    writeln!(output_file, "\tconst unsigned int input_size_8bytes = input_size;")?;
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
                writeln!(output_file, "\t{}_{}_val = malloc({} * nr_of_dpus * sizeof({}));", node_name, field_name, nr_elements, type_name)?;
                writeln!(output_file, "\t{}* buffer_{}_{} = {}_{};", type_name, node_name, field_name, node_name, field_name)?;
            }
        }
    }

    let input_size_bytes = size_byte_dict.values().cloned().max().unwrap_or(0);
    println!("Maximum size in bytes: {}", input_size_bytes);
    writeln!(output_file, "\n\tdpu_input_size_bytes = {}/nr_of_dpus;\n", input_size_bytes)?;

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
    
    // compute on CPU
    writeln!(output_file, "\t\t// Compute output on CPU (performance comparison and verification purposes)")?;
    writeln!(output_file, "\t\tif(rep >= p.n_warmup)")?;
    writeln!(output_file, "\t\t\tstart(&timer, 0, rep - p.n_warmup);")?;
    writeln!(output_file, "\t\tkernel_host(")?;
    for (i, pointer_val) in node_pointer_val_list.iter().enumerate() {
        if i == node_pointer_val_list.len() - 1 {
            writeln!(output_file, "\t\t\t{},", pointer_val)?;
        } else {
            writeln!(output_file, "\t\t\t{},", pointer_val)?;
        }
    }
    writeln!(output_file, "\t\t\tinput_size);\n")?;
    writeln!(output_file, "\t\tif(rep >= p.n_warmup)\n\t\tstop(&timer, 0);\n")?;

    writeln!(output_file, "\t\tprintf(\"Load input data\\n\");")?;
    writeln!(output_file, "\t\tif(rep >= p.n_warmup)")?;
    writeln!(output_file, "\t\t\tstart(&timer, 1, rep - p.n_warmup);")?;
    writeln!(output_file, "\t\t// Input arguments")?;
    writeln!(output_file, "\t\tunsigned int kernel = 0;")?;
    writeln!(output_file, "\t\tdpu_arguments_t input_arguments[NR_DPUS];\n")?;
    writeln!(output_file, "\t\tfor(i=0; i<nr_of_dpus; i++) {{")?;
    writeln!(output_file, "\t\t\tinput_arguments[i].size=dpu_input_size_bytes;")?;
    writeln!(output_file, "\t\t\tinput_arguments[i].transfer_size=dpu_input_size_bytes;")?;
    writeln!(output_file, "\t\t\tinput_arguments[i].kernel=kernel_dpu;")?;
    writeln!(output_file, "\t\t}}")?;

    // Copy input arrays
    writeln!(output_file, "\t\t// Copy input arrays")?;
    writeln!(output_file, "\t\ti = 0;")?;
    writeln!(output_file, "\t\tDPU_FOREACH(dpu_set, dpu, i) {{ 
    \t\tDPU_ASSERT(dpu_prepare_xfer(dpu, &input_arguments[i]));\n\t\t}}")?;
    writeln!(output_file, "\t\tDPU_ASSERT(dpu_push_xfer(dpu_set, DPU_XFER_TO_DPU, \"DPU_INPUT_ARGUMENTS\", 0, sizeof(input_arguments[0]), DPU_XFER_DEFAULT));\n")?;
    
    writeln!(output_file, "\t\t int last_loc = 0;")?;
    for p in &node_pointer_list {
        writeln!(output_file, "\t\tDPU_FOREACH(dpu_set, dpu, i) {{ 
            \t\tDPU_ASSERT(dpu_prepare_xfer(dpu, buffer_{} + input_size_dpu_8bytes * i));\n\t\t}}", p)?;

        writeln!(output_file, "\t\tDPU_ASSERT(dpu_push_xfer(dpu_set, DPU_XFER_TO_DPU, DPU_MRAM_HEAP_POINTER_NAME, last_loc, dpu_input_size_bytes, DPU_XFER_DEFAULT));\n")?;
        if (p != node_pointer_list.last().unwrap()) {
            writeln!(output_file, "\t\tlast_loc += dpu_input_size_bytes;\n")?;
        }
    }

    writeln!(output_file, "\n\t\tif(rep >= p.n_warmup) \n\t\t\tstop(&timer, 1);")?;

    writeln!(output_file, "\t\tprintf(\"Run program on DPU(s)\\n\");")?;
    // Run DPU kernel
    writeln!(output_file, "\t\t// Run DPU kernel")?;

    writeln!(output_file, "\t\tif(rep >= p.n_warmup) {{\n\t\t\tstart(&timer, 2, rep - p.n_warmup);}}")?;

    writeln!(output_file, "\t\tDPU_ASSERT(dpu_launch(dpu_set, DPU_SYNCHRONOUS));")?;
    writeln!(output_file, "\t\tif(rep >= p.n_warmup) {{\n\t\t\tstop(&timer, 2);}}\n\n")?;

    writeln!(output_file, "#if PRINT")?;
    writeln!(output_file, "        {{")?;
    writeln!(output_file, "            unsigned int each_dpu = 0;")?;
    writeln!(output_file, "            printf(\"Display DPU Logs\\n\");")?;
    writeln!(output_file, "            DPU_FOREACH (dpu_set, dpu) {{")?;
    writeln!(output_file, "                printf(\"DPU#%d:\\n\", each_dpu);")?;
    writeln!(output_file, "                DPU_ASSERT(dpulog_read_for_dpu(dpu.dpu, stdout));")?;
    writeln!(output_file, "                each_dpu++;")?;
    writeln!(output_file, "            }}")?;
    writeln!(output_file, "        }}")?;
    writeln!(output_file, "#endif\n")?;
    
    writeln!(output_file, "\t\tprintf(\"Retrieve results\\n\");")?;
    writeln!(output_file, "\t\tif(rep >= p.n_warmup)\n\t\t\tstart(&timer, 3, rep - p.n_warmup);")?;
    writeln!(output_file, "\t\ti = 0;")?;

    writeln!(output_file, "\t\tDPU_FOREACH(dpu_set, dpu, i) {{\n\t\t\tDPU_ASSERT(dpu_prepare_xfer(dpu, {} + input_size_dpu_8bytes * i));}}", node_pointer_val_list.last().unwrap())?;
    writeln!(output_file, "\t\tDPU_ASSERT(dpu_push_xfer(dpu_set, DPU_XFER_FROM_DPU, DPU_MRAM_HEAP_POINTER_NAME, last_loc, dpu_input_size_bytes, DPU_XFER_DEFAULT));")?;
    writeln!(output_file, "\t\tif(rep >= p.n_warmup) stop(&timer, 3);\n")?;
    writeln!(output_file, "\t}}")?;
    

    writeln!(output_file, "\tprintf(\"CPU \");")?;
    writeln!(output_file, "\tprint(&timer, 0, p.n_reps);")?;
    writeln!(output_file, "\tprintf(\"CPU-DPU \");")?;
    writeln!(output_file, "\tprint(&timer, 1, p.n_reps);")?;
    writeln!(output_file, "\tprintf(\"DPU Kernel \");")?;
    writeln!(output_file, "\tprint(&timer, 2, p.n_reps);")?;
    writeln!(output_file, "\tprintf(\"DPU-CPU \");")?;
    writeln!(output_file, "\tprint(&timer, 3, p.n_reps);\n")?;


    writeln!(output_file, "\tbool status = true;")?;
    writeln!(output_file, "\tfor (i = 0; i < input_size; i++) {{")?;
    if let Some(last_pointer) = node_pointer_list.last() {
        writeln!(output_file, "\t\tif({}[i] != buffer_{}[i])  {{status = false;}}", last_pointer, last_pointer)?;
    }
    writeln!(output_file, "\t}}\n")?;

    // Deallocation
    writeln!(output_file, "\t// Deallocation")?;

    for p in &node_pointer_list {
        writeln!(output_file, "\tfree({});", p)?;
    }
    for p in &node_pointer_val_list {
        writeln!(output_file, "\tfree({});", p)?;
    }
    writeln!(output_file, "\tDPU_ASSERT(dpu_free(dpu_set));\n")?;

    writeln!(output_file, "\treturn status ? 0 : -1;")?;
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
