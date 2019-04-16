/**
 * Austen LeBeau
 * ENGR 3310-002
 * 
 * Implementation of the Orbitable class.
 */


#include "orbitable.hpp"
#include <cmath>


#define SOLARGM 2.963092749241593e-4
#define PI2 (M_PI * 2.0)

#define SQR(x) x* x


Vec Orbitable::radial_velocity() {
    return (velocity.dot(position) / position.squaredNorm()) * position;
}

Vec Orbitable::tangential_velocity() { return omega().cross(position); }

double Orbitable::true_anomaly() {
    Vec e_vec = eccentricity_vector();
    Vec p = position.normalized();
    double t_anom = e_vec.dot(p) / (e_vec.norm() * p.norm());
    if (p.dot(velocity.normalized()) < 0.0) {
        return PI2 - acos(t_anom);
    } else {
        return acos(t_anom);
    }
}

Vec Orbitable::position_at_time(double time) {
    double t_anom = true_anomaly_at_time(time);
    double omega = argument_of_periapsis() - t_anom;
    double inc = inclination();
    double tht = argument_of_ascending_node();
    Mat t_mat =
        three_one_three_transform(omega, inc, tht).inverse();
    Vec p = position_at_angle(t_anom);
    return t_mat * p;
}

Vec Orbitable::velocity_at_time(double time) {
    double t_anom = true_anomaly_at_time(time);
    double omega = argument_of_periapsis() - t_anom;
    double inc = inclination();
    double tht = argument_of_ascending_node();
    Mat t_mat =
        three_one_three_transform(omega, inc, tht).inverse();
    Vec v = velocity_at_angle(t_anom);
    return t_mat * v;
}

Vec Orbitable::position_at_angle(double angle) {
    double e = eccentricity();
    double p = orbital_parameter();
    double radius = p / (1.0 + e * cos(angle));
    return Vec(radius, 0.0, 0.0);
}

Vec Orbitable::velocity_at_angle(double angle) {
    Mat t_frame = make_frame();
    Vec h = t_frame * angular_momentum();
    double coeff = h.norm() / orbital_parameter();
    double e = eccentricity();
    return Vec(coeff * -e * sin(angle), coeff * (1.0 + e * cos(angle)), 0.0);
}

Vec Orbitable::eccentricity_vector() {
    Vec h = angular_momentum();
    return (velocity.cross(h) / SOLARGM) - position.normalized();
}

Vec Orbitable::angular_momentum() { return position.cross(velocity); }

double Orbitable::total_energy() {
    return 0.5 * SQR(velocity.norm()) - (SOLARGM / position.norm());
}

Vec Orbitable::omega() { return angular_momentum() / position.squaredNorm(); }

double Orbitable::frame_rotation_rate() { return omega().norm(); }

double Orbitable::angle_to(Orbitable other) {
    return acos(position.dot(other.position) /
                (position.norm() * other.position.norm()));
}

Mat Orbitable::make_frame() {
    Vec e_zi = position.normalized().transpose();
    Vec e_zeta = angular_momentum().normalized().transpose();
    Vec e_eta = e_zeta.cross(e_zi).transpose();
    Mat mat;
    mat << e_zi, e_eta, e_zeta;
    return mat;
}

double Orbitable::semi_major_axis() {
    double h = angular_momentum().norm();
    double e = eccentricity();
    return SQR(h) / (SOLARGM * (1.0 - SQR(e)));
}

double Orbitable::orbital_period() {
    double val = pow(semi_major_axis(), 3.0) / SOLARGM;
    return sqrt(val) * PI2;
}

double Orbitable::orbital_parameter() {
    double e = eccentricity();
    return semi_major_axis() * (1.0 - SQR(e));
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
    Vec h = angular_momentum();
    return acos(h[2] / h.norm());
}

Vec Orbitable::ascending_node() {
    Vec k(0.0, 0.0, 1.0);
    return k.cross(angular_momentum());
}

double Orbitable::argument_of_periapsis() {
    Vec n = ascending_node();
    Vec e = eccentricity_vector();
    double omega = acos((n.dot(e)) / (n.norm() * e.norm()));
    if (e[2] < 0.0) {
        return PI2 - omega;
    } else {
        return omega;
    }
}

double Orbitable::argument_of_ascending_node() {
    Vec n = ascending_node();
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
    double b = a * (1.0 - sqrt(pow(e, 2.0)));
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
    Mat t_mat =
        three_one_three_transform(arg_of_peri, inc, arg_of_an);
    Vec d_vec = t_mat * other.position - t_mat * position;
    return abs(d_vec.norm());
}

Mat three_one_three_transform(double omega, double inc,
                                          double tht) {
    Mat m_c(3, 3);
    Mat m_b(3, 3);
    Mat m_a(3, 3);
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