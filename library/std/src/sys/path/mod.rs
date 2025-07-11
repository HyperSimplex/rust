cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
        mod windows_prefix;
        pub use windows::*;
    } else if #[cfg(all(target_vendor = "fortanix", target_env = "sgx"))] {
        mod sgx;
        pub use sgx::*;
    } else if #[cfg(target_os = "solid_asp3")] {
        mod unsupported_backslash;
        pub use unsupported_backslash::*;
    } else if #[cfg(target_os = "uefi")] {
        mod uefi;
        pub use uefi::*;
    } else if #[cfg(target_os = "cygwin")] {
        mod cygwin;
        mod windows_prefix;
        pub use cygwin::*;
    } else {
        mod unix;
        pub use unix::*;
    }
}
