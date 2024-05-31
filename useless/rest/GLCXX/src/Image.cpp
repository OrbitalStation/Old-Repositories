#include "../include/Image.hpp"
#include "stb_image/read.h"
#include "stb_image/write.h"

extern "C++" {

    namespace gl {

        Image::Image() : m_width(), m_height(), m_format(none), m_data(nullptr) {

        }

        Image::Image(const char * const &filename) : Image() {
            this->loadFromFile(filename);
        }

        Image::~Image() { this->destroy(); }

        void Image::destroy() {
            if (this->m_data != nullptr) {
                stbi_image_free((void *)this->m_data);
                this->m_data = nullptr;
                this->m_width = this->m_height = 0;
                this->m_format = none;
            }
        }

        Image::Format Image::getFormat(const std::string &from) {
            unsigned long int pos = (from.find('.'));
            if (pos == std::string::npos || pos == from.length() - 1) return fdefault;
            std::string f;
            for (; pos < from.length(); ++pos) f.push_back(from[pos]);
            if (f == "png") return png;
            else if (f == "jpg" || f == "jpeg") return jpg;
            return fdefault;
        }

        void Image::loadFromFile(const char * const &filename) {
            switch (this->m_format = this->getFormat(filename)) {
                case png:
                    this->m_data = stbi_load(filename, (int *)&this->m_width, (int *)&this->m_height, nullptr, 4);
                    break;
                case jpg:
                    this->m_data = stbi_load(filename, (int *)&this->m_width, (int *)&this->m_height, nullptr, 3);
                    break;
                case none:
                    break;
            }
        }

        void Image::create(const unsigned int &x, const unsigned int &y, const Color &color, const Format &format) {
            if (this->m_data != nullptr) stbi_image_free((void *)this->m_data);
            this->m_width = x;
            this->m_height = y;
            this->m_format = format;
            if (format == none) {
                this->m_width = this->m_height = 0u;
            } else if (format == png) {
                unsigned long int max = x * y * 4;
                if ((this->m_data = (Uint8 *) malloc (max)) == nullptr) {
                    this->m_width = this->m_height = 0u;
                    this->m_format = none;
                    return;
                }
                --max;
                for (unsigned long int i = 0ul; i < max;) {
                    this->m_data[i++] = color.r;
                    this->m_data[i++] = color.g;
                    this->m_data[i++] = color.b;
                    this->m_data[i++] = color.a;
                }
            } else if (format == jpeg) {
                unsigned long int max = x * y * 3;
                if ((this->m_data = (Uint8 *) malloc (max)) == nullptr) {
                    this->m_width = this->m_height = 0u;
                    this->m_format = none;
                    return;
                }
                --max;
                for (unsigned long int i = 0ul; i < max;) {
                    this->m_data[i++] = color.r;
                    this->m_data[i++] = color.g;
                    this->m_data[i++] = color.b;
                }
            }
        }

        void Image::setPixel(const unsigned int &x, const unsigned int &y, const Color &color) {
            if (this->m_data == nullptr || this->m_width < x || this->m_height < y
                || this->m_format == none) return;
            if (this->m_format == png) {
                auto *pointer = this->m_data + (y * this->m_width + x) * 4;
                *pointer++ = color.r;
                *pointer++ = color.g;
                *pointer++ = color.b;
                *pointer = color.a;
            } else if (this->m_format == jpg) {
                auto *pointer = this->m_data + (y * this->m_width + x) * 3;
                *pointer++ = color.r;
                *pointer++ = color.g;
                *pointer = color.b;
            }
        }

        Color Image::getPixel(const unsigned int &x, const unsigned int &y) const {
            if (this->m_data == nullptr || this->m_width < x || this->m_height < y
                || this->m_format == none) return Color::Black;
            if (this->m_format == png) {
                auto *pointer = this->m_data + (y * this->m_width + x) * 4;
                return Color(*pointer, *(pointer + 1), *(pointer + 2), *(pointer + 3));
            } else if (this->m_format == jpg) {
                auto *pointer = this->m_data + (y * this->m_width + x) * 3;
                return Color(*pointer, *(pointer + 1), *(pointer + 2), '\255');
            }
            return Color::Black;
        }

        const unsigned int& Image::width() const { return this->m_width; }

        const unsigned int& Image::height() const { return this->m_height; }

        Uint8 * const& Image::data() const { return this->m_data; }

        const Image::Format& Image::format() const { return this->m_format; }

        void Image::saveToFile(const char * const &filename) const {
            switch (this->m_format) {
                case png:
                    stbi_write_png(filename, (int)this->m_width, (int)this->m_height,
                            4, (void *)this->m_data, int(this->m_width * 4));
                    break;
                case jpg:
                    stbi_write_jpg(filename, (int)this->m_width, (int)this->m_height,
                            4, (void *)this->m_data, 0);
                case none:
                    break;
            }
        }

    }

}
