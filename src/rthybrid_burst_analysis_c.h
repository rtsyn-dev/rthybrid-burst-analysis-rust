#pragma once
#include <stddef.h>

typedef struct {
  double observation_time;
  double vm;
  double min;
  double max;
  double temp_min;
  double temp_max;
  double count;
  double thresh_up;
  double thresh_down;
  double range;
  double pts_counter;
  double burst_counter;
  double is_burst;
  double burst_dur_sum;
  double old_burst_time;
  double sec_per_burst;
  double out_min;
  double out_max;
  double out_burst_duration;
} rthybrid_burst_analysis_c_state_t;

void rthybrid_burst_analysis_c_init(rthybrid_burst_analysis_c_state_t *s);
void rthybrid_burst_analysis_c_set_config(rthybrid_burst_analysis_c_state_t *s, const char *key, size_t len, double v);
void rthybrid_burst_analysis_c_set_input(rthybrid_burst_analysis_c_state_t *s, const char *key, size_t len, double v);
void rthybrid_burst_analysis_c_process(rthybrid_burst_analysis_c_state_t *s, double period_seconds);
double rthybrid_burst_analysis_c_get_output(const rthybrid_burst_analysis_c_state_t *s, const char *key, size_t len);
