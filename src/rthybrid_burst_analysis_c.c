#include "rthybrid_burst_analysis_c.h"
#include <math.h>
#include <string.h>

void rthybrid_burst_analysis_c_init(rthybrid_burst_analysis_c_state_t *s) {
  s->observation_time = 5.0;
  s->vm = 0.0;
  s->min = 0.0;
  s->max = 0.0;
  s->temp_min = 999999.0;
  s->temp_max = -999999.0;
  s->count = 0.0;
  s->thresh_up = 0.0;
  s->thresh_down = 0.0;
  s->range = 0.0;
  s->pts_counter = 0.0;
  s->burst_counter = -1.0;
  s->is_burst = 1.0;
  s->burst_dur_sum = 0.0;
  s->old_burst_time = 0.0;
  s->sec_per_burst = 0.0;
  s->out_min = 0.0;
  s->out_max = 0.0;
  s->out_burst_duration = 0.0;
}

void rthybrid_burst_analysis_c_set_config(rthybrid_burst_analysis_c_state_t *s, const char *key, size_t len, double v) {
  if ((len == 16 && strncmp(key, "observation_time", len) == 0) ||
      (len == 20 && strncmp(key, "Observation time (s)", len) == 0)) {
    s->observation_time = v > 0.001 ? v : 0.001;
  }
}

void rthybrid_burst_analysis_c_set_input(rthybrid_burst_analysis_c_state_t *s, const char *key, size_t len, double v) {
  if (len == 6 && strncmp(key, "Vm (V)", len) == 0 && isfinite(v)) {
    s->vm = v;
  }
}

void rthybrid_burst_analysis_c_process(rthybrid_burst_analysis_c_state_t *s, double period_seconds) {
  if (period_seconds <= 0.0) {
    return;
  }

  double time_s = s->count * period_seconds;
  double freq = 1.0 / period_seconds;

  if (time_s > s->observation_time) {
    s->min = s->temp_min;
    s->max = s->temp_max;
    if (isfinite(s->min) && isfinite(s->max)) {
      s->out_min = s->min;
      s->out_max = s->max;
    }
    s->temp_min = 999999.0;
    s->temp_max = -999999.0;
    s->count = 0.0;

    if (s->burst_counter > 0.0) {
      s->sec_per_burst = (s->burst_dur_sum / s->burst_counter) / freq;
    } else {
      s->sec_per_burst = 0.0;
    }
    if (isfinite(s->sec_per_burst)) {
      s->out_burst_duration = s->sec_per_burst;
    } else {
      s->out_burst_duration = 0.0;
    }

    s->pts_counter = 0.0;
    s->burst_counter = 0.0;
    s->burst_dur_sum = 0.0;
    s->old_burst_time = 0.0;
  }

  if (s->vm < s->temp_min) s->temp_min = s->vm;
  if (s->vm > s->temp_max) s->temp_max = s->vm;

  s->range = s->max - s->min;
  s->thresh_down = s->min + (s->range * 0.1);
  s->thresh_up = s->min + (s->range * 0.9);

  if (s->is_burst == 0.0 && s->vm > s->thresh_up) {
    s->is_burst = 1.0;
    s->burst_counter += 1.0;
    s->burst_dur_sum += s->pts_counter - s->old_burst_time;
    s->old_burst_time = s->pts_counter;
  } else if (s->is_burst == 1.0 && s->vm < s->thresh_down) {
    s->is_burst = 0.0;
  }

  s->pts_counter += 1.0;
  s->count += 1.0;
}

double rthybrid_burst_analysis_c_get_output(const rthybrid_burst_analysis_c_state_t *s, const char *key, size_t len) {
  if (len == 7 && strncmp(key, "Min (V)", len) == 0) return s->out_min;
  if (len == 7 && strncmp(key, "Max (V)", len) == 0) return s->out_max;
  if (len == 18 && strncmp(key, "Burst duration (s)", len) == 0) return s->out_burst_duration;
  return 0.0;
}
