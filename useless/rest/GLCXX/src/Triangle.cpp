#include "../include/Figures/Triangle.hpp"
#include "glad/glad.h"

extern "C++" {

    namespace gl {

        Triangle::Triangle() : vao(), vbo(), need_flush(true), vertices{} {
            glGenVertexArrays(1, &this->vao);
            glGenBuffers(1, &this->vbo);
            float *pointer = this->vertices;

            *pointer++ = -0.5f;
            *pointer++ = -0.5f;
            *pointer++ = +0.0f;
            *pointer++ = +1.0f;
            *pointer++ = +1.0f;
            *pointer++ = +1.0f;
            *pointer++ = +1.0f;

            *pointer++ = +0.5f;
            *pointer++ = -0.5f;
            *pointer++ = +0.0f;
            *pointer++ = +1.0f;
            *pointer++ = +1.0f;
            *pointer++ = +1.0f;
            *pointer++ = +1.0f;

            *pointer++ = +0.0f;
            *pointer++ = +0.5f;
            *pointer++ = +0.0f;
            *pointer++ = +1.0f;
            *pointer++ = +1.0f;
            *pointer++ = +1.0f;
            *pointer   = +1.0f;
        }

        void Triangle::destroy() {
            if (this->vao == 0u) return;
            glDeleteBuffers(1, &this->vbo);
            glDeleteVertexArrays(1, &this->vao);
            this->vao = 0u;
        }

        void Triangle::flush() const {
            if (!this->need_flush) return;
            this->need_flush = false;
            glBindVertexArray(this->vao);
            glBindBuffer(GL_ARRAY_BUFFER, this->vbo);
            glBufferData(GL_ARRAY_BUFFER, sizeof(this->vertices), this->vertices, GL_DYNAMIC_DRAW);
            glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 7 * sizeof(float), nullptr);
            glEnableVertexAttribArray(0);
            glVertexAttribPointer(1, 4, GL_FLOAT, GL_FALSE, 7 * sizeof(float), (void *)(sizeof(float) * 3));
            glEnableVertexAttribArray(1);
        }

        void Triangle::draw() const noexcept {
            this->flush();
            glDrawArrays(GL_TRIANGLES, 0, 3);
        }

        void Triangle::setColor(const Uint8 &vertex, const Color &color) {
            if (vertex > 2) return;
            float *pointer = this->vertices + vertex * 7 + 3;
            *pointer++ = Color::toFloat(color.r);
            *pointer++ = Color::toFloat(color.g);
            *pointer++ = Color::toFloat(color.b);
            *pointer   = Color::toFloat(color.a);
            this->need_flush = true;
        }

        void Triangle::setPosition(const Uint8 &vertex, const float &x, const float &y, const float &z) {
            if (vertex > 2) return;
            float *pointer = this->vertices + vertex * 7;
            *pointer++ = x;
            *pointer++ = y;
            *pointer   = z;
        }

    }

}
