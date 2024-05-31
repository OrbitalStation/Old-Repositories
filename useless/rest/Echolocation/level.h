#pragma once
#include <SFML/System.hpp>
#include <vector>
#include "drip.h"
#include "bush.h"

class ILevel;

class LevelContainer {

public:

    static void init();

    static void terminate();

    static std::vector <ILevel *> levels;

    static std::vector<ILevel *>::iterator current;

};

class ILevel {

public:

    virtual bool isInside(sf::Vector2f point) = 0;

    virtual sf::Vector2f try_move_and_keep_in_bounds(sf::Vector2f to, void (*on_bound)()) = 0;

    virtual void frame() = 0;

    virtual void on_player_step() = 0;

    virtual ~ILevel() = default;

};

class FirstLevel : public ILevel {

public:

    virtual bool isInside(sf::Vector2f point) override;

    virtual sf::Vector2f try_move_and_keep_in_bounds(sf::Vector2f to, void (*on_bound)()) override;

    virtual void frame() override;

    virtual void on_player_step() override;

private:

    Drip drip;

    Bush bush;

};
