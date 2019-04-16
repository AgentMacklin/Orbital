#include <iostream>
#include "body.h"
#include "date.h"
#include "progress.hpp"

#define RAD2DEG(x) (180.0 / M_PI) * x

int main() {
    Body pluto(
        vec(1.218193989126378e1, -3.149522235231989e1, -1.535562041975234e-1),
        vec(3.000627734261702e-3, 4.635059607321797e-4, -9.300258803000724e-4));
    Body neptune(
        vec(2.905640909261118e1, -7.174984730218214e0, -5.218791016710037e-1),
        vec(7.317748743401405e-4, 3.065897473349852e-3, -8.039332012516184e-5));

    double julian = 2458584.5;
    double time = 10000.352;

    double day = 70000.0;
    double neptune_radius = neptune.position_at_time(day).norm();
    double pluto_radius = pluto.position_at_time(day).norm();

    while (neptune_radius < pluto_radius) {
        day += 1.0;
        neptune_radius = neptune.position_at_time(day).norm();
        pluto_radius = pluto.position_at_time(day).norm();
    }

    double first_date = day + julian;

    neptune_radius = neptune.position_at_time(day).norm();
    pluto_radius = pluto.position_at_time(day).norm();

    while (neptune_radius > pluto_radius) {
        day += 1.0;
        neptune_radius = neptune.position_at_time(day).norm();
        pluto_radius = pluto.position_at_time(day).norm();
    }

    double second_date = day + julian;

    std::cout << "Calculating closest approach of Neptune and Pluto..."
              << std::endl;

    double distance = neptune.distance_to(pluto);
    double min_distance = distance;
    int min_day = 0;
    const int limit = 500 * 365;
    ProgressBar progress(limit, 80);
    for (int i = 1; i < limit; ++i) {
        Body new_pluto(pluto.position_at_time((double)i),
                       pluto.velocity_at_time((double)i));
        Body new_neptune(neptune.position_at_time((double)i),
                         neptune.velocity_at_time((double)i));
        distance = new_neptune.distance_to(new_pluto);
        if (distance < min_distance) {
            min_distance = distance;
            min_day = i;
        }
        ++progress;
        progress.display();
    }
    std::cout << "\n" << std::endl;

    Gregorian first_greg = julian_to_greg(first_date);
    /**
     * ============================
     * PRINTING RESULTS TO TERMINAL
     * ============================
     */

    std::cout << std::scientific;  // enable scientific printing

    std::cout << "G:\n  " << neptune.semi_major_axis() << std::endl;
    std::cout << "H:\n  " << neptune.eccentricity() << std::endl;
    std::cout << "I:\n  " << RAD2DEG(neptune.inclination()) << std::endl;
    std::cout << "J:\n  " << RAD2DEG(neptune.argument_of_periapsis())
              << std::endl;
    std::cout << "K:\n  " << RAD2DEG(neptune.argument_of_ascending_node())
              << std::endl;
    std::cout << "L:\n  " << RAD2DEG(neptune.true_anomaly()) << std::endl;
    std::cout << "M:\n  " << pluto.semi_major_axis() << std::endl;
    std::cout << "N:\n  " << pluto.eccentricity() << std::endl;
    std::cout << "O:\n  " << RAD2DEG(pluto.inclination()) << std::endl;
    std::cout << "P:\n  " << RAD2DEG(pluto.argument_of_periapsis())
              << std::endl;
    std::cout << "Q:\n  " << RAD2DEG(pluto.argument_of_ascending_node())
              << std::endl;
    std::cout << "R:\n  " << RAD2DEG(pluto.true_anomaly()) << std::endl;
    std::cout << "\nS-T-U:\n" << neptune.position_at_time(time) << std::endl;
    std::cout << "\nAE-AF-AG:\n" << julian_to_greg(first_date) << std::endl;
}