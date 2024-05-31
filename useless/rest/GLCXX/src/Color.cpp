#include "../include/Color.hpp"

extern "C++" {

namespace gl {

    Color::Color(const Uint8 &r, const Uint8 &g, const Uint8 &b, const Uint8 &a) noexcept : r(r), g(g), b(b), a(a) {}

    Color::Color(const Uint32 &v) noexcept : Color() { this->operator=(v); }

    Color& Color::operator=(const Color &c) noexcept {
        if (&c == this) return *this;
        this->r = c.r;
        this->g = c.g;
        this->b = c.b;
        this->a = c.a;
        return *this;
    }

    Color& Color::operator=(const Uint32 &v) noexcept {
        this->r = (Uint8(v >> 24u));
        this->g = (Uint8((v << 8u) >> 24u));
        this->b = (Uint8((v << 16u) >> 24u));
        this->a = (Uint8((v << 24u) >> 24u));
        return *this;
    }

    Uint32 Color::toUint32() const noexcept {
        return ((Uint32(r) << 24u) | (Uint32(g) << 16u) | (Uint32(b) << 8u) | Uint32(a));
    }

    Color operator+(const Color &c1, const Color &c2) noexcept {
        const Uint16 r = c1.r + c2.r;
        const Uint16 g = c1.g + c2.g;
        const Uint16 b = c1.b + c2.b;
        const Uint16 a = c1.a + c2.a;
        return Color(Uint8(r > 255 ? 255 : r), Uint8(g > 255 ? 255 : g), Uint8(b > 255 ? 255 : b),Uint8(a > 255 ? 255 : a));
    }

    Color operator-(const Color &c1, const Color &c2) noexcept {
        const Int16 r = c1.r - c2.r;
        const Int16 g = c1.g - c2.g;
        const Int16 b = c1.b - c2.b;
        const Int16 a = c1.a - c2.a;
        return Color(Uint8(r < 0 ? 0 : r), Uint8(g < 0 ? 0 : g), Uint8(b < 0 ? 0 : b), Uint8(a < 0 ? 0 : a));
    }

    Color operator*(const Color &c1, const Color &c2) noexcept {
        const Uint16 r = c1.r * c2.r;
        const Uint16 g = c1.g * c2.g;
        const Uint16 b = c1.b * c2.b;
        const Uint16 a = c1.a * c2.a;
        return Color(Uint8(r / 255), Uint8(g / 255), Uint8(b / 255), Uint8(a / 255));
    }

    bool operator==(const Color &c1, const Color &c2) noexcept {
        return (c1.r == c2.r && c1.g == c2.g && c1.b == c2.b && c1.a == c2.a);
    }

    bool operator!=(const Color &c1, const Color &c2) noexcept { return !(c1 == c2); }

    Color& operator+=(Color &c1, const Color &c2) noexcept { return (c1 = c1 + c2); }

    Color& operator-=(Color &c1, const Color &c2) noexcept { return (c1 = c1 - c2); }

    Color& operator*=(Color &c1, const Color &c2) noexcept { return (c1 = c1 * c2); }

    float Color::toFloat(const Uint8 &c) { return float(c) / 255.f; }

    Uint8 Color::toUint8(const float &c) { return c * 255; }

    const Color Color::Black = Color(0, 0, 0);

    const Color Color::Red = Color(255, 0, 0);

    const Color Color::Green = Color(0, 255, 0);

    const Color Color::Blue = Color(0, 0, 255);

    const Color Color::White = Color(255, 255, 255);

    const Color Color::Transparent = Color(0, 0, 0, 0);

}

}
