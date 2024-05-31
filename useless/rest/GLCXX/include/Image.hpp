#ifndef GLCXX_IMAGE
#define GLCXX_IMAGE

#if __cplusplus < 201103L && !defined GLCXX_CONFIG && !defined GLCXX_WINDOW
extern "C++" {
    namespace gl {
        typedef unsigned char Uint8;
    }
}
#else
# include "System/Config.hpp"
#endif

#include "Color.hpp"

/*
 * By default, if you want to load file without a
 * format(ex. 'my_image', 'my_second_image.') or with unknown format,
 * format will be interpreted as '.png'.
 */
#ifndef GLCXX_IMAGE_DEFAULT_FORMAT
# define GLCXX_IMAGE_DEFAULT_FORMAT png
#endif

extern "C++" {

    namespace gl {

        class Image {

        public:

            enum Format {
                none,
                png,
                jpg,
                /* Alias for 'jpg' */
                jpeg = jpg,
                /* We need 'f'(format) before, because 'default' is a C++ keyword */
                fdefault = GLCXX_IMAGE_DEFAULT_FORMAT
            };

        public:

            Image();

            Image(const char * const &filename);

            ~Image();

            void destroy();

            void loadFromFile(const char * const &filename);

            void create(const unsigned int &x, const unsigned int &y, const Color &color, const Format &format);

            void setPixel(const unsigned int &x, const unsigned int &y, const Color &color);

            Color getPixel(const unsigned int &x, const unsigned int &y) const;

            const unsigned int &width() const;

            const unsigned int &height() const;

            Uint8 * const &data() const;

            const Format& format() const;

            void saveToFile(const char * const &filename) const;

        private:

            static Format getFormat(const std::string &from);

            Format m_format;
            unsigned int m_width, m_height;
            Uint8 *m_data;

        };

    }

}

#endif /* GLCXX_IMAGE */
