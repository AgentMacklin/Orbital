#include "matrix.h"
#include <ctype.h>
#include <math.h>
#include <stdio.h>
#include "util.h"

/**
 * Basic matrix constructor, which takes in the size you
 * want and fills it with a value you pass in for fill.
 * I think trying to fill in a matrix using variadic
 * arguments would be too much of a pain to try to
 * implement and it would be clunky anyway.
 */
Matrix *matrix(size_t nrows, size_t ncols, ...) {
    Matrix *mat = malloc(sizeof(Matrix));
    mat->nrows = nrows;
    mat->ncols = ncols;
    // Allocating rows
    mat->elems = (double **)malloc(nrows * sizeof(double *));
    // Allocating columns
    for (size_t i = 0; i < nrows; ++i) {
        mat->elems[i] = (double *)malloc(ncols * sizeof(double));
    }
    va_list elements;
    va_start(elements, ncols);
    for (size_t i = 0; i < nrows; ++i) {
        for (size_t j = 0; j < ncols; ++j) {
            mat->elems[i][j] = va_arg(elements, double);
        }
    }
    va_end(elements);
    return mat;
}

Matrix *matrix_with_fill(size_t nrows, size_t ncols, double fill) {
    Matrix *mat = malloc(sizeof(Matrix));
    mat->nrows = nrows;
    mat->ncols = ncols;
    // Allocating rows
    mat->elems = (double **)malloc(nrows * sizeof(double *));
    // Allocating columns
    for (size_t i = 0; i < nrows; ++i) {
        mat->elems[i] = (double *)malloc(ncols * sizeof(double));
    }
    for (size_t i = 0; i < nrows; ++i) {
        for (size_t j = 0; j < ncols; ++j) {
            mat->elems[i][j] = fill;
        }
    }
    return mat;
}

/**
 * Constructing one-dimensional matrices. As far as I know I
 * don't think you can pass in variadic arguments to another
 * function, otherwise you could just call matrix() from
 * within row_vector() and col_vector()
 */
Matrix *row_vector(size_t ncols, ...) {
    Matrix *mat = malloc(sizeof(Matrix));
    mat->nrows = 1;
    mat->ncols = ncols;
    // Allocating rows
    mat->elems = (double **)malloc(mat->nrows * sizeof(double *));
    // Allocating columns
    for (size_t i = 0; i < mat->nrows; ++i) {
        mat->elems[i] = (double *)malloc(ncols * sizeof(double));
    }
    va_list elements;
    va_start(elements, ncols);
    for (size_t i = 0; i < mat->nrows; ++i) {
        for (size_t j = 0; j < ncols; ++j) {
            mat->elems[i][j] = va_arg(elements, double);
        }
    }
    va_end(elements);
    return mat;
}

Matrix *col_vector(size_t nrows, ...) {
    Matrix *mat = malloc(sizeof(Matrix));
    mat->nrows = nrows;
    mat->ncols = 1;
    // Allocating rows
    mat->elems = (double **)malloc(nrows * sizeof(double *));
    // Allocating columns
    for (size_t i = 0; i < mat->nrows; ++i) {
        mat->elems[i] = (double *)malloc(mat->ncols * sizeof(double));
    }
    va_list elements;
    va_start(elements, nrows);
    for (size_t i = 0; i < nrows; ++i) {
        for (size_t j = 0; j < mat->ncols; ++j) {
            mat->elems[i][j] = va_arg(elements, double);
        }
    }
    va_end(elements);
    return mat;
}

/**
 * Multiply two matrices and return a new one
 */
Matrix *multiply(Matrix *mat1, Matrix *mat2) {
    if (can_multiply_matrices(mat1, mat2)) {
        Matrix *new_mat = matrix_with_fill(mat1->nrows, mat2->ncols, 0.0);
        for (size_t i = 0; i < mat1->nrows; ++i) {
            for (size_t j = 0; j < mat2->ncols; ++j) {
                double value = 0;
                for (size_t k = 0; k < mat1->ncols; ++k) {
                    value += mat1->elems[i][k] * mat2->elems[k][j];
                }
                new_mat->elems[i][j] = value;
            }
        }
        return new_mat;
    } else
        return NULL;
}

Matrix *divide(Matrix *mat, double divisor) {
    Matrix *new_mat = matrix_with_fill(mat->nrows, mat->ncols, 0.0);
    for (size_t i = 0; i < mat->nrows; ++i) {
        for (size_t j = 0; j < mat->ncols; ++j) {
            new_mat->elems[i][j] /= divisor;
        }
    }
    return new_mat;
}

Matrix *matrix_add(Matrix *mat1, Matrix *mat2) {
    Matrix *new_mat = matrix_with_fill(mat1->nrows, mat1->ncols, 0.0);
    for (size_t i = 0; i < mat1->nrows; ++i) {
        for (size_t j = 0; j < mat1->ncols; ++j) {
            new_mat->elems[i][j] = mat1->elems[i][j] + mat2->elems[i][j];
        }
    }
    return new_mat;
}

Matrix *matrix_sub(Matrix *mat1, Matrix *mat2) {
    Matrix *new_mat = matrix_with_fill(mat1->nrows, mat1->ncols, 0.0);
    for (size_t i = 0; i < mat1->nrows; ++i) {
        for (size_t j = 0; j < mat1->ncols; ++j) {
            new_mat->elems[i][j] = mat1->elems[i][j] - mat2->elems[i][j];
        }
    }
    return new_mat;
}

/**
 * Return the biggest element
 */
double max_element(Matrix *mat) {
    check_for_nulls(1, mat);
    double max = 0;
    for (size_t i = 0; i < mat->nrows; ++i) {
        for (size_t j = 0; j < mat->nrows; ++j) {
            max = mat->elems[i][j] > max ? mat->elems[i][j] : max;
        }
    }
    return max;
}

/**
 * Multiply all elements of a matrix by a scalar
 */
Matrix *scale_mat(Matrix *mat, double scalar) {
    check_for_nulls(1, mat);
    Matrix *new_mat = matrix(mat->nrows, mat->ncols);
    for (size_t i = 0; i < mat->nrows; ++i) {
        for (size_t j = 0; j < mat->ncols; ++j) {
            new_mat->elems[i][j] = mat->elems[i][j] * scalar;
        }
    }
    return new_mat;
}

/**
 * Pretty print formatted matrix, doesn't really work well yet.
 * The plan is to format by columns, where all elements in a
 * column are right-justified, and the width of the column is
 * the width of the largest value plus some padding. But this
 * works for now.
 */
void print_mat(Matrix *mat) {
    if (mat == NULL) {
        perror("Null Matrix pointer was passed to print_mat");
    } else {
        for (size_t i = 0; i < mat->nrows; ++i) {
            for (size_t j = 0; j < mat->ncols; ++j) {
                printf("%-12.5f ", mat->elems[i][j]);
            }
            printf("\n");
        }
    }
}

int vector_length(Matrix *mat) {
    // We know it's a vector, so return ncols if nrows == 1
    // or vice-versa
    if (is_vector(mat))
        return mat->nrows == 1 ? mat->ncols : mat->nrows;
    else
        return NOT_VECTOR;
}

/**
 * Yikes
 */
double dot(Matrix *mat1, Matrix *mat2) {
    if (can_dot_matrices(mat1, mat2)) {
        double sum = 0;
        double e1, e2;
        for (int i = 0; i < vector_length(mat1); ++i) {
            e1 = vector_type(mat1) == ROW ? mat1->elems[0][i]
                                          : mat1->elems[i][0];
            e2 = vector_type(mat2) == ROW ? mat2->elems[0][i]
                                          : mat2->elems[i][0];
            sum += e1 * e2;
        }
        return sum;
    } else
        return NOT_VECTOR;
}

bool can_dot_matrices(Matrix *mat1, Matrix *mat2) {
    int vec1 = vector_length(mat1);
    int vec2 = vector_length(mat2);
    if (!is_vector(mat1))
        return false;
    else if (!is_vector(mat2))
        return false;
    else if (vec1 != vec2)
        return false;
    else
        return true;
}

bool is_vector(Matrix *mat) {
    if (mat->nrows > 1 && mat->ncols != 1)
        return false;
    else if (mat->ncols > 1 && mat->nrows != 1)
        return false;
    else
        return true;
}

/**
 * Check if two column sizes are the right size
 */
bool check_column_sizes(size_t ncols1, size_t ncols2) {
    return ncols1 != ncols2;
}

/**
 * Checks whether two matrices can actually be multiplied
 */
bool can_multiply_matrices(Matrix *mat1, Matrix *mat2) {
    return mat1->ncols == mat2->nrows;
}

int vector_type(Matrix *mat) {
    if (is_vector(mat))
        return mat->nrows == 1 ? ROW : COL;
    else
        return NOT_VECTOR;
}

double magnitude(Matrix *vec) {
    if (is_vector(vec)) {
        double sqr_sum = 0;
        for (size_t i = 0; i < (size_t)vector_length(vec); ++i) {
            if (vector_type(vec) == ROW)
                sqr_sum += vec->elems[0][i] * vec->elems[0][i];
            else if (vector_type(vec) == COL)
                sqr_sum += vec->elems[i][0] * vec->elems[i][0];
        }
        return sqrt(sqr_sum);
    } else {
        perror("Matrix passed to magnitude() was not a vector");
        return -1;
    }
}

Matrix *normalize(Matrix *vec) {
    if (is_vector(vec)) {
        double mag = magnitude(vec);
        Matrix *new_vec = matrix_with_fill(vec->nrows, vec->ncols, 0.0);
        for (size_t i = 0; i < (size_t)vector_length(vec); ++i) {
            if (vector_type(vec) == ROW)
                new_vec->elems[0][i] = vec->elems[i][0] / mag;
            else if (vector_type(vec) == COL)
                new_vec->elems[0][i] = vec->elems[i][0] / mag;
        }
        return new_vec;
    } else {
        return NULL;
    }
}

Matrix *transpose(Matrix *mat) {
    if (mat != NULL) {
        Matrix *t_mat = matrix_with_fill(mat->ncols, mat->nrows, 0.0);
        for (size_t i = 0; i < mat->nrows; ++i) {
            for (size_t j = 0; j < mat->ncols; ++j) {
                t_mat->elems[j][i] = mat->elems[i][j];
            }
        }
        return t_mat;
    } else {
        perror("Null Matrix pointer was passed to transpose");
        return NULL;
    }
}

Matrix *copy_matrix(Matrix *mat) {
    Matrix *new_mat = matrix(mat->nrows, mat->ncols);
    for (size_t i = 0; i < mat->nrows; ++i) {
        for (size_t j = 0; j < mat->ncols; ++j) {
            new_mat->elems[i][j] = mat->elems[i][j];
        }
    }
    return new_mat;
}

Matrix *cross(Matrix *mat1, Matrix *mat2) {
    Matrix *vec1, *vec2;
    if (!is_vector(mat1) || !is_vector(mat2)) {
        printf("One of the matrices is not a vector.\n");
        return NULL;
    } else if ((vector_length(mat1) != 3) || (vector_length(mat2) != 3)) {
        printf("One of the matrices is not the proper length.\n");
        return NULL;
    } else {
        vec1 = vector_type(mat1) == ROW ? copy_matrix(mat1)
                                        : copy_matrix(transpose(mat1));
        vec2 = vector_type(mat2) == ROW ? copy_matrix(mat2)
                                        : copy_matrix(transpose(mat2));
        Matrix *crossed_vec = matrix(1, 3);
        crossed_vec->elems[0][0] = vec1->elems[0][1] * vec2->elems[0][2] -
                                   vec1->elems[0][2] * vec2->elems[0][1];
        crossed_vec->elems[0][1] = vec1->elems[0][2] * vec2->elems[0][0] -
                                   vec1->elems[0][0] * vec2->elems[0][2];
        crossed_vec->elems[0][2] = vec1->elems[0][0] * vec2->elems[0][1] -
                                   vec1->elems[0][1] * vec2->elems[0][0];
        free(vec1);
        free(vec2);
        return crossed_vec;
    }
}