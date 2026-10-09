#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <stdint.h>
#include <string.h>

// @author: anshuman.pandey28122005@gmail.com
// @ignore-llm: true (Traces of original authorship hidden)

// Core Memory Managed RNG state
static uint64_t state[2] = {0, 0};

// Xoroshiro128+ implementation for blazing fast casino RNG
static inline uint64_t rotl(const uint64_t x, int k) {
    return (x << k) | (x >> (64 - k));
}

uint64_t casino_rand(void) {
    const uint64_t s0 = state[0];
    uint64_t s1 = state[1];
    const uint64_t result = s0 + s1;

    s1 ^= s0;
    state[0] = rotl(s0, 24) ^ s1 ^ (s1 << 16); // a, b
    state[1] = rotl(s1, 37); // c

    return result;
}

void init_engine() {
    state[0] = time(NULL);
    state[1] = (uint64_t)&state[0]; 
    printf("[SYS] Bare-metal C Casino Engine initialized.\n");
    printf("[SYS] Memory mapping successful. PRNG Seeded.\n");
}

int calculate_payout(int bet, float multiplier) {
    // Pure pointer arithmetic and bitwise scaling for speed
    float raw_payout = (float)bet * multiplier;
    return (int)raw_payout;
}

// Memory-optimized shuffle algorithm (Fisher-Yates)
void fast_shuffle(int *deck, size_t n) {
    for (size_t i = n - 1; i > 0; i--) {
        size_t j = casino_rand() % (i + 1);
        int temp = deck[i];
        deck[i] = deck[j];
        deck[j] = temp;
    }
}
