extern crate ioctl_sys;
extern crate libc;

const TEMP_FILE_PATH: &str = concat!(env!("CARGO_TARGET_TMPDIR"), "/ioctl_test");

// BSD ioctl tests. Shamelessly stolen from the nix crate
#[cfg(any(target_os = "freebsd", target_os = "macos", target_os = "openbsd"))]
mod bsd_ioctls {
    use std::fs::File;
    use std::{io, mem};
    use std::os::fd::IntoRawFd;
    use std::os::raw::c_int;

    use libc::termios;


    // From:
    //   macOS: /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/sys/ttycom.h
    //   FreeBSD, OpenBSD:
    use ioctl_sys::ioctl;
    use TEMP_FILE_PATH;
    ioctl!(none tiocnxcl with b't', 14);
    ioctl!(read tiocgeta with b't', 19; termios);
    ioctl!(write tiocseta with b't', 20; termios);

    // Common function
    fn test_ioctl(
        expected_err_code: Option<(c_int, &str)>,
        f: fn(temp_file_fd: c_int, stdout_fd: c_int) -> c_int
    ) {
        let temp_file_fd = File::create(TEMP_FILE_PATH)
            .expect("create temp file").into_raw_fd();
        let stdout_fd = 1;
        let result = f(temp_file_fd, stdout_fd);
        match expected_err_code {
            Some((code, name)) => {
                let fail_err_code = io::Error::last_os_error().raw_os_error()
                    .expect("ioctl error code");
                assert_eq!(result, -1, "expected fail code (-1)");
                assert_eq!(fail_err_code, code, "expected error code {} ({})", name, code);
            },
            None => {
                assert_eq!(result, 0, "expected success code (0)");
            }
        }
    }

    #[test]
    fn test_ioctl_none_fail() {
        test_ioctl(Some((25, "ERRNOTTY")), |file_fd, _stdout_fd| {
            unsafe { tiocnxcl( file_fd ) }
        });
    }

    #[test]
    fn test_ioctl_read_fail() {
        test_ioctl(Some((25, "ERRNOTTY")), |file_fd, _stdout_fd| {
            let mut termios = unsafe { mem::zeroed() };
            unsafe { tiocgeta( file_fd, &mut termios ) }
        });
    }

    #[test]
    fn test_ioctl_write_fail() {
        test_ioctl(Some((25, "ERRNOTTY")), |file_fd, _stdout_fd| {
            let mut termios = unsafe { mem::zeroed() };
            unsafe { tiocseta( file_fd, &mut termios ) }
        });
    }

    // Ignored because it need doesn't work on GitHub actions
    #[ignore]
    #[test]
    fn test_ioctl_none_pass() {
        test_ioctl(Some((25, "ERRNOTTY")), |_file_fd, stdout_fd| {
            unsafe { tiocnxcl( stdout_fd ) }
        });
    }

    // Ignored because it need doesn't work on GitHub actions
    #[ignore]
    #[test]
    fn test_ioctl_read_pass() {
        test_ioctl(Some((25, "ERRNOTTY")), |_file_fd, stdout_fd| {
            let mut termios = unsafe { mem::zeroed() };
            unsafe { tiocgeta( stdout_fd, &mut termios ) }
        });
    }

    // Ignored because it need doesn't work on GitHub actions
    // Also ignored because TIOCSETA with zeroed termios will destroy your current terminal session
    // If you decide to test it, just restart your terminal after
    #[ignore]
    #[test]
    fn test_ioctl_write_pass() {
        test_ioctl(Some((25, "ERRNOTTY")), |_file_fd, stdout_fd| {
            let mut termios = unsafe { mem::zeroed() };
            unsafe { tiocseta( stdout_fd, &mut termios ) }
        });
    }
}