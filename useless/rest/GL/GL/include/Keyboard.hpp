#ifndef _GL_KEYBOARD_HPP
#define _GL_KEYBOARD_HPP

#include "Config.hpp"

namespace gl {

    struct Keyboard {

        enum Key {

            Unknown, /* Unknown key */
            Space,
            Apostrophe, /* ' */
            Comma, /* , */
            Minus, /* - */
            Period, /* . */
            Slash, /* / */
            Zero,
            One,
            Two,
            Three,
            Four,
            Five,
            Six,
            Seven,
            Eight,
            Nine,
            Semicolon, /* ; */
            Equal, /* = */
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R,
            S,
            T,
            U,
            V,
            W,
            X,
            Y,
            Z,
            LeftBracket, /* [ */
            BackSlash, /* \ */
            RightBracket, /* ] */
            Tilda, /* ~(or `) */
            Escape,
            Enter,
            Tab,
            BackSpace,
            Insert,
            Delete,
            ArrowUp,
            ArrowLeft,
            ArrowRight,
            ArrowDown,
            PageUp,
            PageDown,
            Home,
            End,
            CapsLock,
            ScrollLock,
            NumLock,
            Pause,
            F1,
            F2,
            F3,
            F4,
            F5,
            F6,
            F7,
            F8,
            F9,
            F10,
            F11,
            F12,
            Num0,
            Num1,
            Num2,
            Num3,
            Num4,
            Num5,
            Num6,
            Num7,
            Num8,
            Num9,
            NumDecimal,
            NumDivide,
            NumMultiply,
            NumSubtract,
            NumAdd,
            NumEnter,
            LeftShift,
            LeftControl,
            LeftAlt,
            RightShift,
            RightControl,
            RightAlt,
            Menu,

            KeyCount /* Total number of keys */

        };

        static const bool& isKeyPressed(const Key &key) _GL_CXX11_NOEXCEPT;

        static void pollEvents() _GL_CXX11_NOEXCEPT;

        static void setKeymap(const unsigned int keys[KeyCount]) _GL_CXX11_NOEXCEPT;

        static void setKeymapKey(const Key &key, const unsigned int &code) _GL_CXX11_NOEXCEPT;

        static void getKeymap(unsigned int keys[KeyCount]) _GL_CXX11_NOEXCEPT;

        static const unsigned int& getKeymapKey(const Key &key) _GL_CXX11_NOEXCEPT;

        static void setDefaultKeymap() _GL_CXX11_NOEXCEPT;

    private:

        /* Do not create the 'Keyboard' objects */
        Keyboard()
#if __cplusplus >= 201103L
        = default;
#else
        { }
#endif

    };

}

#endif /* _GL_KEYBOARD_HPP */
