#include "bush.h"
#include "sound.h"
#include "geometry.h"

#define private public
#include "player.h"
#undef private

const float BUSHES_RADIUS_OUTER = 10.0;
const float BUSHES_RADIUS_INNER_REMOVED = 3.0;

const float BUSHES_RADIUS_OUTER_SQUARED = BUSHES_RADIUS_OUTER * BUSHES_RADIUS_OUTER;
const float BUSHES_RADIUS_INNER_REMOVED_SQUARED = BUSHES_RADIUS_INNER_REMOVED * BUSHES_RADIUS_INNER_REMOVED;

const sf::Time BUSH_DELAY = sf::seconds(1.7);

Bush::Bush() {
    LOAD_SOUND(bush, "bushes");
    
    bush_sound.setRelativeToListener(true);
}

void Bush::sound_if_player_is_over() {
    if (clock.getElapsedTime() < BUSH_DELAY) return;

    auto dest = squared_distance({0, 0}, Player::pos);

    if (dest > BUSHES_RADIUS_INNER_REMOVED_SQUARED && dest < BUSHES_RADIUS_OUTER_SQUARED) {
        bush_sound.play();
        clock.restart();
    }
}
