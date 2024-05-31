#include "drip.h"
#include "sound.h"
#include "geometry.h"

const sf::Time DRIP_DELAY = sf::seconds(3);

Drip::Drip(): position(), drip_sound(), drip_sound_buf(), clock() {
    LOAD_SOUND(drip, "drip");

    drip_sound.setMinDistance(2.);
}

void Drip::sound() {
    if (clock.getElapsedTime() >= DRIP_DELAY) {
        drip_sound.play();

        clock.restart();
    }
}
