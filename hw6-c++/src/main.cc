/**
 * Austen LeBeau
 * ENGR 3310-002
 *
 * Main entry point for the program
 */

#include <iomanip>
#include <iostream>
#include "lambert.h"
#include "date.h"
#include "orbitable.h"
#include "progress.h"
#include "util.h"

#define RAD2DEG(x) x*(180.0 / M_PI)

int main() {
    Vector r (12000, 12000, 0);
    Vector r_0 (15000, 0, 0);
    LambertResults test = practice_lambert(r_0, r, 76 * 3600);
    return 0;
}
