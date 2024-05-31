#pragma ide diagnostic ignored "cert-err58-cpp"

#include "../inc/random.h"

std::mt19937 mersenne(std::random_device().operator()());
