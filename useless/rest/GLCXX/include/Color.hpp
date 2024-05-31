#ifndef _GLCXX_COLOR
#define _GLCXX_COLOR 1

#include "System/Config.hpp"

extern "C++" {

    namespace gl {

        class Color {

        public:

            Uint8 r, g, b, a;

            explicit Color(const Uint8 &r = Uint8(), const Uint8 &g = Uint8(), const Uint8 &b = Uint8(),
                           const Uint8 &a = 255) noexcept;

            explicit Color(const Uint32 &v) noexcept;

            Color& operator=(const Color &c) noexcept;

            Color& operator=(const Uint32 &v) noexcept;

            Uint32 toUint32() const noexcept;

            static float toFloat(const Uint8 &c);

            static Uint8 toUint8(const float &c);

            static const Color Black;

            static const Color Red;

            static const Color Green;

            static const Color Blue;

            static const Color White;

            static const Color Transparent;

        };

        Color operator+(const Color &c1, const Color &c2) noexcept;

        Color operator-(const Color &c1, const Color &c2) noexcept;

        Color operator*(const Color &c1, const Color &c2) noexcept;

        bool operator==(const Color &c1, const Color &c2) noexcept;

        bool operator!=(const Color &c1, const Color &c2) noexcept;

        Color& operator+=(Color &c1, const Color &c2) noexcept;

        Color& operator-=(Color &c1, const Color &c2) noexcept;

        Color& operator*=(Color &c1, const Color &c2) noexcept;

    }

}

#endif /* _GLCXX_COLOR */
