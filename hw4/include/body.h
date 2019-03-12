#ifndef BODY_H
#define BODY_H
#include "matrix.h"

typedef struct {
    Matrix *position;
    Matrix *velocity;
} Body;

Body *body(Matrix *position, Matrix *velocity);

/* Not everything is implemented yet */
Matrix *radial_velocity(Body *body);
Matrix *tangent_velocity(Body *body);
Matrix *make_frame(Body *body);
Matrix *eccentricity_vec(Body *body);
Matrix *angular_momentum(Body *body);
Matrix *omega(Body *body);
Matrix *position_at_angle(Body *body, double angle);
Matrix *velocity_at_angle(Body *body, double angle);

double true_anomaly(Body *body);
double total_energy(Body *body);
double frame_rotation_rate(Body *body);
double angle_to(Body *body1, Body *body2);
double semi_major_axis(Body *body);
double orbital_parameter(Body *body);

#endif  // BODY_H
