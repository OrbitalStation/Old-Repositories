#include "../inc/Interface.h"
#include "../inc/App.h"

#include <iostream>

sf::View Interface::mainCam, Interface::blockListCam;
sf::RectangleShape Interface::bottom, Interface::areaOfCurrent;
sf::Vector2f Interface::offset;
float Interface::distance;
Map::Type Interface::current;

void Interface::init(uint width, uint height) {
    mainCam.reset(sf::FloatRect(0, 0, width, height));
    blockListCam.reset(sf::FloatRect(0, 0, width, height));

    bottom.setSize(sf::Vector2f(width - 6, height /= 4));
    bottom.setPosition(3.f, height * 3 - 3);
    bottom.setOutlineColor(sf::Color::Black);
    bottom.setOutlineThickness(3.f);

    areaOfCurrent.setSize(sf::Vector2f(height, height));
    areaOfCurrent.setPosition(width - height - 3, height * 3 - 3);
    areaOfCurrent.setOutlineColor(sf::Color::Black);
    areaOfCurrent.setOutlineThickness(3.f);

    current = Map::TrueLast;
}

void Interface::draw() {
    App::app.setView(mainCam);
    App::app.draw(bottom);
    App::app.draw(areaOfCurrent);
    if (current != Map::TrueLast) {
        Map::sprites[current].setPosition(areaOfCurrent.getPosition());
        Map::sprites[current].setScale(10.f, 10.f);
        App::app.draw(Map::sprites[current]);
    }

    App::app.setView(blockListCam);

    for (uint i = 0, j; i < Map::LastChoose; i += 4) {
        for (j = 0; j < 4; ++j) {
            if (i + j == Map::LastChoose) break;
            Map::sprites[i + j].setScale(2.f, 2.f);
            Map::sprites[i + j].setPosition((i + 1) * 20 + offset.x, distance * j + offset.y);
            App::app.draw(Map::sprites[i + j]);
            Map::sprites[i + j].setScale(1.f, 1.f);
        }
    }
}

void Interface::updateCams() {
    offset = App::updateCamera(blockListCam, sf::FloatRect(0, 0, App::event.size.width, App::event.size.height));
    distance = 50 - offset.y / 10;
    offset.y += App::event.size.height / 4 * 3;
}
