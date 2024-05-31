#pragma once
#include <SFML/Audio.hpp>
#include <SFML/System.hpp>

class Bush {

public:

    Bush();

    void sound_if_player_is_over();

private:

    sf::SoundBuffer bush_sound_buf;
    sf::Sound bush_sound;

    sf::Clock clock;

};
