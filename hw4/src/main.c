#include <stdio.h>
#include "body.h"
#include "matrix.h"

int main() {
    Body *ryugu =
        body(col_vector(3, 1.132759321672478E+08, -1.733831194873283E+08,
                        1.676906176312257E+07),
             col_vector(3, 1.804598259087825E+01, 1.405146782685940E+01,
                        1.309063084989150E+00));

    double ryugu_SMA = semi_major_axis(ryugu);
    Matrix *eccen_vec = eccentricity_vec(ryugu);
    double SMA_true_anomaly = true_anomaly(ryugu);
}