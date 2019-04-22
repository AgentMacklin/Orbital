/**
 * Austen LeBeau
 * ENGR 3310-002
 *
 * Main entry point for the program
 */

#include <iomanip>
#include <iostream>
#include "date.h"
#include "orbitable.h"
#include "progress.h"
#include "util.h"

#define RAD2DEG(x) x*(180.0 / M_PI)

int main() {
    /*****************************
     * INITIAL SETUP FOR PROGRAM *
     *****************************/

    Orbitable pluto(Vector(1.218193989126378e1, -3.149522235231989e1,
                           -1.535562041975234e-1),
                    Vector(3.000627734261702e-3, 4.635059607321797e-4,
                           -9.300258803000724e-4));
    Orbitable neptune(Vector(2.905640909261118e1, -7.174984730218214e0,
                             -5.218791016710037e-1),
                      Vector(7.317748743401405e-4, 3.065897473349852e-3,
                             -8.039332012516184e-5));

    double julian = 2458584.5;
    double time = 10000.352;

    double day = 70000.0;
    double neptune_radius = neptune.position_at_time(day).norm();
    double pluto_radius = pluto.position_at_time(day).norm();

    /***********************************************
     * CALCULATE WHEN PLUTO PASSES NEPTUNE'S ORBIT *
     ***********************************************/

    while (neptune_radius < pluto_radius) {
        day += 1.0;
        neptune_radius = neptune.position_at_time(day).norm();
        pluto_radius = pluto.position_at_time(day).norm();
    }

    double first_date = day + julian;

    neptune_radius = neptune.position_at_time(day).norm();
    pluto_radius = pluto.position_at_time(day).norm();

    /*****************************************************
     * CALCULATE WHEN PLUTO PASSES NEPTUNE'S ORBIT AGAIN *
     *****************************************************/

    while (neptune_radius > pluto_radius) {
        day += 1.0;
        neptune_radius = neptune.position_at_time(day).norm();
        pluto_radius = pluto.position_at_time(day).norm();
    }

    double second_date = day + julian;

    /*******************************************************
     * CALCULATE THE CLOSEST APPROACH OF NEPTUNE AND PLUTO *
     *******************************************************/

    std::cout << "\nCalculating closest approach of Neptune and Pluto..."
              << std::endl;

    double distance = neptune.distance_to(pluto);
    double min_distance = distance;
    int min_day = 0;
    const int limit = 500 * 365;
    ProgressBar progress(limit, 80);
    for (int i = 1; i < limit; ++i) {
        Orbitable new_pluto(pluto.position_at_time((double)i),
                            pluto.velocity_at_time((double)i));
        Orbitable new_neptune(neptune.position_at_time((double)i),
                              neptune.velocity_at_time((double)i));
        distance = new_neptune.distance_to(new_pluto);

        if (distance < min_distance) {
            min_distance = distance;
            min_day = i;
        }

        // Just progress bar stuff
        ++progress;
        progress.display();
    }

    Gregorian current_day = julian_to_greg(julian);
    Gregorian first_greg = julian_to_greg(first_date);

    /**
     * ============================
     * PRINTING RESULTS TO TERMINAL
     * ============================
     */

    std::cout << std::scientific;  // enable scientific printing
    printer("\nCurrent Date", current_day);
    printer("G", neptune.semi_major_axis());
    printer("H", neptune.eccentricity());
    printer("I", RAD2DEG(neptune.inclination()));
    printer("J", RAD2DEG(neptune.argument_of_periapsis()));
    printer("K", RAD2DEG(neptune.argument_of_ascending_node()));
    printer("L", RAD2DEG(neptune.true_anomaly()));
    printer("M", pluto.semi_major_axis());
    printer("N", pluto.eccentricity());
    printer("O", RAD2DEG(pluto.inclination()));
    printer("P", RAD2DEG(pluto.argument_of_periapsis()));
    printer("Q", RAD2DEG(pluto.argument_of_ascending_node()));
    printer("R", RAD2DEG(pluto.true_anomaly()));
    printer("S-T-U", neptune.position_at_time(time));
    printer("V-W-X", neptune.velocity_at_time(time));
    printer("Y-Z-AA", pluto.position_at_time(time));
    printer("AB-AC-AD", pluto.velocity_at_time(time));
    printer("AE-AF-AG", julian_to_greg(first_date));
    printer("AH-AI-AJ", neptune.position_at_time(first_date - julian));
    printer("AK-AL-AM", pluto.position_at_time(first_date - julian));
    printer("AN-AO-AP", julian_to_greg(second_date));
    printer("AQ-AR-AS", neptune.position_at_time(second_date - julian));
    printer("AT-AU-AV", pluto.position_at_time(second_date - julian));
    printer("Date of Closest Approach",
            julian_to_greg(julian + (double)min_day));
    std::cout << '\n';
}