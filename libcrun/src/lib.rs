/* void libcrun_warning (const char *msg, ...); */

/* void libcrun_error (int errno_, const char *msg, ...); */

/* int libcrun_make_error (libcrun_error_t *err, int status, const char *msg, ...); */

/* void libcrun_error_write_warning_and_release (FILE *out, libcrun_error_t **err); */

/* void libcrun_fail_with_error (int errno_, const char *msg, ...) __attribute__ ((noreturn)); */

/* int libcrun_set_log_format (const char *format, libcrun_error_t *err); */

/* int libcrun_init_logging (crun_output_handler *output_handler, void **output_handler_arg, const char *id,
const char *log, libcrun_error_t *err); */

/* int libcrun_error_release (libcrun_error_t *err); */

/* void libcrun_set_verbosity (int verbosity); */

/* int libcrun_get_verbosity (); */

use libcrun_sys::libcrun_error;
use std::ffi::CString;
use std::os::raw::c_char;

fn error(err_no: i32, msg: &str) {
    let c_str = CString::new(msg).unwrap();
    let c_world: *const c_char = c_str.as_ptr() as *const c_char;
    unsafe {
        libcrun_error(err_no, c_world);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        error(1, "foo");
    }
}
