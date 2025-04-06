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

#include <common.h>
#include <timer.h>
#include <params.h>

#ifndef DPU_BINARY
#define DPU_BINARY "./dpu_binary.bin"

#endif

// Struct definitions for nodes
typedef struct _Alice { 
	int32_t vec1 [128];
	int32_t vec2 [128];
} Alice;

// Struct definitions for edges
// Struct definitions for walkers
// Instantiate nodes
	Alice alice;

// Instantiate edges
// Instantiate walkers
// Pointer declaration
static int32_t* alice_vec1;
static int32_t* alice_vec2;
// Create input arrays TBD

int main(int argc, char **argv) {

	struct Params p = input_params(argc, argv);

	struct dpu_set_t dpu_set, dpu;
	uint32_t nr_of_dpus;

	DPU_ASSERT(dpu_alloc(NR_DPUS, NULL, &dpu_set));
	DPU_ASSERT(dpu_load(dpu_set, DPU_BINARY, NULL));
	DPU_ASSERT(dpu_get_nr_dpus(dpu_set, &nr_of_dpus));
	printf("Allocated %d DPU(s)\n", nr_of_dpus);
	unsigned int i = 0;

	const unsigned int input_size_8bytes = p.input_size;
	const unsigned int input_size_dpu_8bytes = divceil(input_size, nr_of_dpus);


	// Input/output allocation and initialization
	alice_vec1 = malloc(128 * nr_of_dpus * sizeof(int32_t));
	int32_t* buffer_alice_vec1 = alice_vec1;
	alice_vec2 = malloc(128 * nr_of_dpus * sizeof(int32_t));
	int32_t* buffer_alice_vec2 = alice_vec2;
	read_input(alice_vec1, 128);
	read_input(alice_vec2, 128);
	// Timer declaration
	Timer timer;

	printf("NR_TASKLETS\t%d\tBL\t%d\n", NR_TASKLETS, BL);

	for(int rep = 0; rep < p.n_warmup + p.n_reps; rep++) {
		printf("Load input data\n");
		if(rep >= p.n_warmup)
			start(&timer, 1, rep - p.n_warmup);
		// Input arguments
		unsigned int kernel = 0;
		dpu_arguments_t input_arguments[NR_DPUS];

		
		
		
		
		
		
		
		
		
		
	}
	// Instantiate node Alice
	Alice alice;
	// Initialize alice.vec1 and alice.vec2 (example: set all elements to 0)
	memset(alice.vec1, 0, sizeof(alice.vec1));
	memset(alice.vec2, 0, sizeof(alice.vec2));



	// Call kernel_host to compute: alice.vec2 = alice.vec1 + alice.vec2
	kernel_host(alice.vec2, alice.vec1, alice.vec2, 128);

	return 0;
}
