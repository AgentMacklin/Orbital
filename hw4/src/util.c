#include "util.h"
#include <math.h>
#include <stdio.h>

/**
 * Check for any null pointers
 */
bool check_for_nulls(int num_ptrs, ...) {
    va_list ptrs;
    va_start(ptrs, num_ptrs);
    for (int i = 0; i < num_ptrs; ++i) {
        // If a null pointer was found, return false
        if (va_arg(ptrs, void *)) {
            va_end(ptrs);
            perror("Null pointer was passed");
            return false;
        }
    }
    va_end(ptrs);
    // else if everything is good, return true
    return true;
}

/**
 * Number of whole digits for a double,
 * supposed to be used for formatting data
 * for pretty printing.
 */
int data_len(double data) {
    int n = (int)data;
    if (n == 0)
        return 1;
    // if data is negative, make room for negative sign
    else if (n < 0)
        return floor(log10(abs(n))) + 2;
    // else return number of whole digits
    else
        return floor(log10(abs(n))) + 1;
}
