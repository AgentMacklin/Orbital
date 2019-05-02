/**
 * Austen LeBeau
 * ENGR 3310-002
 * 
 * Compiled on Linux to main
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
    
    // Vectors for problem 1
    Vector r (12214.83899, 10249.46731, 0.0);
    Vector r_0 (15945.34, 0.0, 0.0);
    LambertPractice test = practice_lambert(r_0, r, -1, 76.0 * 60.0);
    
    /* PROBLEM 2 */

    // Creating Earth and Mars objects
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


    Gregorian initial_date (2005, 3, 4, 0, 0, 0);
    Gregorian future_date (2005, 12, 11, 0, 0, 0);
    double initial_julian = gregorian_to_julian(initial_date);
    double future_julian = gregorian_to_julian(future_date);
    double time_span = future_julian - initial_julian;

    int day = 3;
    std::vector<double> rel_velocs;
    std::vector<LambertResults> lamb_results;
    while (day <= 500) {
        Vector position = Mars.position_at_time(day);
        LambertResults temp = lambert(Earth.current_position(), position, 0.0, day);
        double relative_veloc = (temp.v_0 - Earth.current_velocity()).norm();
        rel_velocs.push_back(relative_veloc);
        lamb_results.push_back(temp);
        day += 1;
    }

    int min_day = std::min_element(rel_velocs.begin(), rel_velocs.end()) - rel_velocs.begin();
    double time_of_arrival = min_day + initial_julian;
    Gregorian arrival_gregorian = julian_to_greg(time_of_arrival);

    // Printing results
    std::cout << std::scientific;
    printer("E", test.f);
    printer("F", test.g);
    printer("G", test.f_dot);
    printer("H", test.g_dot);
    printer("I-J-K", test.v_0);
    printer("L-M-N", test.v);
    printer("O-P-Q", Earth.current_position());
    printer("R-S-T", Earth.current_velocity());
    printer("U-V-W", Mars.current_position());
    printer("X-Y-Z", Mars.current_velocity());
    printer("AA", future_julian);
    printer("AB-AC-AD", Mars.position_at_time(time_span));
    printer("AE-AF-AG", Mars.velocity_at_time(time_span));
    printer("AH-AI-AJ", lambert(Earth.current_position(), Mars.position_at_time(time_span), 1, time_span).v_0);
    printer("AK-AL-AM", lambert(Earth.current_position(), Mars.position_at_time(time_span), 1, time_span).v);
    printer("AN-AO-AP", lambert(Earth.current_position(), Mars.position_at_time(time_span), -1, time_span).v_0);
    printer("AQ-AR-AS", lambert(Earth.current_position(), Mars.position_at_time(time_span), -1, time_span).v);
    printer("AT-AU-AV", arrival_gregorian);
    printer("AW-AX-AY", lamb_results[min_day].v_0);
    printer("AZ-BA-BB", lamb_results[min_day].v);


    return 0;
}
