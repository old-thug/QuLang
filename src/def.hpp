#pragma once

#include <cstdlib>

#define QU_METHOD static

#define UNREACHABLE(fmt, ...)                                                  \
    do {                                                                       \
        std::println("{}:{}: unreachable: " fmt, __FILE__, __LINE__,       \
                         ##__VA_ARGS__);                                       \
        abort();                                                               \
    } while (0)
