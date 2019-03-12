#include "util.h"
#include <stdio.h>

int main() {
    Vector vec1 = Vector {.nelems = 3, [1, 1, 0]}; 
    Vector vec2 = Vector {.nelems = 3, [0, 1, 0]}; 
    double dotted_vec = dot(&vec1, &vec2);
    printf("%f", dotted_vec);
}
