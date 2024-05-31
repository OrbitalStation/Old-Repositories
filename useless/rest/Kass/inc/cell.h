#pragma once

#include "types.h"

enum Cell: u8 {
    Alive,  /* There's creature in this cell(it should be 'Creature', but there is class with the same name) */
    Food,   /* There's food in this cell */
    Wall,   /* There's wall in this cell */
    Poison, /* There's poison in this cell */
    Void,   /* There's nothing int his cell */

    MaxCell /* Number of cell types */
};
