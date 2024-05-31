#ifndef GLCXX_KEYBOARD
#define GLCXX_KEYBOARD

extern "C++" {

    namespace gl {

        namespace Keyboard {

            enum Key {

                Unknown = -1,
                Space,
                Apostrophe, // '
                Comma, // ,
                Minus, // -
                Period, // .
                Slash, // /
                Zero, One, Two, Three, Four,
                Five, Six, Seven, Eight, Nine,
                Semicolon, // ;
                Equal, // =
                A, B, C, D, E, F, G, H, I,
                J, K, L, M, N, O, P, Q, R,
                S, T, U, V, W, X, Y, Z,
                LeftBracket, // [
                RightBracket, // ]
                BackSlash, /* \ */
                GraveAccent, // `
                Escape,
                Enter,
                Tab,
                World1, // non-US 1
                World2, // non-US 2
                Backspace,
                Insert,
                Delete,
                Up, Left, Right, Down, // Arrows
                PageUp,
                PageDown,
                Home,
                End,
                CapsLock,
                ScrollLock,
                NumLock,
                PrintScreen,
                Pause,
                F1, F2, F3, F4, F5, F6,
                F7, F8, F9, F10, F11, F12,
                F13, F14, F15, F16, F17,
                F18, F19, F20, F21, F22,
                F23, F24, F25,           // F1-F25
                Num0, Num1, Num2, Num3, Num4,
                Num5, Num6, Num7, Num8, Num9,
                NumDecimal, NumDivide,        // Numpad keys
                NumMultiply, NumSubtract,
                NumAdd, NumEnter, NumEqual,
                LShift, RShift,
                LControl, RControl,
                LAlt, RAlt,
                LSuper, RSuper,
                Menu,

                KeyTotal    // Total num of keys

            };

            enum class KeyType {
                Pressed,
                Released,
                Repeated
            };

            bool isKeyPressed(const Key &key);

        }

    }

}

#endif /* GLCXX_KEYBOARD */
