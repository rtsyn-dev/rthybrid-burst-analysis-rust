use rtsyn_plugin::prelude::*;
use serde_json::Value;

#[derive(Debug)]
struct RthybridBurstAnalysisRust {
    vm: f64,
    out_0: f64,
    out_1: f64,
    out_2: f64,
    observation_time: f64,
    min: f64,
    max: f64,
    temp_min: f64,
    temp_max: f64,
    count: f64,
    thresh_up: f64,
    thresh_down: f64,
    range: f64,
    pts_counter: f64,
    burst_counter: f64,
    is_burst: bool,
    burst_dur_sum: f64,
    old_burst_time: f64,
    sec_per_burst: f64,
}

impl Default for RthybridBurstAnalysisRust {
    fn default() -> Self {
        Self {
            vm: 0.0,
            out_0: 0.0,
            out_1: 0.0,
            out_2: 0.0,
            observation_time: 5.0,
            min: 0.0,
            max: 0.0,
            temp_min: 999_999.0,
            temp_max: -999_999.0,
            count: 0.0,
            thresh_up: 0.0,
            thresh_down: 0.0,
            range: 0.0,
            pts_counter: 0.0,
            burst_counter: -1.0,
            is_burst: true,
            burst_dur_sum: 0.0,
            old_burst_time: 0.0,
            sec_per_burst: 0.0,
        }
    }
}

impl PluginDescriptor for RthybridBurstAnalysisRust {
    fn name() -> &'static str {
        "RTHybrid Burst Analysis"
    }

    fn kind() -> &'static str {
        "rthybrid_burst_analysis"
    }

    fn plugin_type() -> PluginType {
        PluginType::Standard
    }

    fn inputs() -> &'static [&'static str] {
        &["Vm (V)"]
    }

    fn outputs() -> &'static [&'static str] {
        &["Min (V)", "Max (V)", "Burst duration (s)"]
    }

    fn internal_variables() -> &'static [&'static str] {
        &["out_min", "out_max", "min", "max", "temp_min", "temp_max"]
    }

    fn default_vars() -> Vec<(&'static str, Value)> {
        vec![("observation_time", 5.0.into())]
    }

    fn behavior() -> PluginBehavior {
        PluginBehavior {
            supports_start_stop: true,
            supports_restart: true,
            supports_apply: false,
            extendable_inputs: ExtendableInputs::None,
            loads_started: false,
            external_window: false,
            starts_expanded: true,
            start_requires_connected_inputs: Vec::new(),
            start_requires_connected_outputs: Vec::new(),
        }
    }
}

impl PluginRuntime for RthybridBurstAnalysisRust {
    fn set_config_value(&mut self, key: &str, value: &Value) {
        if key == "observation_time" || key == "Observation time (s)" {
            if let Some(v) = value.as_f64() {
                self.observation_time = if v > 0.001 { v } else { 0.001 };
            }
        }
    }

    fn set_input_value(&mut self, key: &str, v: f64) {
        match key {
            "Vm (V)" => self.vm = if v.is_finite() { v } else { 0.0 },
            _ => {}
        }
    }

    fn process_tick(&mut self, _tick: u64, period_seconds: f64) {
        if !period_seconds.is_finite() || period_seconds <= 0.0 {
            return;
        }
        let time_s = self.count * period_seconds;
        let freq = 1.0 / period_seconds;

        if time_s > self.observation_time {
            self.min = self.temp_min;
            self.max = self.temp_max;
            if self.min.is_finite() && self.max.is_finite() {
                self.out_0 = self.min;
                self.out_1 = self.max;
            }
            self.temp_min = 999_999.0;
            self.temp_max = -999_999.0;
            self.count = 0.0;

            if self.burst_counter > 0.0 {
                self.sec_per_burst = (self.burst_dur_sum / self.burst_counter) / freq;
            } else {
                self.sec_per_burst = 0.0;
            }
            self.out_2 = if self.sec_per_burst.is_finite() {
                self.sec_per_burst
            } else {
                0.0
            };

            self.pts_counter = 0.0;
            self.burst_counter = 0.0;
            self.burst_dur_sum = 0.0;
            self.old_burst_time = 0.0;
        }

        if self.vm < self.temp_min {
            self.temp_min = self.vm;
        }
        if self.vm > self.temp_max {
            self.temp_max = self.vm;
        }

        self.range = self.max - self.min;
        self.thresh_down = self.min + self.range * 0.1;
        self.thresh_up = self.min + self.range * 0.9;

        if !self.is_burst && self.vm > self.thresh_up {
            self.is_burst = true;
            self.burst_counter += 1.0;
            self.burst_dur_sum += self.pts_counter - self.old_burst_time;
            self.old_burst_time = self.pts_counter;
        } else if self.is_burst && self.vm < self.thresh_down {
            self.is_burst = false;
        }

        self.pts_counter += 1.0;
        self.count += 1.0;
    }

    fn get_output_value(&self, key: &str) -> f64 {
        match key {
            "Min (V)" => self.out_0,
            "Max (V)" => self.out_1,
            "Burst duration (s)" => self.out_2,
            _ => 0.0,
        }
    }

    fn get_internal_value(&self, key: &str) -> Option<f64> {
        match key {
            "out_min" => Some(self.out_0),
            "out_max" => Some(self.out_1),
            "min" => Some(self.min),
            "max" => Some(self.max),
            "temp_min" => Some(self.temp_min),
            "temp_max" => Some(self.temp_max),
            _ => None,
        }
    }
}

rtsyn_plugin::export_plugin!(RthybridBurstAnalysisRust);
