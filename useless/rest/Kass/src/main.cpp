#pragma ide diagnostic ignored "cert-err58-cpp"
#pragma ide diagnostic ignored "cppcoreguidelines-narrowing-conversions"

#include <SFML/Graphics.hpp>
#include <cstring>
#include <iostream>
#include <fstream>

#include "../inc/field.h"
#include "../inc/creature.h"
#include "../inc/random.h"

void save_on_exit(int, void *);
void load(const char *file);

int main(int argc, char *argv[]) {
    bool loaded = false;
    if (argc > 1) {
        if (strcmp(argv[1], "help") == 0) {
            if (argc != 2) {
                std::cerr << "Too many options!" << std::endl;
                return 1;
            }
            std::cout << "Commands:\n\thelp: show this info and exit.\n\t"
                         "save <file>: saves simulation to <file> before exit.\n\t"
                         "load <file>: loads simulation from <file>.\n";
            return 0;
        } else if (strcmp(argv[1], "save") == 0) {
            if (argc == 2) {
                std::cerr << "<file> is not specified!" << std::endl;
                return 1;
            } else if (argc > 3) {
                std::cerr << "Too many options!" << std::endl;
                return 1;
            }
            on_exit(save_on_exit, (void *)argv[2]);
        } else if (strcmp(argv[1], "load") == 0) {
            if (argc == 2) {
                std::cerr << "<file> is not specified!" << std::endl;
                return 1;
            } else if (argc > 3) {
                std::cerr << "Too many options!" << std::endl;
                return 1;
            }
            load(argv[2]);
            loaded = true;
        }
    }
    sf::RenderWindow w(sf::VideoMode(width * 20, height * 20), "Kass");
    sf::Event e{};
    sf::Texture texture;
    sf::Sprite sprite;

    texture.loadFromFile("../tile.png");
    sprite.setTexture(texture);

    if (!loaded) field_fill();

    while (w.isOpen()) {
        while (w.pollEvent(e)) if (e.type == sf::Event::Closed) w.close();

        std::shuffle(creatures.begin(), creatures.end(), mersenne);

        for (u16 i = 0; i < capacity; ++i) std::for_each(creatures.begin(), creatures.end(), eval_cmd);

        for (u8 x = 0, y; x < width; ++x) {
            for (y = 0; y < height; ++y) {
                sprite.setTextureRect(sf::IntRect(field[x][y] * 20, 0, 20, 20));
                sprite.setPosition(x * 20, y * 20);
                w.draw(sprite);
            }
        }
        w.display();
    }
}

void save_on_exit(int, void *file) {
    std::ofstream f((char *)file);
    f.write(reinterpret_cast <const char *> (field), sizeof(field));
    if (f.bad()) {
        std::cerr << "Oops, seems like an error occurred while trying to save data!" << std::endl;
        f.close();
        return;
    }
    f << creatures.size();
    for (auto &i: creatures) {
        f << i.pos.x << i.pos.y;
        f << i.health;
        f << (u8)i.dir;
        f.write(reinterpret_cast <const char *> (i.code), capacity);
        f << i.regs.ip << i.regs.ax << i.regs.fp;
    }
    f.close();
}

void load(const char *file) {
    std::ifstream f(file);
    f.read(reinterpret_cast <char *> (field), sizeof(field));
    if (f.bad()) {
        std::cerr << "Oops, seems like an error occurred while trying to save data!" << std::endl;
        f.close();
        return;
    }
    u64 size;
    f >> size;
    creatures.resize(size);
    for (auto &i: creatures) {
        f >> i.pos.x >> i.pos.y;
        f >> i.health;
        f >> (u8 &)i.dir;
        f.read(reinterpret_cast <char *> (i.code), capacity);
        f >> i.regs.ip >> i.regs.ax >> i.regs.fp;
    }
    f.close();
}
