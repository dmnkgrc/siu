pub trait Tool {
    fn install(&self) -> Result<(), String>;
    fn print_command(&self);
}
