#ifndef _RUST_FOO_H
#define _RUST_FOO_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void use_foo(foo *foo);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* _RUST_FOO_H */
