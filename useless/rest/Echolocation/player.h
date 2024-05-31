#pragma once
#include <SFML/System.hpp>
#include <SFML/Audio.hpp>

class Player {

public:

    static void init();

    static void move_forward();

    static void move_left();

    static void move_backward();

    static void move_right();

    static void update_angle(float degree);

    static void rotate_left();

    static void rotate_right();

private:

    static void step(const sf::Time &delay, sf::Vector2f pos);

private:

    /* Position in meters */
    static sf::Vector2f pos;

    static float angle;

    static float sine, cosine;

    static sf::Clock pause_steps_clock, rotation_clock;

    static sf::Sound step_sound, wall_bumped_sound;

    static sf::SoundBuffer step_sound_buf, wall_bumped_sound_buf;
};
