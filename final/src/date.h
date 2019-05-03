/**
 * date.h defines a Gregorian struct which represents a Gregorian date and defines
 * its << operator from streams. Functions for converting dates are also defined
 * here, as well as a function to slice C++ vectors for the Julian to Gregorian function
 */

#ifndef DATE_H
#define DATE_H

#include <cmath>
#include <iostream>
#include <vector>

// Something I pulled off the internet to slice a C++ vector
template <typename T>
std::vector<T> slice(std::vector<T> const& v, int m, int n) {
    auto first = v.cbegin() + m;
    auto last = v.cbegin() + n + 1;
    std::vector<T> Vector(first, last);
    return Vector;
}

class Gregorian {
   public:
    int year;
    int month;
    int day;
    int hour;
    int minute;
    double second;
    
    Gregorian(int y, int m, int d, int h, int mon, double s) :
        year(y), 
        month(m), 
        day(d), 
        hour(h), 
        minute(mon), 
        second(s) {}; 

    // Define how to print out a Gregorian object
    friend std::ostream& operator<<(std::ostream& out, Gregorian const& date) {
        out << "Year:    " << date.year << std::endl;
        out << "Month:   " << date.month << std::endl;
        out << "Day:     " << date.day << std::endl;
        out << "Hour:    " << date.hour << std::endl;
        out << "Minute:  " << date.minute << std::endl;
        out << "Second:  " << date.second;  // << std::endl;
        return out;
    }
};

/**
 * Convert a Gregorian date to a Julian
 */
double gregorian_to_julian(Gregorian& date) {
    return (367.0 * date.year) -
           trunc(7.0 * (trunc(date.year + (date.month + 9.0) / 12.0)) / 4.0) +
           trunc((275.0 * date.month) / 9.0) + 1721013.5 + date.day +
           (((((date.second / 60.0) + date.minute) / 60.0) + date.hour) / 24.0);
}

/**
 * Convert a Julian date to a Gregorian
 */
Gregorian julian_to_greg(double julian) {
    std::vector<double> l_months = {31.0, 28.0, 31.0, 30.0, 31.0, 30.0,
                                    31.0, 31.0, 30.0, 31.0, 30.0, 31.0};
    double t_1900 = (julian - 2415019.5) / 365.25;
    double year = 1900.0 + trunc(t_1900);
    double leap_years = trunc(0.25 * (year - 1900.0 - 1.0));
    double days = (julian - 2415019.5) - (365.0 * (year - 1900.0) + leap_years);
    if (days < 1.0) {
        year -= 1.0;
        leap_years = 0.25 * (year - 1900.0 - 1.0);
        days = (julian - 2415019.5) - (365.0 * (year - 1900.0) + leap_years);
    }
    if ((int)year % 4 == 0) {
        l_months[2] = 29.0;
    }
    double day_of_year = trunc(days);
    double sum_days = 0.0;
    int i = 0;
    while ((sum_days + 1.0) < day_of_year) {
        sum_days += l_months[i];
        i += 1;
    }
    double month = (double)i;
    std::vector<double> l_month_sum = slice(l_months, 0, i - 2);
    double m_sum = 0.0;
    for (double month : l_month_sum) {
        m_sum += month;
    }
    double day = day_of_year - m_sum;
    double tau = (days - day_of_year) * 24.0;
    double hour = trunc(tau);
    double minute = trunc((tau - hour) * 60.0);
    double second = (tau - hour - (minute / 60.0)) * 3600.0;
    return Gregorian{(int)year, (int)month,  (int)day,
                     (int)hour, (int)minute, second};
}

#endif  // DATE_H