#ifndef LAMBERT_H
#define LAMBERT_H

#include <Eigen/Dense>
#include <cmath>
#include "util.h"

#define PRACTICEGM 398600.4418
#define SOLARGM 2.963092749241593e-4

using Vector = Eigen::Vector3d;

struct LambertResults {
    Vector v_0, v;
};

struct LambertPractice {
    Vector v_0, v;
    double f, f_dot, g, g_dot;
};

double c2_coeff(double phi) { 
    return (1.0 - cos(sqrt(phi))) / phi; 
}

double c3_coeff(double phi) {
    return (sqrt(phi) - sin(sqrt(phi))) / sqrt(pow(phi, 3));
}

LambertResults lambert(Vector r_0, Vector r, double tm, double delta_t) {
    double r_mag = r.norm();
    double r_0_mag = r_0.norm();
    double cos_delta_tht = r.dot(r_0) / (r_mag * r_0_mag);
    if (tm == 0) {
        double delta_tht = acos(cos_delta_tht);
        tm = -sin(delta_tht) / sqrt(1.0 - sqr(cos_delta_tht));
    }
    double A = tm * sqrt(r_mag * r_0_mag * (1.0 + cos_delta_tht));
    
    if (A == 0.0) 
        throw std::runtime_error("Lambert Algorithm error: PANIC, YOU'RE DOOMED");

    double phi_n = 0;

    double c2 = 0.5;
    double c3 = 1.0 / 6.0;

    double phi_upper = 4.0 * sqr(M_PI);
    double phi_lower = -4.0 * M_PI;

    double delta_t_n = 0;
    double y_new = 0;


    while (fabs(delta_t - delta_t_n) > 1.0e-6) {
        y_new = r_0_mag + r_mag + A * (((phi_n * c3) - 1.0) / sqrt(c2));
        if ((A > 0) && (y_new < 0)) {
            while (y_new < 0.0) {
                phi_lower += M_PI_4;
                y_new = r_0_mag + r_mag + A * (((phi_lower * c3) - 1.0) / sqrt(c2));
            }
        }
        double x_n = sqrt(y_new / c2);
        delta_t_n = ((pow(x_n, 3) * c3) + (A * sqrt(y_new))) / sqrt(PRACTICEGM);
        delta_t_n <= delta_t ? phi_lower = phi_n : phi_upper = phi_n;
        phi_n = (phi_upper + phi_lower) / 2.0;
        if (phi_n >= 0.0) {
            c2 = c2_coeff(phi_n);
            c3 = c3_coeff(phi_n);
        } else {
            break;
        }
    }

    double f = 1.0 - (y_new / r_0_mag);
    double g_dot = 1.0 - (y_new / r_mag);
    double g = A * sqrt(y_new / PRACTICEGM);
    double f_dot = ((f * g_dot) - 1.0) / g;
    Vector v_0 = (r - (f * r_0)) / g;
    Vector v = ((g_dot * r) - r_0) / g;

    return LambertResults {
        .v_0 = v_0,
        .v = v,
    };

}

LambertPractice practice_lambert(Vector r_0, Vector r, double tm, double delta_t) {
    double r_mag = r.norm();
    double r_0_mag = r_0.norm();
    double cos_delta_tht = r.dot(r_0) / (r_mag * r_0_mag);
    if (tm == 0) {
        double delta_tht = acos(cos_delta_tht);
        tm = -sin(delta_tht) / sqrt(1.0 - sqr(cos_delta_tht));
    }
    double A = tm * sqrt(r_mag * r_0_mag * (1.0 + cos_delta_tht));
    
    if (A == 0.0) 
        throw std::runtime_error("Lambert Algorithm error: PANIC, YOU'RE DOOMED");

    double phi_n = 0;

    double c2 = 0.5;
    double c3 = 1.0 / 6.0;

    double phi_upper = 4.0 * sqr(M_PI);
    double phi_lower = -4.0 * M_PI;

    double delta_t_n = 0;
    double y_new = 0;

    bool first_loop = true;

    while (fabs(delta_t - delta_t_n) > 1.0e-6) {
        y_new = r_0_mag + r_mag + A * (((phi_n * c3) - 1.0) / sqrt(c2));
        if ((A > 0) && (y_new < 0)) {
            while (y_new < 0.0) {
                phi_lower += M_PI_4;
                y_new = r_0_mag + r_mag + A * (((phi_lower * c3) - 1.0) / sqrt(c2));
            }
        }
        double x_n = sqrt(y_new / c2);
        delta_t_n = ((pow(x_n, 3) * c3) + (A * sqrt(y_new))) / sqrt(PRACTICEGM);
        delta_t_n <= delta_t ? phi_lower = phi_n : phi_upper = phi_n;
        phi_n = (phi_upper + phi_lower) / 2.0;
        if (phi_n >= 0.0) {
            c2 = c2_coeff(phi_n);
            c3 = c3_coeff(phi_n);
        } else {
            break;
        }
        if (first_loop) {
            std::cout << std::scientific;
            printer("A", y_new);
            printer("B", 0.5);
            printer("C", 1.0 / 6.0);
            printer("D", delta_t_n);
            first_loop = false;
        }
    }

    double f = 1.0 - (y_new / r_0_mag);
    double g_dot = 1.0 - (y_new / r_mag);
    double g = A * sqrt(y_new / PRACTICEGM);
    double f_dot = ((f * g_dot) - 1.0) / g;
    Vector v_0 = (r - (f * r_0)) / g;
    Vector v = ((g_dot * r) - r_0) / g;

    return LambertPractice {
        .v_0 = v_0,
        .v = v,
        .f = f,
        .f_dot = f_dot,
        .g = g,
        .g_dot = g_dot
    };

}

#endif  // LAMBERT_H
