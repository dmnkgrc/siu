pub trait Tool {
    fn install(&self, sub_step: usize) -> Result<bool, String>;
    fn print_command(&self);
}
