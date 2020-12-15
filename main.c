#include "foo.h"
#include "rust/rust_cbindgen.h"

int main()
{
    struct foo foo = { .bar = 42 };
    use_foo(&foo);
}