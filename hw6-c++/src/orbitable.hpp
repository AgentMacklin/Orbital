/**
 * Austen LeBeau
 * ENGR 3310-002
 * 
 * Defines the Orbitable class, which is the bread and butter of this program.
 * It takes in two vectors (defined by the Eigen library) for position and
 * velocity, and uses methods to return everything else.
 */


#ifndef BODY_H
#define BODY_H


// Linear algebra library
#include <Eigen/Dense>

// changing type names to something more convenient
typedef Eigen::Matrix3d Mat;
typedef Eigen::Vector3d Vec;
typedef Eigen::RowVector3d RVec;

/**
 * Class declaration
 */
class Orbitable {

   private:
    Vec position;
    Vec velocity;

   public:

    // basic constructor
    Orbitable(Vec p, Vec v) : position(p), velocity(v) {};

    // Make a transformation matrix from a body's orbital parameters
    Mat make_frame();


    /**
     * Methods that return a vector
     */
    Vec radial_velocity();
    Vec tangential_velocity();
    Vec position_at_time(double time);
    Vec velocity_at_time(double time);
    Vec position_at_angle(double angle);
    Vec velocity_at_angle(double angle);
    Vec eccentricity_vector();
    Vec angular_momentum();
    Vec omega();
    Vec ascending_node();


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

// Create a three-one-three transformation matrix from a body's orbital parameters
Mat three_one_three_transform(double omega, double inc, double tht);

// Newton's method
double elliptic_kepler(double nt, double e);

#endif  // BODY_H
