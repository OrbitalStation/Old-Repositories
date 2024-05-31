#include "echo.h"
#include <cstdio>
#include <SFML/Window.hpp>

const sf::Time ECHO_ATTENUATION_DELAY = sf::seconds(0.1);
const float ECHO_ATTENUATION_POWER = 0.5;

Echo::Echo(): sounds() {}

void Echo::add_sound(sf::Sound sound) {
    sounds.push_back({ sound, sf::Clock(), false });
    sounds.back().sound.setVolume(sounds.back().sound.getVolume() / 5.f);
    
    sounds.back().sound.play();
}

void Echo::shrink() {
    auto it = sounds.begin();
    while (it != sounds.end()) {
        if (it->clock.getElapsedTime().asSeconds() >= 0.2) {
            if (it->sound.getStatus() != sf::Sound::Playing) {
                if (it->launched) {
                    it = sounds.erase(it);
                    continue;
                } else {
                    it->sound.play();
                    it->launched = true;
                }
            }
        }
        // if (it->clock.getElapsedTime().asSeconds() >= 0.2) {
        //     switch (it->sound.getStatus()) {
        //         case sf::Sound::Playing:

        //             break;

        //         case sf::Sound::Paused:
        //             break;
                
        //         case sf::Sound::Stopped:
        //             it = sounds.erase(it);
        //             continue;
        //     }
        // }
        ++it;
    }

    // size_t i = 0;
    // size_t len = sounds.size();
    // while (i < len) {
    //     auto it = sounds.begin();

    // auto it = sounds.begin();
    // while (it != sounds.end()) {
    //     if (it->sound.getStatus() != sf::Sound::Playing) {
    //         it = sounds.erase(it);
    //     } else {
    //         if (it->clock.getElapsedTime() >= ECHO_ATTENUATION_DELAY) {
    //             it->clock.restart();
    //             it->sound.setVolume(it->sound.getVolume() * ECHO_ATTENUATION_POWER);
    //         }
    //         ++it;
    //     }
    // }

    //     if (sounds[i]->getStatus() != sf::Sound::Playing) {
    //         sounds.erase(sounds[i]);
    //         --len;
    //     } else {
    //         ++i;
    //     }
    // }


}
