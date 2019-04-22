#ifndef LAMBERT_H
#define LAMBERT_H

#include <Eigen/Dense>
#include <cmath>
#include "util.h"

#define PRACTICEGM 398600.4418

using Vector = Eigen::Vector3d;

struct LambertResults {
    double v_0, v;
};

double c2_coeff(double phi) { return (1 - cos(phi)) / phi; }

double c3_coeff(double phi) {
    return (sqrt(phi) - sin(sqrt(phi))) / sqrt(pow(phi, 3));
}

LambertResults practice_lambert(Vector r_0, Vector r, double delta_t) {
    double r_mag = r.norm();
    double r_0_mag = r_0.norm();
    double cos_delta_tht = r_0.dot(r) / (r_mag * r_0_mag);
    double sin_delta_tht = sqrt(1 - sqr(cos_delta_tht));
    double A = sqrt(r_mag * r_0_mag * (1 + cos_delta_tht));
    if (A == 0) throw "Lambert algorithm cannot provide a solution.";

    double phi_n = 0;

    double c2 = c2_coeff(phi_n);
    double c3 = c3_coeff(phi_n);

    double phi_upper = 4 * sqr(M_PI);
    double phi_lower = -4 * M_PI;

    double delta_t_n = 0;

    while (fabs(delta_t - delta_t_n) > 1e-6) {
        double y_new = r_0_mag + r_mag + (A * phi_lower * c3 - 1) / sqrt(c2);
        if ((A > 0) && (y_new < 0)) {
            while (y_new < 0) {
                phi_lower += M_PI_4;
                y_new = r_0_mag + r_mag + (A * phi_lower * c3 - 1) / sqrt(c2);
            }
        }
        double x_n = sqrt(y_new / c2);
        delta_t_n = (pow(x_n, 3) * c3 + A * sqrt(y_new));
    }
}

#endif  // LAMBERT_H