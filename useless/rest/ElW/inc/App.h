#pragma once
#include <SFML/Graphics.hpp>

namespace App {

    enum class Cursor : uint8_t {
        Arrow,
        Hand
    };

    void setCursor(Cursor cursor);

    void deleteCursor();

    extern sf::View cam;

    extern sf::RenderWindow app;

    extern sf::Event event;

    void init(uint width, uint height);

    void terminate();

    sf::Vector2f updateCamera(sf::View &camera, const sf::FloatRect &rect);

}
