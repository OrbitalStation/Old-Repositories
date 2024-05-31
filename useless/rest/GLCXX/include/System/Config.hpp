#ifndef GLCXX_CONFIG
#define GLCXX_CONFIG

#if !defined __cplusplus
# error "This file is not for C, but for C++!"
#endif

#if __cplusplus <= 201103L
# error "You need C++11 to use this file!"
#endif

#include <bits/exception.h>
#include <string>

extern "C++" {

    namespace gl {

        typedef unsigned char Uint8;
        typedef unsigned short int Uint16;
        typedef signed short int Int16;
        typedef unsigned int Uint32;

        class error : std::exception {

        public:

            error() = default;

            ~error() = default;

            error(const char * const &source) : m_what(source) { }

            error(const error &error) : m_what(error.m_what) { }

            error(error &&error) : m_what(std::move(error.m_what)) { }

            const char *what() const noexcept override { return m_what.c_str(); }

        private:

            std::string m_what;

        };

    }

}

#endif /* GLCXX_CONFIG */
