#if !defined(BODY_H)
#define BODY_H

#include <Eigen/Dense>


typedef Eigen::Vector3d vec;

class Body {
private:
    vec position;
    vec velocity;


public: 

    Body(vec p, vec v) {
        this->position = p;
        this->velocity = v;
    };

    vec radial_velocity();
    vec tangential_velocity();
    double true_anomaly();
    vec position_at_time(double time);
    vec velocity_at_time(double time);
    vec position_at_angle(double angle);
    vec velocity_at_angle(double angle);
    vec eccentricity_vector();
    vec angular_momentum();
    double total_energy();
    vec omega();
    double frame_rotation_rate();
    double angle_to(Body body);
    Eigen::Matrix3d make_frame();
    double semi_major_axis();
    double orbital_period();
    double orbital_parameter();
    double eccentric_anomaly();
    double time_since_periapsis();
    double eccentricity();
    double inclination();
    vec ascending_node();
    double argument_of_periapsis();
    double argument_of_ascending_node();
    double true_to_eccentric(double t_anom);
    double true_anomaly_at_time(double time);
    double eccentric_from_mean(double m_anom);
    double kepler(double m_anom);
    double eccentric_to_true_anomaly(double e_anom);
    double mean_anomaly(double time);
    double distance_to(Body body);



};

Eigen::Matrix3d three_one_three_transform(double omega, double inc, double tht);

double elliptic_kepler(double nt, double e);

#endif // BODY_H
