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
//PIM_Size: 512 byte
static int32_t* alice_vec2;
//PIM_Size: 512 byte
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


	const unsigned int input_size = p.exp == 0 ? p.input_size * nr_of_dpus : p.input_size; // Total input size (weak or strong scaling)

	const unsigned int input_size_8bytes = input_size;
	const unsigned int input_size_dpu_8bytes = divceil(input_size, nr_of_dpus);


	// Input/output allocation and initialization
	alice_vec1 = malloc(128 * nr_of_dpus * sizeof(int32_t));
	alice_vec1_val = malloc(128 * nr_of_dpus * sizeof(int32_t));
	int32_t* buffer_alice_vec1 = alice_vec1;
	alice_vec2 = malloc(128 * nr_of_dpus * sizeof(int32_t));
	alice_vec2_val = malloc(128 * nr_of_dpus * sizeof(int32_t));
	int32_t* buffer_alice_vec2 = alice_vec2;

	dpu_input_size_bytes = 512/nr_of_dpus;

	read_input(alice_vec1, 128);
	read_input(alice_vec2, 128);
	// Timer declaration
	Timer timer;

	printf("NR_TASKLETS\t%d\tBL\t%d\n", NR_TASKLETS, BL);

	for(int rep = 0; rep < p.n_warmup + p.n_reps; rep++) {
		// Compute output on CPU (performance comparison and verification purposes)
		if(rep >= p.n_warmup)
			start(&timer, 0, rep - p.n_warmup);
		kernel_host(
			alice_vec1_val,
			alice_vec2_val,
			input_size);

		if(rep >= p.n_warmup)
		stop(&timer, 0);

		printf("Load input data\n");
		if(rep >= p.n_warmup)
			start(&timer, 1, rep - p.n_warmup);
		// Input arguments
		unsigned int kernel = 0;
		dpu_arguments_t input_arguments[NR_DPUS];

		for(i=0; i<nr_of_dpus; i++) {
			input_arguments[i].size=dpu_input_size_bytes;
			input_arguments[i].transfer_size=dpu_input_size_bytes;
			input_arguments[i].kernel=kernel_dpu;
		}
		// Copy input arrays
		i = 0;
		DPU_FOREACH(dpu_set, dpu, i) { 
    		DPU_ASSERT(dpu_prepare_xfer(dpu, &input_arguments[i]));
		}
		DPU_ASSERT(dpu_push_xfer(dpu_set, DPU_XFER_TO_DPU, "DPU_INPUT_ARGUMENTS", 0, sizeof(input_arguments[0]), DPU_XFER_DEFAULT));

		 int last_loc = 0;
		DPU_FOREACH(dpu_set, dpu, i) { 
            		DPU_ASSERT(dpu_prepare_xfer(dpu, buffer_alice_vec1 + input_size_dpu_8bytes * i));
		}
		DPU_ASSERT(dpu_push_xfer(dpu_set, DPU_XFER_TO_DPU, DPU_MRAM_HEAP_POINTER_NAME, last_loc, dpu_input_size_bytes, DPU_XFER_DEFAULT));

		last_loc += dpu_input_size_bytes;

		DPU_FOREACH(dpu_set, dpu, i) { 
            		DPU_ASSERT(dpu_prepare_xfer(dpu, buffer_alice_vec2 + input_size_dpu_8bytes * i));
		}
		DPU_ASSERT(dpu_push_xfer(dpu_set, DPU_XFER_TO_DPU, DPU_MRAM_HEAP_POINTER_NAME, last_loc, dpu_input_size_bytes, DPU_XFER_DEFAULT));


		if(rep >= p.n_warmup) 
			stop(&timer, 1);
		printf("Run program on DPU(s)\n");
		// Run DPU kernel
		if(rep >= p.n_warmup) {
			start(&timer, 2, rep - p.n_warmup);}
		DPU_ASSERT(dpu_launch(dpu_set, DPU_SYNCHRONOUS));
		if(rep >= p.n_warmup) {
			stop(&timer, 2);}


#if PRINT
        {
            unsigned int each_dpu = 0;
            printf("Display DPU Logs\n");
            DPU_FOREACH (dpu_set, dpu) {
                printf("DPU#%d:\n", each_dpu);
                DPU_ASSERT(dpulog_read_for_dpu(dpu.dpu, stdout));
                each_dpu++;
            }
        }
#endif

		printf("Retrieve results\n");
		if(rep >= p.n_warmup)
			start(&timer, 3, rep - p.n_warmup);
		i = 0;
		DPU_FOREACH(dpu_set, dpu, i) {
			DPU_ASSERT(dpu_prepare_xfer(dpu, alice_vec2_val + input_size_dpu_8bytes * i));}
		DPU_ASSERT(dpu_push_xfer(dpu_set, DPU_XFER_FROM_DPU, DPU_MRAM_HEAP_POINTER_NAME, last_loc, dpu_input_size_bytes, DPU_XFER_DEFAULT));
		if(rep >= p.n_warmup) stop(&timer, 3);

	}
	printf("CPU ");
	print(&timer, 0, p.n_reps);
	printf("CPU-DPU ");
	print(&timer, 1, p.n_reps);
	printf("DPU Kernel ");
	print(&timer, 2, p.n_reps);
	printf("DPU-CPU ");
	print(&timer, 3, p.n_reps);

	bool status = true;
	for (i = 0; i < input_size; i++) {
		if(alice_vec2[i] != buffer_alice_vec2[i])  {status = false;}
	}

	// Deallocation
	free(alice_vec1);
	free(alice_vec2);
	free(alice_vec1_val);
	free(alice_vec2_val);
	DPU_ASSERT(dpu_free(dpu_set));

	return status ? 0 : -1;
}
