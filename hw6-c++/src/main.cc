/**
 * Austen LeBeau
 * ENGR 3310-002
 */

#include <iomanip>
#include <iostream>
#include "lambert.h"
#include "date.h"
#include "orbitable.h"
#include <progress.h>
#include "util.h"

#define RAD2DEG(x) x*(180.0 / M_PI)

int main() {
    // Vectors for problem 1
    Vector r (12214.83899, 10249.46731, 0.0);
    Vector r_0 (15945.34, 0.0, 0.0);
    LambertPractice test = practice_lambert(r_0, r, -1, 76.0 * 60.0);
    
    // Problem 2
    Orbitable Earth (
        Vector (
            -9.461495867803592e-1,
            2.827386520764219e-1,
            -1.082702427768024e-4
        ),
        Vector (
            -5.189853346265090e-3,
            -1.654674749332120e-2,
            5.062030407932107e-7
        )
    );

    Orbitable Mars (
        Vector (
            -3.914741740463327e-1,
            -1.436094702371459e0,
            -2.047823201895446e-2
        ),
        Vector (
            1.401859610775295e-2,
            -2.508557862682466e-3,
            -3.971649629870528e-4
        )
    );

    // Printing results
    std::cout << std::scientific;
    printer("E", test.f);
    printer("F", test.g);
    printer("G", test.f_dot);
    printer("H", test.g_dot);
    printer("I-J-K", test.v_0);
    printer("L-M-N", test.v);
    return 0;
}
