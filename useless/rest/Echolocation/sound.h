#pragma once
#include <cstdio>

#define LOAD_SOUND(SOUND, FILE) \
    if (!SOUND ## _sound_buf.loadFromFile(FILE ".wav")) { \
        fprintf(stderr, "Failed to load " FILE " sound; exiting..."); \
        std::exit(-1); \
    } \
    SOUND ## _sound.setBuffer(SOUND ## _sound_buf)
