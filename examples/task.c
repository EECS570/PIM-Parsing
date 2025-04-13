// Generated C code
#include <stdint.h>
#include <stdio.h>
#include <defs.h>
#include <mram.h>
#include <alloc.h>
#include <perfcounter.h>
#include <barrier.h>
#include "kernel.c"

#include <../support/common.h>
__host dpu_arguments_t DPU_INPUT_ARGUMENTS;

BARRIER_INIT(my_barrier, NR_TASKLETS);

extern int main_kernel1(void);

int (*kernels[nr_kernels])(void) = {main_kernel1};

int main(void) {
	return kernels[DPU_INPUT_ARGUMENTS.kernel](); 
}

int main_kernel1() {
	unsigned int tasklet_id = me();
	if (tasklet_id == 0){mem_reset();}

	barrier_wait(&my_barrier);

	uint32_t input_size_dpu_bytes = DPU_INPUT_ARGUMENTS.size; // Input size per DPU in bytes
	uint32_t input_size_dpu_bytes_transfer = DPU_INPUT_ARGUMENTS.transfer_size; // Transfer input size per DPU in bytes

	// Address of the current processing block in MRAM
	uint32_t base_tasklet = tasklet_id << BLOCK_SIZE_LOG2;
	int i = 0;
	uint32_t mram_base_addr_alice_vec1 = (uint32_t)(DPU_MRAM_HEAP_POINTER + input_size_dpu_bytes_transfer * i);
	i ++;
	uint32_t mram_base_addr_alice_vec2 = (uint32_t)(DPU_MRAM_HEAP_POINTER + input_size_dpu_bytes_transfer * i);
	i ++;

	// Initialize a local cache to store the MRAM block
	int32_t *cache_alice_vec1 = (int32_t *) mem_alloc(BLOCK_SIZE);
	int32_t *cache_alice_vec2 = (int32_t *) mem_alloc(BLOCK_SIZE);
	for(unsigned int byte_index = base_tasklet; byte_index < input_size_dpu_bytes; byte_index += BLOCK_SIZE * NR_TASKLETS){

		uint32_t l_size_bytes = (byte_index + BLOCK_SIZE >= input_size_dpu_bytes) ? (input_size_dpu_bytes - byte_index) : BLOCK_SIZE;

		mram_read((__mram_ptr void const*)(mram_base_addr_alice_vec1 + byte_index), cache_alice_vec1, l_size_bytes);
		mram_read((__mram_ptr void const*)(mram_base_addr_alice_vec2 + byte_index), cache_alice_vec2, l_size_bytes);
		kernel_dpu(
			cache_alice_vec1,
			cache_alice_vec2, 
			l_size_bytes >> DIV);

		mram_write(cache_alice_vec2, (__mram_ptr void*)(mram_base_addr_alice_vec2 + byte_index), l_size_bytes);

	}

return 0;
}
