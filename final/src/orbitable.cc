/**
 * Austen LeBeau
 * ENGR 3310-002
 *
 * Implementation of the Orbitable class.
 */

#include "orbitable.h"
#include <cmath>
#include <iostream>

#define SOLARGM 2.963092749241593e-4
#define PI2 (M_PI * 2.0)

#define SQR(x) x* x

Vector Orbitable::radial_velocity() {
    return (m_velocity.dot(m_position) / m_position.squaredNorm()) * m_position;
}

Vector Orbitable::tangential_velocity() { return omega().cross(m_position); }

double Orbitable::true_anomaly() {
    Vector e_vec = eccentricity_vector();
    Vector p = m_position.normalized();
    double t_anom = e_vec.dot(p) / (e_vec.norm() * p.norm());
    if (p.dot(m_velocity.normalized()) < 0.0) {
        return PI2 - acos(t_anom);
    } else {
        return acos(t_anom);
    }
}

Vector Orbitable::position_at_time(double time) {
    double t_anom = true_anomaly_at_time(time);
    double omega = argument_of_periapsis() - t_anom;
    double inc = inclination();
    double tht = argument_of_ascending_node();
    Matrix t_mat = three_one_three_transform(omega, inc, tht).inverse();
    Vector p = position_at_angle(t_anom);
    Vector out_vector = t_mat * p;
    return out_vector;
}

Vector Orbitable::velocity_at_time(double time) {
    double t_anom = true_anomaly_at_time(time);
    double omega = argument_of_periapsis() - t_anom;
    double inc = inclination();
    double tht = argument_of_ascending_node();
    Matrix t_mat = three_one_three_transform(omega, inc, tht).inverse();
    Vector v = velocity_at_angle(t_anom);
    return t_mat * v;
}

Vector Orbitable::position_at_angle(double angle) {
    double e = eccentricity();
    double p = orbital_parameter();
    double radius = p / (1.0 + e * cos(angle));
    return Vector(radius, 0.0, 0.0);
}

Vector Orbitable::velocity_at_angle(double angle) {
    Matrix t_frame = make_frame();
    Vector h = t_frame * angular_momentum();
    double coeff = h.norm() / orbital_parameter();
    double e = eccentricity();
    return Vector(coeff * -e * sin(angle), coeff * (1.0 + e * cos(angle)), 0.0);
}

Vector Orbitable::eccentricity_vector() {
    Vector h = angular_momentum();
    return (m_velocity.cross(h) / SOLARGM) - m_position.normalized();
}

Vector Orbitable::angular_momentum() { return m_position.cross(m_velocity); }

double Orbitable::total_energy() {
    return 0.5 * (m_velocity.norm() * m_velocity.norm()) -
           (SOLARGM / m_position.norm());
}

Vector Orbitable::omega() {
    return angular_momentum() / m_position.squaredNorm();
}

double Orbitable::frame_rotation_rate() { return omega().norm(); }

double Orbitable::angle_to(Orbitable other) {
    return acos(m_position.dot(other.m_position) /
                (m_position.norm() * other.m_position.norm()));
}

Matrix Orbitable::make_frame() {
    Vector e_zi = m_position.normalized().transpose();
    Vector e_zeta = angular_momentum().normalized().transpose();
    Vector e_eta = e_zeta.cross(e_zi).transpose();
    Matrix mat;
    mat << e_zi, e_eta, e_zeta;
    return mat;
}

double Orbitable::semi_major_axis() {
    double h = angular_momentum().norm();
    double e = eccentricity();
    return (h * h) / (SOLARGM * (1.0 - (e * e)));
}

double Orbitable::orbital_period() {
    double val = pow(semi_major_axis(), 3.0) / SOLARGM;
    return sqrt(val) * PI2;
}

double Orbitable::orbital_parameter() {
    double e = eccentricity();
    return semi_major_axis() * (1.0 - (e * e));
}

double Orbitable::eccentric_anomaly() {
    double e = eccentricity();
    double tht = true_anomaly();
    return 2.0 * atan(tan(tht / 2.0) / sqrt((1.0 + e) / (1.0 - e)));
}

double Orbitable::time_since_periapsis() {
    double t_anom = true_anomaly();
    double e_anom = true_to_eccentric(t_anom);
    double a = pow(semi_major_axis(), 3.0);
    double e = eccentricity();
    return sqrt(a / SOLARGM) * (e_anom - e * sin(e_anom));
}

double Orbitable::eccentricity() { return eccentricity_vector().norm(); }

double Orbitable::inclination() {
    Vector h = angular_momentum();
    return acos(h[2] / h.norm());
}

Vector Orbitable::ascending_node() {
    Vector k(0.0, 0.0, 1.0);
    return k.cross(angular_momentum());
}

double Orbitable::argument_of_periapsis() {
    Vector n = ascending_node();
    Vector e = eccentricity_vector();
    double omega = acos((n.dot(e)) / (n.norm() * e.norm()));
    if (e[2] < 0.0) {
        return PI2 - omega;
    } else {
        return omega;
    }
}

double Orbitable::argument_of_ascending_node() {
    Vector n = ascending_node();
    double n_x = n[0];
    double n_y = n[1];
    if (n_y >= 0.0) {
        return acos(n_x / n.norm());
    } else {
        return PI2 - acos(n_x / n.norm());
    }
}

double Orbitable::true_to_eccentric(double t_anom) {
    double a = semi_major_axis();
    double e = eccentricity();
    double b = a * sqrt(1.0 - pow(e, 2.0));
    double p = orbital_parameter();
    double r = p / (1.0 + e * cos(t_anom));
    double c = (a * e + r * cos(t_anom)) / a;
    double s = (r / b) * sin(t_anom);
    return atan2(s, c);
}

double Orbitable::true_anomaly_at_time(double time) {
    double t_peri = time_since_periapsis();
    double m_anom = mean_anomaly(time + t_peri);
    double angle = eccentric_from_mean(m_anom);
    return PI2 - eccentric_to_true_anomaly(angle);
}

double Orbitable::eccentric_from_mean(double m_anom) { return kepler(m_anom); }

double Orbitable::kepler(double m_anom) {
    double e = eccentricity();
    return elliptic_kepler(m_anom, e);
}

double Orbitable::eccentric_to_true_anomaly(double e_anom) {
    double e = eccentricity();
    double e_sqrt = sqrt((1.0 + e) / (1.0 - e));
    return 2.0 * atan(e_sqrt * tan(e_anom / 2.0));
}

double Orbitable::mean_anomaly(double time) {
    double n = sqrt(SOLARGM / pow(semi_major_axis(), 3));
    return n * time;
}

double Orbitable::distance_to(Orbitable other) {
    double arg_of_peri = argument_of_periapsis();
    double inc = inclination();
    double arg_of_an = argument_of_ascending_node();
    Matrix t_mat = three_one_three_transform(arg_of_peri, inc, arg_of_an);
    Vector d_vec = t_mat * other.m_position - t_mat * m_position;
    return abs(d_vec.norm());
}

Matrix three_one_three_transform(double omega, double inc, double tht) {
    Matrix m_c(3, 3);
    Matrix m_b(3, 3);
    Matrix m_a(3, 3);

    m_c << cos(omega), sin(omega), 0.0, -sin(omega), cos(omega), 0.0, 0.0, 0.0,
        1.0;

    m_b << 1.0, 0.0, 0.0, 0.0, cos(inc), sin(inc), 0.0, -sin(inc), cos(inc);

    m_a << cos(tht), sin(tht), 0.0, -sin(tht), cos(tht), 0.0, 0.0, 0.0, 1.0;

    return m_c * m_b * m_a;
}

double elliptic_kepler(double nt, double eccen) {
    double tolerance = 1.0e-12;
    auto kep = [&](double E) { return E - eccen * sin(E) - nt; };
    auto kep_d = [&](double E) { return 1.0 - eccen * cos(E); };
    double e_0 = 0.0;
    double e = e_0 - (kep(e_0) / kep_d(e_0));
    while (abs(e - e_0) > tolerance) {
        e_0 = e;
        e = e_0 - (kep(e_0) / kep_d(e_0));
    }
    return e;
}