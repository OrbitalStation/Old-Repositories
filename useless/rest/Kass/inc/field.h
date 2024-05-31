#pragma once

#include "cell.h"

#define width  50 /* Width of field */
#define height 50 /* Height of field */

typedef Cell field_t[width][height];

extern field_t field;

/* Fill field with random cells */
void field_fill();
