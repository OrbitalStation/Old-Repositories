#include "geometry.h"

float squared_distance(sf::Vector2f a, sf::Vector2f b) {
    auto x = a.x - b.x;
    auto y = a.y - b.y;
    return x * x + y * y;
}

float fast_invsqrt(float x) {
    union {
		float    f;
		uint32_t i;
	} conv = { .f = x };
	conv.i  = 0x5f3759df - (conv.i >> 1);
	conv.f *= 1.5F - (x * 0.5F * conv.f * conv.f);
	return conv.f;
}
