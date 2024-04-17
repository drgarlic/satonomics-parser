pub trait AnyMap {
    fn path(&self) -> &str;

    fn t_name(&self) -> &str;

    fn exported_path_with_t_name(&self) -> (&str, &str) {
        (self.path(), self.t_name())
    }

    fn reset(&mut self) -> color_eyre::Result<()>;

    fn pre_export(&mut self);
    fn export(&self) -> color_eyre::Result<()>;
    fn post_export(&mut self);
}
