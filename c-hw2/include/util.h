#ifndef LINALG
#define LINALG

#define PI 3.141592653589793

typedef struct {
  int nelems;
  double *elements;
} Vector;

typedef struct {
  int nrows;
  int ncols;
  double **elements;
} Matrix;

Vector cross(Vector *v1, Vector *v2);
double dot(Vector *v1, Vector *v2);
double sqr(double x);
double magnitude(Vector *Vector);
Vector normalize(Vector *Vector);

Vector vec_div(Vector *Vector, double scalar);
Vector vec_mult(Vector *vec1, Vector *vec2);
Vector vec_add(Vector *vec1, Vector *vec2);
Vector vec_sub(Vector *vec1, Vector *vec2);
double to_degrees(double radians);

#endif // LINALG

#if !defined(ORBITAL)
#define ORBITAL

#define SOLARGM 1.328905188132376e11

typedef struct {
  Vector *position;
  Vector *velocity;
} Body;

Vector radial_velocity(Body *body, double radial_mag);
Vector tangent_velocity(Body *body);
double true_anomaly(Body *body);
Vector eccentricity_vec(Body *body);
Vector angular_momentum(Body *body);
double total_energy(Body *body);
Vector omega(Body *body);
double frame_rotation_rate(Body *body);
Vector relative_velocity(Body *body1, Body *body2);

Matrix transform(Vector *frame1[3], Vector *frame2[3]);

void print_vec(const char *msg, Vector *Vector);
void print_scaler(const char *msg, double scalar);
void print_matrix(const char *msg, Matrix *mat);

#endif // ORBITAL
