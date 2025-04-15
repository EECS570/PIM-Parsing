use crate::code_gen::type_code::TypeCodeGen;
use crate::sem_type::SemanticGlobal;
use indoc::indoc;

pub fn includes_host() -> String {
    let header = indoc! {r#"
        // Generated C code
        #include <stdint.h>
        #include <stdio.h>
        #include <string.h>
        #include <stdlib.h>
        #include <stdbool.h>
        #include <dpu.h>
        #include <dpu_log.h>
        #include <unistd.h>
        #include <getopt.h>
        #include <assert.h>
        #include "kernel.c"

        #include <../support/common.h>
        #include <../support/timer.h>
        #include <../support/params.h>

        #ifndef DPU_BINARY
        #define DPU_BINARY "./dpu_binary.bin"
        #endif
    "#};
    String::from(header)
}

pub fn includes_dpu() -> String {
    let header = indoc! {r#"
        // Generated C code
        #include <stdint.h>
        #include <stdio.h>
        #include <defs.h>
        #include <mram.h>
        #include <alloc.h>
        #include <perfcounter.h>
        #include <barrier.h>
    "#};
    String::from(header)
}

pub fn includes_support() -> String {
    let header = indoc! {"
  #include <stdint.h>
  "};
    String::from(header)
}

pub fn shared_definitions(sem: &SemanticGlobal) -> String {
    let mut result = String::new();

    result.push_str(indoc! {"
    #ifndef SHARED_DEFINITIONS
    #define SHARED_DEFINITIONS
    "});
    result.push_str("// Struct definitions for nodes\n");
    for graph in &sem.graphs {
        for node_inst in &graph.node_insts {
            result.push_str(&format!("{};\n\n", node_inst.node_type.type_code()));
        }
    }

    result.push_str("// Struct definitions for edges\n");
    for (_edge_name, edge) in &sem.edges {
        result.push_str(&format!("{};\n\n", edge.type_code()));
    }

    result.push_str("// Struct definitions for walkers\n");
    for (_walker_name, walker) in &sem.walkers {
        result.push_str(&format!(
            "using {} = {};\n",
            walker.name, walker.node_type.name
        ));
    }

    result.push_str("#endif\n");

    result
}
