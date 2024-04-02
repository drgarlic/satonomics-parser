pub trait AnyMap {
    fn export_then_clean(&self) -> color_eyre::Result<()>;

    fn import_tmp_data(&self);

    fn path(&self) -> &str;

    fn path_last(&self) -> Option<&String>;

    fn t_name(&self) -> &str;

    fn exported_paths_with_t_name(&self) -> Vec<(&str, &str)> {
        let t_name = self.t_name();

        let tuple = (self.path(), t_name);

        if let Some(path_last) = self.path_last().to_owned() {
            vec![tuple, (path_last, t_name)]
        } else {
            vec![tuple]
        }
    }

    fn reset(&mut self) -> color_eyre::Result<()>;
}
