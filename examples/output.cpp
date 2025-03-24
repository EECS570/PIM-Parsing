// Generated C++ code
#include <cstdint>

using namespace std;

// Struct definitions for nodes
typedef struct _Alice { 
	int8_t number1;
	char essay1 [800];
} Alice;

typedef struct _Bob { 
	int8_t number2;
	char essay2 [800];
} Bob;

typedef struct _Bob { 
	int8_t number2;
	char essay2 [800];
} Bob;

typedef struct _Charlie { 
	int8_t number;
} Charlie;

// Struct definitions for edges
typedef struct _Friend {
	int8_t weight;
	int8_t age;
	Alice from;
	Bob to;
} Friend;

// Struct definitions for walkers
using Worker1 = Alice;

int main() {
// Instantiate nodes
	Alice alice;
	Bob bob;
	Bob bob_2;
	Charlie charlie;

// Instantiate edges
	Friend alice_bob;
	alice_bob.weight = 10;
	alice_bob.from = alice;
	alice_bob.to = bob;

	Friend alice_bob_2;
	alice_bob_2.weight = 20;
	alice_bob_2.from = alice;
	alice_bob_2.to = bob_2;

// Instantiate walkers
	Worker1 walker_on_alice;
	return 0;
}
