use rtsyn_plugin::prelude::*;
use serde_json::Value;
use std::mem::MaybeUninit;

#[repr(C)]
struct RthybridBurstAnalysisCState {
    observation_time: f64,
    vm: f64,
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
    is_burst: f64,
    burst_dur_sum: f64,
    old_burst_time: f64,
    sec_per_burst: f64,
    out_min: f64,
    out_max: f64,
    out_burst_duration: f64,
}

unsafe extern "C" {
    fn rthybrid_burst_analysis_c_init(state: *mut RthybridBurstAnalysisCState);
    fn rthybrid_burst_analysis_c_set_config(
        state: *mut RthybridBurstAnalysisCState,
        key: *const u8,
        len: usize,
        value: f64,
    );
    fn rthybrid_burst_analysis_c_set_input(
        state: *mut RthybridBurstAnalysisCState,
        key: *const u8,
        len: usize,
        value: f64,
    );
    fn rthybrid_burst_analysis_c_process(state: *mut RthybridBurstAnalysisCState, period_seconds: f64);
    fn rthybrid_burst_analysis_c_get_output(
        state: *const RthybridBurstAnalysisCState,
        key: *const u8,
        len: usize,
    ) -> f64;
}

struct RthybridBurstAnalysisC {
    state: RthybridBurstAnalysisCState,
}

impl Default for RthybridBurstAnalysisC {
    fn default() -> Self {
        let mut state = MaybeUninit::<RthybridBurstAnalysisCState>::uninit();
        unsafe {
            rthybrid_burst_analysis_c_init(state.as_mut_ptr());
            Self {
                state: state.assume_init(),
            }
        }
    }
}

impl PluginDescriptor for RthybridBurstAnalysisC {
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

impl PluginRuntime for RthybridBurstAnalysisC {
    fn set_config_value(&mut self, key: &str, value: &Value) {
        if let Some(v) = value.as_f64() {
            unsafe { rthybrid_burst_analysis_c_set_config(&mut self.state, key.as_ptr(), key.len(), v) };
        }
    }

    fn set_input_value(&mut self, key: &str, value: f64) {
        unsafe { rthybrid_burst_analysis_c_set_input(&mut self.state, key.as_ptr(), key.len(), value) };
    }

    fn process_tick(&mut self, _tick: u64, period_seconds: f64) {
        unsafe { rthybrid_burst_analysis_c_process(&mut self.state, period_seconds) };
    }

    fn get_output_value(&self, key: &str) -> f64 {
        unsafe { rthybrid_burst_analysis_c_get_output(&self.state, key.as_ptr(), key.len()) }
    }

    fn get_internal_value(&self, key: &str) -> Option<f64> {
        match key {
            "out_min" => Some(self.state.out_min),
            "out_max" => Some(self.state.out_max),
            "min" => Some(self.state.min),
            "max" => Some(self.state.max),
            "temp_min" => Some(self.state.temp_min),
            "temp_max" => Some(self.state.temp_max),
            _ => None,
        }
    }
}

rtsyn_plugin::export_plugin!(RthybridBurstAnalysisC);
