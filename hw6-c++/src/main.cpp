#include <iostream>
#include "body.h"

int main() {
    Body earth(
        vec(
            -3.914741740463327E-01,
            -1.436094702371459E+00,
            -2.047823201895446E-02),
        vec(
            1.401859610775295E-02,
            -2.508557862682466E-03,
            -3.971649629870528E-04
            )
    );

    std::cout << earth.eccentricity() << std::endl;
}