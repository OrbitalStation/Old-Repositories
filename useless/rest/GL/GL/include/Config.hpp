#ifndef _GL_CONFIG_HPP
#define _GL_CONFIG_HPP

#ifndef __cplusplus
# error "This file is a part of 'GL' library, which is only for C++"
#endif

#ifndef _GL_INLINE
# ifdef __GNUC__
#  define _GL_INLINE __attribute__((__always_inline__)) inline
# else
#  define _GL_INLINE inline
# endif
#endif

#ifndef _GL_CXX11_NOEXCEPT
# if __cplusplus >= 201103L
#  define _GL_CXX11_NOEXCEPT noexcept
# else
#  define _GL_CXX11_NOEXCEPT throw()
# endif
#endif

#ifndef _GL_CXX11_NULLPTR
# if __cplusplus >= 201103L
#  define _GL_CXX11_NULLPTR nullptr
# else
#  ifndef NULL
#   define __need_NULL
#   include <stddef.h>
#  endif
#  define _GL_CXX11_NULLPTR NULL
# endif
#endif

#endif /* _GL_CONFIG_HPP */
