#include <SFML/Graphics.hpp>
#include "player.h"
#include "level.h"
#include "model.h"

int main() {
    sf::RenderWindow window(sf::VideoMode::getDesktopMode(), "Echolocation");

    sf::Image image;
    image.loadFromFile("icon.png");
    window.setIcon(image.getSize().x, image.getSize().y, image.getPixelsPtr());

    Player::init();
    Model::init(window);
    LevelContainer::init();

    sf::Event event;

    while (window.isOpen()) {

        while (window.pollEvent(event))
            if (event.type == sf::Event::Closed)
                window.close();
        
        if (sf::Keyboard::isKeyPressed(sf::Keyboard::W)) Player::move_forward();
        else if (sf::Keyboard::isKeyPressed(sf::Keyboard::A)) Player::move_left();
        else if (sf::Keyboard::isKeyPressed(sf::Keyboard::S)) Player::move_backward();
        else if (sf::Keyboard::isKeyPressed(sf::Keyboard::D)) Player::move_right();

        if (sf::Keyboard::isKeyPressed(sf::Keyboard::Left)) Player::rotate_left();
        else if (sf::Keyboard::isKeyPressed(sf::Keyboard::Right)) Player::rotate_right();

        (*LevelContainer::current)->frame();

        Model::draw(window);

    }

    LevelContainer::terminate();

}
