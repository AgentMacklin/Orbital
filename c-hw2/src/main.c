#include "util.h"
#include <stdio.h>

int main() {
  Body new_horizons = {
      .position = &(Vector){.nelems = 3,
                            .elements = (double[3]){1.801739848920305e9,
                                                    -6.213258711638543e9,
                                                    2.206937879367335e8}},
      .velocity = &(Vector){.nelems = 3,
                            .elements = (double[3]){5.470110892244167,
                                                    -1.297147544331515e1,
                                                    5.086892733753903e-1}},
  };

  Body ultima_thule = {
      .position = &(Vector){.nelems = 3,
                            .elements = (double[3]){1.801762521239009e9,
                                                    -6.213547109786671e9,
                                                    2.207029697048225e8}},
      .velocity = &(Vector){.nelems = 3,
                            .elements = (double[3]){4.356656512602084e0,
                                                    1.406772063575476e0,
                                                    -1.228651055471098e-1}},
  };

  Vector ut_ang_moment = angular_momentum(&ultima_thule);

  Vector e_r = normalize(ultima_thule.position);
  Vector e_h = normalize(&ut_ang_moment);
  Vector e_tht = cross(&e_r, &e_h);

  /* Unit vectors in the B frame */
  Vector e_1 = (Vector){3, (double[3]){1.0, 0.0, 0.0}};
  Vector e_2 = (Vector){3, (double[3]){0.0, 1.0, 0.0}};
  Vector e_3 = (Vector){3, (double[3]){0.0, 0.0, 1.0}};

  Vector *i_units[3] = {&e_r, &e_tht, &e_h};
  Vector *b_units[3] = {&e_1, &e_2, &e_3};

  Matrix trans_mat = transform(i_units, b_units);

  //   print_vec("New Horizons Position", new_horizons.position);
  print_vec("e_r", &e_r);
  //   print_vec("e_h", &e_h);
  //   print_vec("e_tht", &e_tht);
  //   print_matrix("Transformation Matrix", &trans_mat);
}