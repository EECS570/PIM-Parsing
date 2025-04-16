#include <iostream>
#include <dpu.h>

Scan init_walker() {
  Scan walker;
  walker.sum = 0;
  return walker;
}


int kernel_dpu(Scan *walker, Data * here, Regular * edges, unsigned int edge_num) {
  for (unsigned int i = 0; i < edge_num; i++) {
    if (edges[i].src == here->id) {
      here->sum[i] = walker->sum;
      walker->sum += edges[i].weight;
    }
  }
  if (edge_num > 0) {
    return 0;
  } else {
    return -1;
  }
}