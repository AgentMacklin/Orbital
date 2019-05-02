/**
 * Austen LeBeau
 * ENGR 3310-002
 */

#include <iomanip>
#include <iostream>
#include <algorithm>
#include "lambert.h"
#include "date.h"
#include "orbitable.h"
#include "util.h"

#define RAD2DEG(x) x*(180.0 / M_PI)

int main() {

    std::cout << std::scientific;

    Orbitable Earth (
            Vector (
                -8.601208063143356e-1,
                -5.149780543253566e-1,
                -7.874942568326342e-6
                ),
            Vector (
                8.657167839482775e-3,
                -1.475977839094277e-2,
                8.628767650706375e-7
                )
            );

    Orbitable Mars (
            Vector (
                -2.622004974599996e-1,
                1.577561813000981,
                3.925526885339471e-2
                ),
            Vector (
                -1.328446880856093e-2,
                -1.101073656563134e-3,
                3.028803664779320e-4
                )
            );

    // Asteroid
    Orbitable ToshiH (
            Vector (
                -7.436280142422779e-1,
                -2.774619548644846,
                1.476905508096402e-1
                ),
            Vector (
                8.942035174142673e-3,
                -3.065578719141641e-3,
                9.533854781906894e-4
                )
            );

    double current_date = gregorian_to_julian(Gregorian (2019, 4, 22, 0, 0 ,0));
    double earth_departure_date = gregorian_to_julian(Gregorian (2033, 13, 8, 0, 0, 0));
    double mars_flyby_date = gregorian_to_julian(Gregorian (2037, 9, 21, 0, 0, 0));
    double asteroid_arrival_date = gregorian_to_julian(Gregorian (2039, 7, 13, 0, 0, 0));
    
    Vector r_0 = Earth.position_at_time(earth_departure_date - current_date);
    Vector r = Mars.position_at_time(mars_flyby_date - current_date);

    LambertResults earth_to_mars = lambert(r_0, r, 1, mars_flyby_date - earth_departure_date);

    printer("A-B-C", r_0);
    printer("D-E-F", r);
    printer("G-H-I", earth_to_mars.v_0);
    printer("J-K-L", earth_to_mars.v);



    std::cout << std::endl;

    return 0;
}
