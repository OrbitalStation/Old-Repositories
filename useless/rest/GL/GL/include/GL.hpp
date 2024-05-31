#ifndef _GL_LIBRARY_HPP
#define _GL_LIBRARY_HPP

#include "Window.hpp"
#include "Keyboard.hpp"

namespace gl {

    extern bool Init() _GL_CXX11_NOEXCEPT;

    extern void Terminate() _GL_CXX11_NOEXCEPT;

    struct Version {
        static
#if __cplusplus >= 201103L
        constexpr
#else
        const
#endif
        unsigned int major = 1, minor = 0;
    };

}

#endif /* _GL_LIBRARY_HPP */
