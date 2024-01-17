pub trait DatabaseGroup {
    fn drain_export(&mut self) -> color_eyre::Result<()>;

    fn folder<'a>() -> &'a str;
}
