#pragma once
#include <SFML/Audio.hpp>

class Drip {

public:

    Drip();

    void sound();

private:

    sf::Vector2f position;

    sf::Sound drip_sound;
    sf::SoundBuffer drip_sound_buf;

    sf::Clock clock;

};
