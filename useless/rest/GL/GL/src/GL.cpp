#include <iostream>
#include <X11/Xlibint.h>
#include <X11/Xlib.h>
#include <X11/Xutil.h>
#include "../include/GL.hpp"

namespace gl {

    namespace priv {

        static Display *_disp;

        static bool _init = false;

        static int _def_screen;

        struct _window_data {
            GC gc;
            XWindowAttributes attributes;
        };

        namespace Keyboard {

            static struct _ {
                bool mode;
                unsigned int code;
            } keys[::gl::Keyboard::KeyCount];

            ::gl::Keyboard::Key translate(const unsigned int &key) {
                for (unsigned int i = ::gl::Keyboard::Unknown; i < ::gl::Keyboard::KeyCount; ++i)
                    if (key == keys[i].code) return static_cast <gl::Keyboard::Key> (i);
                return gl::Keyboard::Unknown;
            }

            static Bool keyboard_helper(Display *display, XEvent *event, XPointer) {
                if (event->type == KeyPress) keys[translate(event->xkey.keycode)].mode = true;
                else XPutBackEvent(display, event);
                return False;
            }

        }

    }

    bool Init() _GL_CXX11_NOEXCEPT {
        if (priv::_init) return true;
        if ((priv::_disp = XOpenDisplay(_GL_CXX11_NULLPTR)) == _GL_CXX11_NULLPTR) return false;
        priv::_def_screen = XDefaultScreen(priv::_disp);
        return (priv::_init = true);
    }

    void Terminate() _GL_CXX11_NOEXCEPT {
        if (!priv::_init) return;
        XCloseDisplay(priv::_disp);
        priv::_init = false;
    }

    void Window::create(const unsigned int &width, const unsigned int &height, const char * const &title) _GL_CXX11_NOEXCEPT {
        m_ID = XCreateSimpleWindow(priv::_disp,
                                   XRootWindow(priv::_disp, priv::_def_screen), 0, 0, width, height, 0,
                                   XBlackPixel(priv::_disp, priv::_def_screen),
                                   XBlackPixel(priv::_disp, priv::_def_screen));
        XSelectInput(priv::_disp, m_ID, ExposureMask | KeyPressMask); /* 33554341 */
        m_open = false;
        XTextProperty name;
        XStringListToTextProperty(const_cast <char **> (&title), 1, &name);
        XSetWMProperties(priv::_disp, m_ID, &name, &name, _GL_CXX11_NULLPTR, 0, _GL_CXX11_NULLPTR, _GL_CXX11_NULLPTR, _GL_CXX11_NULLPTR);
        m_data = new priv::_window_data;
        m_data->gc = XCreateGC(priv::_disp, m_ID, 0, _GL_CXX11_NULLPTR);
    }

    void Window::show() _GL_CXX11_NOEXCEPT {
        XMapWindow(priv::_disp, m_ID);
        XFlush(priv::_disp);
        m_open = true;
    }

    void Window::unshow() _GL_CXX11_NOEXCEPT {
        XUnmapWindow(priv::_disp, m_ID);
        m_open = false;
    }

    void Window::destroy() _GL_CXX11_NOEXCEPT {
        if (m_data == _GL_CXX11_NULLPTR) return;
        XDestroyWindow(priv::_disp, m_ID);
        XFreeGC(priv::_disp, m_data->gc);
        delete m_data;
        m_data = _GL_CXX11_NULLPTR;
    }

    void Window::collapse() const _GL_CXX11_NOEXCEPT {
        XIconifyWindow(priv::_disp, m_ID, priv::_def_screen);
    }

    void Window::setPosition(const int &x, const int &y) _GL_CXX11_NOEXCEPT {
        XMoveWindow(priv::_disp, m_ID, x, y);
    }

    void Window::setSize(const unsigned int &x, const unsigned int &y) _GL_CXX11_NOEXCEPT {
        XResizeWindow(priv::_disp, m_ID, x, y);
    }

    void Window::getPosition(int &x, int &y) _GL_CXX11_NOEXCEPT {
        XGetWindowAttributes(priv::_disp, m_ID, &m_data->attributes);
        x = m_data->attributes.x;
        y = m_data->attributes.y;
    }

    void Window::getSize(int &width, int &height) _GL_CXX11_NOEXCEPT {
        XGetWindowAttributes(priv::_disp, m_ID, &m_data->attributes);
        width = m_data->attributes.width;
        height = m_data->attributes.height;
    }

    void Keyboard::pollEvents() _GL_CXX11_NOEXCEPT {
        for (unsigned long int i = 0; i < KeyCount; ++i) priv::Keyboard::keys[i].mode = false;
        XEvent event;
        XCheckIfEvent(priv::_disp, &event, priv::Keyboard::keyboard_helper, _GL_CXX11_NULLPTR);
    }

    const bool& Keyboard::isKeyPressed(const Key &key) _GL_CXX11_NOEXCEPT {
        return priv::Keyboard::keys[key].mode;
    }

    void Keyboard::setKeymap(const unsigned int keys[KeyCount]) _GL_CXX11_NOEXCEPT {
        for (unsigned int i = Unknown; i < KeyCount; ++i)
            priv::Keyboard::keys[i].code = keys[i];
    }

    void Keyboard::setKeymapKey(const Key &key, const unsigned int &code) _GL_CXX11_NOEXCEPT { priv::Keyboard::keys[key].code = code; }

    void Keyboard::getKeymap(unsigned int keys[KeyCount]) _GL_CXX11_NOEXCEPT {
        for (unsigned int i = Unknown; i < KeyCount; ++i)
            keys[i] = priv::Keyboard::keys[i].code;
    }

    const unsigned int& Keyboard::getKeymapKey(const Key &key) _GL_CXX11_NOEXCEPT { return priv::Keyboard::keys[key].code; }

    void Keyboard::setDefaultKeymap() _GL_CXX11_NOEXCEPT {
        unsigned int codes[KeyCount] = {
                0, /* Special for 'Unknown' */
                65, 48, 59, 20, 60, 61, 19, 10, 11, 12,
                13, 14, 15, 16, 17, 18, 47, 21, 38, 56,
                54, 40, 26, 41, 42, 43, 31, 44, 45, 46,
                58, 57, 32, 33, 24, 27, 39, 28, 30, 55,
                25, 53, 29, 52, 34, 51, 35, 49, 9 , 36,
                23, 22, 118, 119, 111, 113, 114,   116,
                112, 117, 110, 115, 66, 78, 77,    127,
                67, 68, 69, 70, 71, 72, 73, 74, 75, 76,
                95, 96, 90, 87, 88, 89, 83, 84, 85, 79,
                80, 81, 91, 106, 82, 86, 104, 63,   50,
                37, 64, 62, 105, 108, 135
        };
        setKeymap(codes);
    }

}
