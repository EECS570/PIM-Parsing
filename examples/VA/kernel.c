#include <stdint.h>

int kernel_dpu(Adder * walker, Data *here, Next * edges, unsigned int edge_num) {
    for (unsigned int i = 0; i < 128; i++){
        here.vec1[i] += here.vec2[i];
    }
    if (edge_num > 0) {
        return 0;
    } else {
        return -1;
    }
}
