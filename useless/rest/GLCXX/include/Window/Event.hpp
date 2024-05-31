#ifndef GLCXX_EVENT
#define GLCXX_EVENT

extern "C++" {

    namespace gl {

        class Event {

        public:

            Event();

            void clear();

        public:

            enum {
                nothing,
                windowClosed
            } type;

        };

    }

}

#endif /* GLCXX_EVENT */
