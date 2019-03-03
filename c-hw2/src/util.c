#include "util.h"
#include "math.h"
#include <stdio.h>

double to_degrees(double radians) { return radians * (180 / PI); }

double sqr(double x) { return x * x; }

double magnitude(Vector *vec) {
  double sqr_sum = 0;
  for (int i = 0; i < vec->nelems; ++i) {
    sqr_sum += sqr(vec->elements[i]);
  }
  return sqrt(sqr_sum);
}

Vector vec_mult(Vector *vec1, Vector *vec2) {
  double new_vec[vec1->nelems];
  if (vec1->nelems == vec2->nelems) {
    for (int i = 0; i < vec1->nelems; ++i) {
      new_vec[i] = vec1->elements[i] * vec2->elements[i];
    }
  }
  return (Vector){.nelems = vec1->nelems, .elements = new_vec};
}

Vector vec_add(Vector *vec1, Vector *vec2) {
  double new_vec[vec1->nelems];
  if (vec1->nelems == vec2->nelems) {
    for (int i = 0; i < vec1->nelems; ++i) {
      new_vec[i] = vec1->elements[i] + vec2->elements[i];
    }
  }
  return (Vector){.nelems = vec1->nelems, .elements = new_vec};
}

Vector vec_sub(Vector *vec1, Vector *vec2) {
  double new_vec[vec1->nelems];
  if (vec1->nelems == vec2->nelems) {
    for (int i = 0; i < vec1->nelems; ++i) {
      new_vec[i] = vec1->elements[i] - vec2->elements[i];
    }
  }
  return (Vector){.nelems = vec1->nelems, .elements = new_vec};
}

Vector vec_div(Vector *vec, double scalar) {
  double new_vec[vec->nelems];
  for (int i = 0; i < vec->nelems; ++i) {
    new_vec[i] = vec->elements[i] / scalar;
  }
  return (Vector){.nelems = vec->nelems, .elements = new_vec};
}

Vector cross(Vector *vec1, Vector *vec2) {
  double new_vec[vec1->nelems];
  if (vec1->nelems == 3 && vec1->nelems == vec2->nelems) {
    new_vec[0] = vec1->elements[2] * vec2->elements[3] -
                 vec1->elements[3] * vec2->elements[2];
    new_vec[1] = -(vec1->elements[1] * vec2->elements[3] -
                   vec1->elements[3] * vec2->elements[1]);
    new_vec[2] = vec1->elements[1] * vec2->elements[2] -
                 vec1->elements[2] * vec2->elements[1];
  }
  return (Vector){.nelems = vec1->nelems, .elements = new_vec};
}

double dot(Vector *vec1, Vector *vec2) {
  double dot_product;
  if (vec1->nelems == vec2->nelems) {
    for (int i = 0; i < vec1->nelems; ++i) {
      dot_product += vec1->elements[i] * vec2->elements[i];
    }
  }
  return dot_product;
}

Vector normalize(Vector *vec) {
  double mag = magnitude(vec);
  double new_vec[vec->nelems];
  for (int i = 0; i < vec->nelems; ++i) {
    new_vec[i] = vec->elements[i] / mag;
  }
  return (Vector){.nelems = vec->nelems, .elements = new_vec};
}

/* what a mess */
Matrix transform(Vector *frame1[3], Vector *frame2[3]) {
  double new_mat[3][3];
  for (int i = 0; i < 3; ++i) {
    for (int j = 0; j < 3; ++j) {
      new_mat[i][j] = dot(frame2[j], frame1[i]);
    }
  }
  return (Matrix){.nrows = 3, .ncols = 3, .elements = (double **)new_mat};
}

Vector radial_velocity(Body *body, double radial_mag) {
  Vector norm_vec = normalize(body->position);
  for (int i = 0; i < norm_vec.nelems; ++i) {
    norm_vec.elements[i] * radial_mag;
  }
  return norm_vec;
}

Vector tangent_velocity(Body *body) {
  Vector body_omega = omega(body);
  return cross(&body_omega, body->position);
}

Vector omega(Body *body) {
  Vector ang_moment = angular_momentum(body);
  double position_mag = sqr(magnitude(body->position));
  return vec_div(&ang_moment, position_mag);
}

double true_anomaly(Body *body) {
  Vector e_vec = eccentricity_vec(body);
  Vector norm_position = normalize(body->position);
  double numerator = dot(&e_vec, &norm_position);
  double denominator = magnitude(&e_vec) * magnitude(body->position);
  double angle = numerator / denominator;
  return to_degrees(angle);
}

Vector angular_momentum(Body *body) {
  return cross(body->position, body->velocity);
}

double total_energy(Body *body) {
  double velocity = magnitude(body->velocity);
  double position = magnitude(body->position);
  return 0.5 * sqr(velocity) - (SOLARGM / position);
}

Vector eccentricity_vec(Body *body) {
  Vector ang_moment = angular_momentum(body);
  Vector norm_position = normalize(body->position);
  Vector crossed_vec = cross(body->velocity, &ang_moment);
  Vector div_crossed = vec_div(&crossed_vec, SOLARGM);
  return vec_sub(&div_crossed, &norm_position);
}

void print_vec(const char *msg, Vector *vec) {
  printf("%s:\n", msg);
  for (int i = 0; i < vec->nelems; ++i) {
    printf("  %.10e\n", vec->elements[i]);
  }
  printf("\n");
}

void print_scalar(const char *msg, double scalar) {
  printf("%s:\n  %.10e\n\n", msg, scalar);
}

void print_matrix(const char *msg, Matrix *mat) {
  printf("%s:\n", msg);
  for (int i = 0; i < mat->nrows; ++i) {
    for (int j = 0; j < mat->ncols; ++j) {
      printf("  %.10e", mat->elements[i][j]);
    }
    printf("\n");
  }
}