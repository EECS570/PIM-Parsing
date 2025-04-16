#include <iostream>
#include <dpu.h>

Scan init_walker() {
  Scan walker;
  walker.sum = 0;
  return walker;
}


int kernel_dpu(Scan *walker, Data * here, Regular * edges, unsigned int edge_num, Special * special_edges, unsigned int special_edge_num) {
  for (unsigned int i = 0; i < edge_num; i++) {
    if (edges[i].src == here->id) {
      walker->sum += edges[i].weight;
    }
  }
  if (edge_num > 0) {
    return 0;
  }
  if (special_edge_num > 0) {
    return edge_num;
  }
  return -1;
}