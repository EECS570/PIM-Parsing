NUM_OF_NODES = 100
SIZE_OF_WALKER_BYTES = 4
NUM_DPUS = 40
MAX_NODES_PER_DPU = 10

import z3

def network_generator() -> list[list[int]]:
  import random
  res: list[list[int]] = []
  for i in range(NUM_OF_NODES):
    res.append([])
  for i in range(NUM_OF_NODES):
    # Connect node i to a random node
    # Ensure that the node is not connected to itself
    # and that the connection does not already exist

    available_nodes = [j for j in range(NUM_OF_NODES) if j != i and j not in res[i]]
    if not available_nodes:
      continue
    node_id = random.choices(available_nodes, k=random.randint(1, max(3, len(available_nodes))))
    for j in node_id:
      if j not in res[i]:
        res[i].append(j)

  return res
    


def random_dpu_scheduling() -> list[list[int]]:
  import random
  res: list[list[int]] = []
  for i in range(NUM_DPUS):
    res.append([])
  for i in range(NUM_OF_NODES):
    # Randomly assign a node to a DPU
    # Ensure that the DPU has not reached its maximum capacity
    available_core = [i for i in range(NUM_DPUS) if len(res[i]) < MAX_NODES_PER_DPU]
    dpu_id = random.choice(available_core)
    res[dpu_id].append(i)
  return res

def z3_scheduling(network: list[list[int]]) -> list[list[int]]:
  s = z3.Optimize()
  nodes = [z3.Int(f"node_{i}") for i in range(NUM_OF_NODES)]
  node_constraints = z3.And([z3.And(nodes[i] >= 0, nodes[i] < NUM_DPUS) for i in range(NUM_OF_NODES)])
  # dpu_sizes = [0] * NUM_DPUS

  # for i in range(NUM_DPUS):
  #   dpu_sizes[i] = z3.Sum([z3.If(nodes[j] == i, 1, 0) for j in range(NUM_OF_NODES)])
  dpu_sizes = [z3.Sum([z3.If(nodes[j] == i, 1, 0) for j in range(NUM_OF_NODES)]) for i in range(NUM_DPUS)]
  dpu_size_constraints = z3.And([z3.And(dpu_sizes[i] <= MAX_NODES_PER_DPU) for i in range(NUM_DPUS)])

  s.add(node_constraints)
  s.add(dpu_size_constraints)

  expr = 0
  for i in range(NUM_OF_NODES):
    for j in network[i]:
      # Ensure that the nodes are not on the same DPU
      expr += z3.If(nodes[i] == nodes[j], 0, 1)
      
  s.minimize(expr)

  if s.check() == z3.sat:
    model = s.model()
    res = []
    for i in range(NUM_DPUS):
      res.append([])
    for i in range(NUM_OF_NODES):
      dpu_id = model[nodes[i]].as_long()
      res[dpu_id].append(i)
    return res
  
  



def bfs_walk(network: list[list[int]], start_node: int, max_depth: int) -> list[int]:
  from collections import deque
  visited = []
  queue = deque([(start_node, 0)])  # (node, depth)
  visited.append(start_node)
  while queue:
    node, depth = queue.popleft()
    if depth < max_depth:
      for neighbor in network[node]:
        if neighbor not in visited:
          visited.append(neighbor)
          queue.append((neighbor, depth + 1))
  return visited


def evaluation(scheduling: list[list[int]], path: list[int]) -> int:
  # Calculate the number of jumps between nodes that are not on the same DPU
  node_to_dpu = {}
  for dpu_id, nodes in enumerate(scheduling):
    for node in nodes:
      node_to_dpu[node] = dpu_id
  jumps = 0
  for i in range(len(path) - 1):
    if node_to_dpu[path[i]] != node_to_dpu[path[i + 1]]:
      jumps += 1
  return jumps

def main():
  network = network_generator()
  scheduling = random_dpu_scheduling()
  z3_scheduling_result = z3_scheduling(network)
  print(f"Random DPU Scheduling: {scheduling}")
  start_node = 0
  max_depth = 3
  path = bfs_walk(network, start_node, max_depth)
  jumps = evaluation(scheduling, path)
  z3_jumps = evaluation(z3_scheduling_result, path)
  print(f"Jump: {jumps}")
  print(f"Z3 Jump: {z3_jumps}")

main()