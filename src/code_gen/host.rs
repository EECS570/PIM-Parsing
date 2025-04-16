use std::rc::Rc;

use crate::sem_type::{SemanticGlobal, SemanticGraph, SemanticNodeInst};
use indoc::{formatdoc, indoc};

pub fn initialization_declaration(graph: &SemanticGraph) -> String {
    // Generate h file code that askes the user to initialize the nodes and the edges
    let header = indoc! {"
  #include \"support.h\"
  "};
    // Declare a function that initializes nodes
    let nodes = graph
        .node_insts
        .iter()
        .map(|node| format!("{} {}_init();", node.node_type.name, node.varname))
        .collect::<Vec<String>>()
        .join("\n");

    // Declare a function that initializes edges
    let edges = graph
        .edge_insts
        .iter()
        .map(|edge| {
            format!(
                "{} {}_{}_init();",
                edge.edge_type.named_block.name, edge.from_var.varname, edge.to_var.varname
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    // Concatenate the header and the function declarations

    let result = format!("{}\n{}\n{}", header, nodes, edges);
    result
}

pub fn main_function(
    global: &SemanticGlobal,
    core_num: u64,
    core_node_allocation: &Vec<Vec<Rc<SemanticNodeInst>>>,
) -> String {
    let graph = &global.graphs[0];
    // Instantiate all the nodes
    let nodes = graph
        .node_insts
        .iter()
        .map(|node| format!("{} {}_inst;", node.node_type.name, node.varname))
        .collect::<Vec<String>>()
        .join("\n");
    // Instantiate all the edges
    let edges = graph
        .edge_insts
        .iter()
        .map(|edge| {
            format!(
                "{} {}_{}_inst;",
                edge.edge_type.named_block.name, edge.from_var.varname, edge.to_var.varname
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    // initialize the nodes
    let init_nodes = graph
        .node_insts
        .iter()
        .map(|node| format!("{} = {}_init();", node.varname, node.varname))
        .collect::<Vec<String>>()
        .join("\n");

    // initialize the edges
    let init_edges = graph
        .edge_insts
        .iter()
        .map(|edge| {
            format!(
                "{} = {}_{}_init();",
                edge.from_var.varname, edge.to_var.varname, edge.edge_type.named_block.name
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    let header = indoc! {"
    #include \"support.h\"
    #include \"dpu.h\"
    #include \"dpu_binary.h\"
    "};
    // Generate the DPU allocation code
    let dpu_alloc = formatdoc! {
      r#"
    struct dpu_set_t set, dpu;
    uint32_t checksum;

    DPU_ASSERT(dpu_alloc({}, NULL, &set));
    "#,
      core_num
    };

    // Populate the nodes to the MRAM
    // TODO

    // Start the DPU
    let dpu_start = formatdoc! {
      r#"
    // Load the DPU binary
    DPU_ASSERT(dpu_load(set, DPU_BINARY));
    // Launch
    DPU_ASSERT(dpu_launch(set, DPU_SYNCHRONOUS));
    "#
    };

    // Generate the main function
    let main = formatdoc! {
      r#"
    int main(int argc, char **argv) {{
      // Instantiate all the nodes
      {}
      // Instantiate all the edges
      {}
      // Initialize the DPU
      {}
      // Initialize the nodes
      {}
      // Initialize the edges
      {}
      // Start the DPU
      {}
    }}
    "#,
        nodes,
        edges,
      dpu_alloc,
      init_nodes,
      init_edges,
        dpu_start
    };
    main
}
