#pragma ide diagnostic ignored "cert-err58-cpp"
#include "../inc/Map.h"
#include "../inc/App.h"

sf::Texture Map::texture;
sf::Sprite Map::sprites[TrueLast];
std::vector <sf::Vector2f> Map::pos[TrueLast];

void Map::init(const char *textures) {
    texture.loadFromFile(textures);
    for (uint i = 0; i < LastChoose; ++i) {
        sprites[i].setTexture(texture);
        sprites[i].setTextureRect(sf::IntRect(i * 20, 0, 20, 20));
    }
    for (uint i = LastChoose; i < TrueLast; ++i) {
        sprites[i].setTexture(texture);
        sprites[i].setTextureRect(sf::IntRect((i - LastChoose) * 20, 20, 20, 20));
    }
}

void Map::draw() {
    for (uint i = 0; i < TrueLast; ++i) {
        for (auto &j: pos[i]) {
            sprites[i].setPosition(j);
            App::app.draw(sprites[i]);
        }
    }
}

void Map::add(Type type, sf::Vector2f position) {
    pos[type].emplace_back(position);
}

//void Map::terminate() { }
