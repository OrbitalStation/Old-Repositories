#ifndef _GL_WINDOW_HPP
#define _GL_WINDOW_HPP

#include "Config.hpp"

namespace gl {

    namespace priv {

        struct _window_data;

    }

    class Window {

    public:

        Window() _GL_CXX11_NOEXCEPT : m_ID(), m_data(_GL_CXX11_NULLPTR), m_open(false) { }

        _GL_INLINE Window(const unsigned int &width, const unsigned int &height, const char * const &title)
        _GL_CXX11_NOEXCEPT : m_ID(), m_data(), m_open() { create(width, height, title); }

        _GL_INLINE ~Window() _GL_CXX11_NOEXCEPT { destroy(); }

        void show() _GL_CXX11_NOEXCEPT;

        void unshow() _GL_CXX11_NOEXCEPT;

        void create(const unsigned int &width, const unsigned int &height, const char * const &title) _GL_CXX11_NOEXCEPT;

        void destroy() _GL_CXX11_NOEXCEPT;

        void collapse() const _GL_CXX11_NOEXCEPT;

        void setPosition(const int &x, const int &y) _GL_CXX11_NOEXCEPT;

        void setSize(const unsigned int &x, const unsigned int &y) _GL_CXX11_NOEXCEPT;

        void getPosition(int &x, int &y) _GL_CXX11_NOEXCEPT;

        void getSize(int &width, int &height) _GL_CXX11_NOEXCEPT;

        _GL_INLINE const bool &isShowed() const _GL_CXX11_NOEXCEPT { return m_open; }

        _GL_INLINE bool exist() const _GL_CXX11_NOEXCEPT { return m_data != _GL_CXX11_NULLPTR; }

        /* XWindowEvent */

    private:

        unsigned long int m_ID;

        priv::_window_data *m_data;

        bool m_open;

    };

}

#endif /* _GL_WINDOW_HPP */
