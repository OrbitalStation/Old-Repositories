#pragma once

#include <SFML/System/Vector2.hpp>
#include <vector>
#include <thread>

#include "direction.h"
#include "cell.h"

#define capacity      256 /* Capacity of buffer that contains creature's code */
#define poison_damage 100 /* Receiving damage after stepping on poison */
#define food_heal     50  /* Healing health after eating food */
#define max_health    255 /* Max health */
#define beat_power    10  /* Health decreasing after creature was beating by other creature */

class Creature {

public:

    /*
     * x position <-- [0; width)
     * y position <-- [0; height)
     */
    sf::Vector2 <u8> pos;

    /* health [0; Max_health]  */
    u8 health;

    /* where looks */
    Direction dir;

    /* brains */
    u8 code[capacity];

    /* registers */
    struct {
        u8 ip; /* instruction pointer */
               /* u8 xp <-- pos.x */
               /* u8 yp <-- pos.y */
               /* u8 hp <-- health */
        u8 ax; /* auxiliary register */
        u8 fp; /*
                * flags register:
                * 0 - OF - overflow flag
                * 1 - AF - auxiliary flag
                * 2 - ZF - zero flag
                * 3 - DF - direction flag
                */
    } regs;

    Creature() = default;

    /* Constructor */
    Creature(u8 x, u8 y);

    /* Returns x position of cell in front */
    u8 x() const;

    /* Returns y position of cell in front */
    u8 y() const;

    /* Moves creature to cell in front */
    void move();

    /* Kills creature */
    void kill();

    /* Takes damage */
    void damage(u8 damage);

    /* Beats creature on cell in front */
    void beat() const;

    /* Eats food on cell in front */
    void eat();

    /* Changes type of cell in front */
    void set(Cell type) const;

    /* Is creature alive(health > 0)? */
    bool alive() const;

    /* Returns type of cell in front */
    Cell type() const;

private:

    void move_case(u8 x, u8 y);
};

/* List of all creatures */
extern std::vector <Creature> creatures;

void eval_cmd(Creature &creature);
