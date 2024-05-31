#pragma once
#include <SFML/Graphics.hpp>
#include "../inc/Map.h"

class Interface {

public:

    static void init(uint width, uint height);

    static void draw();

    static void updateCams();

private:

    static sf::View mainCam, blockListCam;

    static sf::Vector2f offset;

    static float distance;

    static sf::RectangleShape bottom, areaOfCurrent;

    static Map::Type current;

};
