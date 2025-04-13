#include <stdint.h>

#define T int32_t


static void kernel_dpu(T *bufferB, T *bufferA, unsigned int l_size) {
    for (unsigned int i = 0; i < l_size; i++){
        bufferB[i] += bufferA[i];
    }
}


static void kernel_host(T* A, T* B, unsigned int nr_elements) {
    for (unsigned int i = 0; i < nr_elements; i++) {
        B[i] += A[i];
    }
}

// Create input arrays
static void read_input(T* A, unsigned int nr_elements) {
    srand(0);
    printf("nr_elements\t%u\t", nr_elements);
    for (unsigned int i = 0; i < nr_elements; i++) {
        A[i] = (T) (rand());
    }
}
