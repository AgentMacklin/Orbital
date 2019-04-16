/**
 * Austen LeBeau
 * ENGR 3310-002
 * 
 * Main entry point for the program
 */


#include <iostream>
#include <iomanip>
#include "orbitable.hpp"
#include "date.hpp"
#include "progress.hpp"
#include "rang.hpp"

#define RAD2DEG(x) x * (180.0 / M_PI)
#define CYAN rang::fg::cyan
#define RESET rang::fg::reset

// this feels like heresy, but it makes it more convenient to print out results
#define PRINT(msg, val) std::cout << "\n" << CYAN << msg\
                        << ":\n" << RESET << std::setprecision(12)\
                        << val << std::endl;


int main() {
    
    /*****************************
     * INITIAL SETUP FOR PROGRAM *
     *****************************/

    Orbitable pluto(
        Vec(1.218193989126378e1, -3.149522235231989e1, -1.535562041975234e-1),
        Vec(3.000627734261702e-3, 4.635059607321797e-4, -9.300258803000724e-4));
    Orbitable neptune(
        Vec(2.905640909261118e1, -7.174984730218214e0, -5.218791016710037e-1),
        Vec(7.317748743401405e-4, 3.065897473349852e-3, -8.039332012516184e-5));

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
    PRINT("\nCurrent Date", current_day)
    PRINT("G", neptune.semi_major_axis())
    PRINT("H", neptune.eccentricity())
    PRINT("I", RAD2DEG(neptune.inclination()))
    PRINT("J", RAD2DEG(neptune.argument_of_periapsis()))
    PRINT("K", RAD2DEG(neptune.argument_of_ascending_node()))
    PRINT("L", RAD2DEG(neptune.true_anomaly()))
    PRINT("M", pluto.semi_major_axis())
    PRINT("N", pluto.eccentricity())
    PRINT("O", RAD2DEG(pluto.inclination()))
    PRINT("P", RAD2DEG(pluto.argument_of_periapsis()))
    PRINT("Q", RAD2DEG(pluto.argument_of_ascending_node()))
    PRINT("R", RAD2DEG(pluto.true_anomaly()))
    PRINT("S-T-U", neptune.position_at_time(time))
    PRINT("V-W-X", neptune.velocity_at_time(time))
    PRINT("Y-Z-AA", pluto.position_at_time(time))
    PRINT("AB-AC-AD", pluto.velocity_at_time(time))
    PRINT("AE-AF-AG", julian_to_greg(first_date))
    PRINT("AH-AI-AJ", neptune.position_at_time(first_date - julian))
    PRINT("AK-AL-AM", pluto.position_at_time(first_date - julian))
    PRINT("AN-AO-AP", julian_to_greg(second_date))
    PRINT("AQ-AR-AS", neptune.position_at_time(second_date - julian))
    PRINT("AT-AU-AV", pluto.position_at_time(second_date - julian))
    PRINT("Date of Closest Approach", julian_to_greg(julian + (double)min_day))
}