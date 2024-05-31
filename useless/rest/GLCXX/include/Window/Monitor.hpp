#ifndef GLCXX_MONITOR
#define GLCXX_MONITOR

#include "../System/Config.hpp"
#include "VideoMode.hpp"

extern "C++" {

    namespace gl {

        class Monitor {

        public:

            Monitor();

            static Monitor primary();

            ///
            ///@pointer_lifetime
            /// Pointer is allocated by 'new',
            /// you need to 'delete[]' it
            ///

            static Monitor* all();

            static Monitor* all(int &count);

            static int count();

            ///
            ///@pointer_lifetime
            /// Pointer is allocated by 'new',
            /// you need to 'delete[]' it
            ///

            VideoMode* videoModes(int &count);

            VideoMode videoMode() const;

            void size(int &x, int &y);

            void position(int &x, int &y);

            const char* name();

            std::string info();

        private:

#ifdef GLCXX_LOW
            friend void low::setMonitorCallback(const low::MonitorCallback &);
#endif

#ifdef GLCXX_WINDOW
            friend class Window;
#endif

            explicit Monitor(void * const &data);

            void *data;

        };

    }

}

#endif /* GLCXX_MONITOR */
