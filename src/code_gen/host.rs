use std::rc::Rc;

use crate::sem_type::{SemanticGraph, SemanticNodeInst};
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

pub fn main_function(graph: &SemanticGraph, core_num: u64, node_allocation: &Vec<Vec<Rc<SemanticNodeInst>>>) -> String {
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

    // Here is an example of the generated code
    //   int main() {
    //   struct dpu_set_t set, dpu;
    //   uint32_t checksum;

    //   DPU_ASSERT(dpu_alloc(1, NULL, &set));
    //   DPU_ASSERT(dpu_load(set, DPU_BINARY, NULL));
    //   populate_mram(set);

    //   DPU_ASSERT(dpu_launch(set, DPU_SYNCHRONOUS));
    //   DPU_FOREACH(set, dpu) {
    //     DPU_ASSERT(dpu_copy_from(dpu, "checksum", 0, (uint8_t *)&checksum, sizeof(checksum)));
    //     printf("Computed checksum = 0x%08x\n", checksum);
    //   }
    //   DPU_ASSERT(dpu_free(set));
    //   return 0;
    // }
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
    // Here is an example of the generated code
    //   struct dpu_set_t dpu;
    //   uint32_t each_dpu;
    //   uint8_t *buffer = malloc(BUFFER_SIZE * nr_dpus);
    //   DPU_FOREACH(set, dpu, each_dpu) {
    //     for (int byte_index = 0; byte_index < BUFFER_SIZE; byte_index++) {
    //       buffer[each_dpu * BUFFER_SIZE + byte_index] = (uint8_t)byte_index;
    //     }
    //     buffer[each_dpu * BUFFER_SIZE] += each_dpu; // each dpu will compute a different checksum
    //     DPU_ASSERT(dpu_prepare_xfer(dpu, &buffer[each_dpu * BUFFER_SIZE]));
    //   }
    //   DPU_ASSERT(dpu_push_xfer(set, DPU_XFER_TO_DPU, "buffer", 0, BUFFER_SIZE, DPU_XFER_DEFAULT));
    //   free(buffer);
    let 


    // Generate the main function
    let main = formatdoc! {
      r#"
    int main(int argc, char **argv) {{
      // Initialize the DPU
      {}
      // Initialize the nodes
      {}
      // Initialize the edges
      {}
      // Start the DPU
      dpu_start();
      // Wait for the DPU to finish
      dpu_wait();
      // Free the DPU
      dpu_free();
    }}
    "#,
      dpu_alloc,
      init_nodes,
      init_edges
    };
    main
}
