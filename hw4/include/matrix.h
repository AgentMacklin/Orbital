/**
 * This library represents vectors as matrices that happen to have one
 * row or column. So in the interest of making it a bit simpler to look
 * at, row_vector() or col_vector() return a 1D matrix, so there are no
 * actual Vector structs.
 */

#ifndef MATRIX_H
#define MATRIX_H
#include <stdarg.h>
#include <stdbool.h>
#include <stdlib.h>

// Working out how to handle errors
typedef enum {
    INVALID_LENGTH = -1,
    NOT_VECTOR = -2,
} MatrixErrors;

/**
 * Used for checking whether a matrix is a vector,
 * and what type of vector since some operations
 * depend on the vector being a row or column vector.
 */
typedef enum { ROW, COL } VectorType;

/**
 * Main struct which the library depend on.
 *
 * It has a double pointer which points to two arrays. A Matrix's
 * elements can be accessed by doing: mat->elems[i][j]. If you use
 * the constructors, be aware that they manually allocate the memory
 * using malloc. If you use temporary matrices in a function, make
 * sure to free the memory before the function ends. Otherwise not
 * even God can help you.
 */
typedef struct {
    size_t nrows;
    size_t ncols;
    double **elems;
} Matrix;

/**
 * Basic matrix construction. Accepts variadic arguments,
 * which matrix() uses to fill in the new matrix row by row.
 * So, calling matrix(3, 3, 1, 2, 3, 6, 5, 4, 8, 4, 5)
 * results in the following matrix:
 *
 *                        |1   2   3|
 *               matrix = |6   5   4|
 *                        |8   4   5|
 */
Matrix *matrix(size_t nrows, size_t ncols, ...);

/**
 * Like matrix() but fills all elements with one value
 */
Matrix *matrix_with_fill(size_t nrows, size_t ncols, double fill);

/**
 * Will construct "vectors", so 1D matrices. row_vector
 * will create a matrix like so:
 *
 *               matrix = |0, 3, 4, ...|
 *
 * Likewise, col_vector will create:
 *
 *                        | 4 |
 *               matrix = | 3 |
 *                        | 7 |
 *                        |...|
 */
Matrix *row_vector(size_t ncols, ...);
Matrix *col_vector(size_t nrows, ...);

Matrix *cross(Matrix *vec1, Matrix *vec2);
Matrix *normalize(Matrix *vec); // Assuming this is a vector
Matrix *transpose(Matrix *mat);
Matrix *scale_mat(Matrix *mat, double scalar);
Matrix *multiply(Matrix *mat1, Matrix *mat2);
Matrix *divide(Matrix *mat1, double divisor);
Matrix *matrix_add(Matrix *mat1, Matrix *mat2);
Matrix *matrix_sub(Matrix *mat1, Matrix *mat2);

double magnitude(Matrix *vec);
double dot(Matrix *vec1, Matrix *vec2);
double max_element(Matrix *mat);

bool is_vector(Matrix *mat);
bool can_dot_matrices(Matrix *mat1, Matrix *mat2);
bool check_column_sizes(size_t ncols1, size_t ncols2);
bool can_multiply_matrices(Matrix *mat1, Matrix *mat2);

int vector_length(Matrix *mat);
int vector_type(Matrix *mat);

void print_mat(Matrix *mat);

#endif // MATRIX_H
