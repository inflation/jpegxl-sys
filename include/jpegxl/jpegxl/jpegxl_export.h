
#ifndef JPEGXL_EXPORT_H
#define JPEGXL_EXPORT_H

#ifdef JPEGXL_STATIC_DEFINE
#  define JPEGXL_EXPORT
#  define JPEGXL_NO_EXPORT
#else
#  ifndef JPEGXL_EXPORT
#    ifdef JPEGXL_INTERNAL_LIBRARY_BUILD
        /* We are building this library */
#      define JPEGXL_EXPORT __attribute__((visibility("default")))
#    else
        /* We are using this library */
#      define JPEGXL_EXPORT __attribute__((visibility("default")))
#    endif
#  endif

#  ifndef JPEGXL_NO_EXPORT
#    define JPEGXL_NO_EXPORT __attribute__((visibility("hidden")))
#  endif
#endif

#ifndef JPEGXL_DEPRECATED
#  define JPEGXL_DEPRECATED __attribute__ ((__deprecated__))
#endif

#ifndef JPEGXL_DEPRECATED_EXPORT
#  define JPEGXL_DEPRECATED_EXPORT JPEGXL_EXPORT JPEGXL_DEPRECATED
#endif

#ifndef JPEGXL_DEPRECATED_NO_EXPORT
#  define JPEGXL_DEPRECATED_NO_EXPORT JPEGXL_NO_EXPORT JPEGXL_DEPRECATED
#endif

#if 0 /* DEFINE_NO_DEPRECATED */
#  ifndef JPEGXL_NO_DEPRECATED
#    define JPEGXL_NO_DEPRECATED
#  endif
#endif

#endif /* JPEGXL_EXPORT_H */
