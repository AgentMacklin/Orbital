#include "body.h"
#include <stdio.h>
#include "math.h"

#define sqr(x) x *x
#define SOLARGM 1.328905188132376e20
#define RAD2DEG (180.0 / acos(-1.0))

/**
 * This function constructs a body pointer since doing it
 * the old fashioned way with an initializer list is doesn't
 * seem to want to work
 */
Body *body(Matrix *position, Matrix *velocity) {
    if (position == NULL) {
        perror("Body constructer was passed a null position vector");
        return NULL;
    } else if (velocity == NULL) {
        perror("Body constructer was passed a null velocity vector");
        return NULL;
    } else {
        Body *new_body = malloc(sizeof(Body));
        new_body->position = position;
        new_body->velocity = velocity;
        return new_body;
    }
}

Matrix *radial_velocity(Body *body) {
    double norm = magnitude(body->position);
    double dot_prod = dot(body->velocity, body->position);
    double scale = dot_prod / (norm * norm);
    return scale_mat(body->position, scale);
}

Matrix *tangent_velocity(Body *body) {
    Matrix *omeg = omega(body);
    Matrix *mat = cross(omeg, body->position);
    free(omeg);
    return mat;
}

double true_anomaly(Body *body) {
    Matrix *e_vec = eccentricity_vec(body);
    Matrix *position = normalize(body->position);
    double angle =
        dot(e_vec, position) / (magnitude(e_vec) * magnitude(body->position));
    free(e_vec);
    free(position);
    return acos(angle) * RAD2DEG;
}

Matrix *eccentricity_vec(Body *body) {
    Matrix *temp_vec = cross(body->velocity, angular_momentum(body));
    Matrix *lhs_mat = divide(temp_vec, SOLARGM);
    Matrix *norm_position = normalize(body->position);
    Matrix *result = matrix_sub(lhs_mat, norm_position);
    free(temp_vec);
    free(lhs_mat);
    free(norm_position);
    return result;
}

Matrix *angular_momentum(Body *body) {
    return cross(body->position, body->velocity);
}

double total_energy(Body *body) {
    double posit_mag = magnitude(body->position);
    double veloc_mag = magnitude(body->position);
    return 0.5 * (veloc_mag * veloc_mag) - (SOLARGM / posit_mag);
}

Matrix *omega(Body *body) {
    Matrix *ang_moment = angular_momentum(body);
    double posit_sqr = sqr(magnitude(body->position));
    Matrix *result = divide(ang_moment, posit_sqr);
    free(ang_moment);
    return result;
}