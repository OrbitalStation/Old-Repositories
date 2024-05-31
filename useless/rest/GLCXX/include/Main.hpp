#ifndef GLCXX_MAIN
#define GLCXX_MAIN

#include "System/Config.hpp"

extern "C++" {

    namespace gl {

        /* Initialize the GLCXX library */
        void init();

        /* Terminate the GLCXX library */
        void terminate();

        double getTime();

    }

}

#endif /* GLCXX_MAIN */
