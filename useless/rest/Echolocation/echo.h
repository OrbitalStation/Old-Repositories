#pragma once
#include <SFML/Audio.hpp>
#include <list>

class Echo {

public:

    struct Node {
        sf::Sound sound;
        sf::Clock clock;
        bool launched;
    };

public:

    Echo();

    void add_sound(sf::Sound sound);

    void shrink();

private:

    std::list <Node> sounds;

};
