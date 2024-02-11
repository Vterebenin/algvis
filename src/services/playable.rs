pub trait Playable<Config, StepType, Output> {
    fn reset(&mut self, config: &Config);
    fn play(&mut self, config: &Config);
    fn stop(&mut self);
    fn tick(&mut self);
    fn set_step(&mut self, step: u32);
    fn get_active_step_string(&self) -> String;
    fn tick_time(&self) -> u32;
    fn get_output_by_step(&mut self, step: u32) -> Output;
    fn get_steps_len_string(&self) -> String;
    fn get_active_step_item(&self) -> StepType;
    fn calculate_time(&mut self, sort_config: &Config);
}

