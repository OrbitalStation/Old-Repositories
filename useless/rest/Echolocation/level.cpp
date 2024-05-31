#include "level.h"
#include <cstdint>
#include <cstdio>
#include "geometry.h"

/***********************************************/
//                  STATICS                    //
/***********************************************/

std::vector <ILevel *> LevelContainer::levels;
std::vector<ILevel *>::iterator LevelContainer::current;

/***********************************************/
//                 CONSTANTS                   //
/***********************************************/

const float FIRST_LEVEL_RADIUS_IN_METERS = 20;

const float FIRST_LEVEL_RADIUS_IN_METERS_SQUARED = FIRST_LEVEL_RADIUS_IN_METERS * FIRST_LEVEL_RADIUS_IN_METERS;

const sf::Vector2f FIRST_LEVEL_CENTER = {0, 0};

/***********************************************/
//                  METHODS                    //
/***********************************************/

void LevelContainer::init() {
    levels.push_back((ILevel *)new FirstLevel());
    current = levels.begin();
}

void LevelContainer::terminate() {
    for (size_t i = 0; i < levels.size(); ++i) delete levels[i];
    levels.clear();
}

bool FirstLevel::isInside(sf::Vector2f point) {
    return squared_distance(point, FIRST_LEVEL_CENTER) < FIRST_LEVEL_RADIUS_IN_METERS_SQUARED;
}

sf::Vector2f FirstLevel::try_move_and_keep_in_bounds(sf::Vector2f to, void (*on_bound)()) {
    if (to.x == 0) {
        if (to.y >= FIRST_LEVEL_RADIUS_IN_METERS) {
            to.y = FIRST_LEVEL_RADIUS_IN_METERS;
            on_bound();
        } else if (to.y <= -FIRST_LEVEL_RADIUS_IN_METERS) {
            to.y = -FIRST_LEVEL_RADIUS_IN_METERS;
            on_bound();
        }
    } else if (to.y == 0) {
        if (to.x > FIRST_LEVEL_RADIUS_IN_METERS) {
            to.x = FIRST_LEVEL_RADIUS_IN_METERS;
            on_bound();
        } else if (to.x < -FIRST_LEVEL_RADIUS_IN_METERS) {
            to.x = -FIRST_LEVEL_RADIUS_IN_METERS;
            on_bound();
        }
    } else if (!isInside(to)) {
        auto bybx = to.y / to.x;
        to.x = fast_invsqrt(bybx * bybx + 1) * FIRST_LEVEL_RADIUS_IN_METERS;
        to.y = to.x * bybx;
        on_bound();
    }

    return to;
}

void FirstLevel::frame() {
    drip.sound();
}

void FirstLevel::on_player_step() {
    bush.sound_if_player_is_over();
}
