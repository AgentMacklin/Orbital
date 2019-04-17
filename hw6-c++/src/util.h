#ifndef UTIL_H
#define UTIL_H

#include "rang.h"
#include <iostream>

#define CYAN rang::fg::cyan
#define GREEN rang::fg::green
#define BLUE rang::fg::blue
#define RESET rang::fg::reset


std::string underline(std::string str) {
    return std::string(str + "\n") + std::string(str.length(), '-');
}

template <typename T> void printer(std::string msg, T val) {
    if (std::is_same<T, Vector>::value) {
        std::cout << "\n" << CYAN << msg << ":\n" << RESET << std::setprecision(12)\
        << val << std::endl;
    } else if (std::is_same<T, double>::value) {
        std::cout << "\n" << GREEN << msg << ":\n  " << RESET << std::setprecision(12)\
        << val << std::endl;
    } else if (std::is_same<T, Gregorian>::value) {
        std::cout << "\n" << BLUE << underline(msg) << "\n" << RESET << std::setprecision(12)\
        << val << std::endl;
    }
}


#endif // UTIL_H