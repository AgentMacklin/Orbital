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


#define AU 1.49597870700e11
#define RAD2DEG(x) x*(180.0 / M_PI)

int main() {

    std::cout << std::scientific;

    Orbitable Earth (
            Vector (
                -8.601208063143356e-1,
                -5.149780543253561e-1,
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

    Gregorian current_gregorian (2019, 4, 22, 0, 0, 0);
    Gregorian earth_departure (2033, 8, 13, 0, 0, 0);
    Gregorian mars_flyby(2037, 9, 21, 0, 0, 0);
    Gregorian asteroid_arrival (2039, 7, 13, 0 ,0, 0);

    double current_date = gregorian_to_julian(current_gregorian);
    double earth_departure_date =  gregorian_to_julian(earth_departure);
    double mars_flyby_date = gregorian_to_julian(mars_flyby);
    double asteroid_arrival_date = gregorian_to_julian(asteroid_arrival);
    
    Vector r_0 = Earth.position_at_time(earth_departure_date - current_date);
    Vector r = Mars.position_at_time(mars_flyby_date - current_date);

    LambertResults earth_to_mars = lambert(r_0, r, -1, mars_flyby_date - earth_departure_date);

    r_0 = r;
    r = ToshiH.position_at_time(asteroid_arrival_date - current_date);

    LambertResults mars_to_toshi = lambert(r_0, r, -1, asteroid_arrival_date - mars_flyby_date);


    /* PROBLEM 1D */
    double MARSGM = 9.54953192489925e-11;
    Vector mars_v = Mars.velocity_at_time(mars_flyby_date - current_date);
    Vector delta_v_1 = earth_to_mars.v - mars_v;
    Vector delta_v_2 = mars_to_toshi.v_0 - mars_v;
    Vector e_1 = delta_v_1.normalized();
    Vector e_2 = delta_v_2.normalized();
    double semi_periapsis = MARSGM / delta_v_1.dot(delta_v_1);
    Vector ecc_epsilon = (e_1 - e_2) / (e_1 - e_2).norm();
    double new_eccen = -1 / ecc_epsilon.dot(e_2);
    double peri_location = semi_periapsis * (new_eccen - 1) * (AU / 1000);

    /* PROBLEM 1E */
    double EARTHGM = 8.887692546888129e-10;
    double earth_p = 6371.01;
    double veloc_sc = (earth_to_mars.v_0 - Earth.velocity_at_time(earth_departure_date - current_date)).norm();
    double E = -EARTHGM / sqr(veloc_sc);
    double v_esc_coeff = EARTHGM * ((2 / (1000 * earth_p / AU)) - (1 / E));
    double v_esc = sqrt(v_esc_coeff) * AU / 1000 / 86400;


    /* PROBLEM 1F */
    Vector toshi_v = ToshiH.velocity_at_time(asteroid_arrival_date - current_date);
    double delta_v_m = abs((mars_to_toshi.v_0 - mars_v).norm() - (earth_to_mars.v - mars_v).norm()) * AU / (1000 * 86400);
    double delta_v_t = (mars_to_toshi.v - toshi_v).norm() * AU / (1000 * 86400);
    double delta_v = v_esc + delta_v_m + delta_v_t;

    /* PROBLEM 2 */
    Gregorian init_date (2030, 1, 1, 0, 0, 0);
    Gregorian end_date (2039, 12, 31, 0, 0, 0);
    double init_julian = gregorian_to_julian(init_date) - current_date;
    double end_julian = gregorian_to_julian(end_date) - current_date;


    for (double i = init_julian; i < end_julian; i += 60) {
        Vector new_E_pos = Earth.position_at_time(i);
        Vector new_E_veloc = Earth.velocity_at_time(i);
        for (double j = i + 60; i < end_julian; j += 60) {
            Vector new_M_pos = Mars.position_at_time(j);
            Vector new_M_veloc = Mars.velocity_at_time(j);
            for (double k = j + 60; k < end_julian; k += 60) {
                Vector new_T_pos = ToshiH.position_at_time(k);
                Vector new_T_veloc = ToshiH.velocity_at_time(k);
                double delta_t_1 = j - i;
                double delta_t_2 = k - j;
                LambertResults v1v2 = lambert(new_E_pos, new_M_pos, -1, delta_t_1);
                LambertResults v3v4 = lambert(new_M_pos, new_T_pos, -1, delta_t_2);


            }
        }
    }

    printer("A-B-C", r_0);
    printer("D-E-F", r);
    printer("G-H-I", earth_to_mars.v_0);
    printer("J-K-L", earth_to_mars.v);
    printer("M-N-O", mars_to_toshi.v_0);
    printer("P-Q-R", mars_to_toshi.v);
    printer("S", peri_location);
    printer("T", v_esc);
    printer("U", delta_v);
    std::cout << std::endl;

    return 0;
}
