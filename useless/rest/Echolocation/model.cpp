#include "model.h"
#include <cmath>

#define private public
#include "player.h"
#undef private

sf::Texture Model::texture;

sf::Sprite Model::sprite;

bool Model::redraw;

void Model::init(sf::RenderWindow &window) {
    texture.loadFromFile("model.png");
    sprite.setTexture(texture);
    sprite.setPosition(window.getSize().x / 2, window.getSize().y / 2);
    sprite.setOrigin(texture.getSize().x / 2, texture.getSize().y / 2);
    redraw = true;
}

void Model::draw(sf::RenderWindow &window) {
    if (redraw) {
        window.clear(sf::Color(40, 41, 41));
        window.draw(sprite);
        window.display();
        redraw = false;
    }
}

void Model::setRotation(float radians) {
    sprite.setRotation(radians * 180 / M_PI);
    redraw = true;
}
