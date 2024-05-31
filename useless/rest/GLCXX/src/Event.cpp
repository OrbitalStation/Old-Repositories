#include "../include/Window/Event.hpp"

extern "C++" {

    namespace gl {

        Event::Event() : type(nothing) { }

        void Event::clear() { type = nothing; }

    }

}
