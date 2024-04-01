pub trait AnyMap: AnyExportableMap {
    fn import_tmp_data(&self);

    fn path(&self) -> &str;

    fn t_name(&self) -> &str;

    fn reset(&mut self) -> color_eyre::Result<()>;
}

pub trait AnyExportableMap {
    fn export_then_clean(&self) -> color_eyre::Result<()>;
}
