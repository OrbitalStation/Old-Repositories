#include "../inc/random.h"
#include "../inc/field.h"
#include "../inc/creature.h"

field_t field;

void field_fill() {
    Cell cell;
    u64 i;
    bool found;
    for (u8 x = 0, y; x < width; ++x) {
        for (y = 0; y < height; ++y) {
            cell = Cell(mersenne() % MaxCell);
            if (cell == Alive) {
                for (i = 0, found = false; i < creatures.size(); ++i) {
                    if (!creatures[i].alive()) {
                        creatures[i] = Creature(x, y);
                        found = true;
                        break;
                    }
                }
                if (!found) creatures.emplace_back(x, y);
            }
            field[x][y] = cell;
        }
    }
}
