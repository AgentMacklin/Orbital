/**
 * Austen LeBeau
 * ENGR 3310-002
 *
 * Defines the Orbitable class, which is the bread and butter of this program.
 * It takes in two vectors (defined by the Eigen library) for m_position and
 * m_velocity, and uses methods to return everything else.
 */

#ifndef BODY_H
#define BODY_H

// Linear algebra library
#include <Eigen/Dense>
#include <cmath>

// changing type names to something more convenient
typedef Eigen::Matrix3d Matrix;
typedef Eigen::Vector3d Vector;
typedef Eigen::RowVector3d RowVector;

/**
 * Class declaration
 */
class Orbitable {
   private:
    Vector m_position;
    Vector m_velocity;

   public:
    // basic constructor
    Orbitable(Vector p, Vector v) : m_position(p), m_velocity(v){};

    // Make a transformation matrix from a body's orbital parameters
    Matrix make_frame();

    /**
     * Methods that return a vector
     */
    Vector radial_velocity();
    Vector tangential_velocity();
    Vector position_at_time(double time);
    Vector velocity_at_time(double time);
    Vector position_at_angle(double angle);
    Vector velocity_at_angle(double angle);
    Vector eccentricity_vector();
    Vector angular_momentum();
    Vector omega();
    Vector ascending_node();

    Vector current_position() { return m_position; }

    Vector current_velocity() { return m_velocity; }

    /**
     * Rest of the methods
     */
    double total_energy();
    double true_anomaly();
    double frame_rotation_rate();
    double semi_major_axis();
    double orbital_period();
    double orbital_parameter();
    double eccentric_anomaly();
    double time_since_periapsis();
    double eccentricity();
    double inclination();
    double argument_of_periapsis();
    double argument_of_ascending_node();
    double true_to_eccentric(double t_anom);
    double true_anomaly_at_time(double time);
    double eccentric_from_mean(double m_anom);
    double kepler(double m_anom);
    double eccentric_to_true_anomaly(double e_anom);
    double mean_anomaly(double time);
    double distance_to(Orbitable body);
    double angle_to(Orbitable body);
};

// Create a three-one-three transformation matrix from a body's orbital
// parameters
Matrix three_one_three_transform(double omega, double inc, double tht);

// Newton's method
double elliptic_kepler(double nt, double e);

#endif  // BODY_H
