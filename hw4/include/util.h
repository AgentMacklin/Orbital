/**
 * util has some helper functions that don't have anything
 * to do with math, but check pointers, handle errors,
 * calculate stuff that makes formatting output easier,
 * etc.
 *
 * Currently the only error handling here is printing a
 * message and exiting the whole program with EXIT_FAILURE.
 * So, not really the most elegant error handling, but
 * pointer checking is in place.
 */
#ifndef UTIL_H
#define UTIL_H
#include <stdarg.h>
#include <stdbool.h>
#include <stdlib.h>

bool check_for_nulls(int num_ptrs, ...);
int data_len(double data);

#endif // UTIL_H
