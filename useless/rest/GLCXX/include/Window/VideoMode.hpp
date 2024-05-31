#ifndef GLCXX_VIDEOMODE
#define GLCXX_VIDEOMODE

extern "C++" {

    namespace gl {

        class Monitor;

        class VideoMode {

        public:

            VideoMode();

            VideoMode(const int &width, const int &height);

            explicit VideoMode(const Monitor &monitor);

            VideoMode(const int &width, const int &height, const int &r, const int &g, const int &b, const int &refresh);

            int width{}, height{}, redBits{}, greenBits{}, blueBits{}, refreshRate{};

        private:

#ifdef GLCXX_MONITOR
            friend class Monitor;
#endif

            VideoMode& operator = (const void * const &data);

        };

    }

}

#endif /* GLCXX_VIDEOMODE */
