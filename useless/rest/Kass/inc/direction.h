#pragma once

#include "types.h"

/* Clockwise */
enum Direction: u8 {
    Up,    /* Direction to one cell up    (--y) */
    Right, /* Direction to one cell right (++x) */
    Down,  /* Direction to one cell down  (++y) */
    Left   /* Direction to one cell left  (--x) */
};
