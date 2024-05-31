#pragma once
#include <SFML/Graphics.hpp>
#include <vector>

class Model {

public:

    static void init(sf::RenderWindow &window);

    static void draw(sf::RenderWindow &window);

    static void setRotation(float radians);

private:

    static sf::Texture texture;

    static sf::Sprite sprite;

    static bool redraw;

};
