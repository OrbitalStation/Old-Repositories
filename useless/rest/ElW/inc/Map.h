#pragma once
#include <SFML/Graphics.hpp>
#include <vector>

class Map {

    friend class Interface;

public:

    enum Type {
        Wire,
        And,
        Or,
        Xor,

        LastChoose,

        WireOn = LastChoose,

        TrueLast
    };

    static void init(const char *textures);

    static void draw();

    //static void terminate();

    static void add(Type type, sf::Vector2f position);


private:

    static sf::Texture texture;
    static sf::Sprite sprites[TrueLast];
    static std::vector <sf::Vector2f> pos[TrueLast];

};
