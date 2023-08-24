#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub fn create_cliprdr_context(
    enable_files: bool,
    enable_others: bool,
    response_wait_timeout_secs: u32,
) -> crate::ResultType<Box<dyn crate::CliprdrServiceContext>> {
    windows::create_cliprdr_context(enable_files, enable_others, response_wait_timeout_secs)
}