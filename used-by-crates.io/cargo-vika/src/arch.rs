cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        pub const fn arch() -> &'static str {
            "x86_64-unknown-uefi"
        }

        pub const fn efi_output_file() -> &'static str {
            "BootX64.efi"
        }

        pub const fn ovmf_code() -> &'static str {
            "OVMF_CODE.fd"
        }

        pub const fn ovmf_vars() -> &'static str {
            "OVMF_VARS.fd"
        }

        pub const fn qemu_binary() -> &'static str {
            "qemu-system-x86_64"
        }
    } else if #[cfg(target_arch = "aarch64")] {
        pub const fn arch() -> &'static str {
            "aarch64-unknown-uefi"
        }

        pub const fn efi_output_file() -> &'static str {
            "BootAA64.efi"
        }

        pub const fn ovmf_code() -> &'static str {
            "QEMU_EFI-pflash.raw"
        }

        pub const fn ovmf_vars() -> &'static str {
            "vars-template-pflash.raw"
        }

        pub const fn qemu_binary() -> &'static str {
            "qemu-system-aarch64"
        }
    } else {
        pub fn arch() -> ! {
            panic!("unknown arch")
        }

        pub fn efi_output_file() -> ! {
            panic!("unknown arch")
        }

        pub fn ovmf_code() -> ! {
            panic!("unknown arch")
        }

        pub fn ovmf_vars() -> ! {
            panic!("unknown arch")
        }

        pub fn qemu_binary() -> ! {
            panic!("unknown arch")
        }
    }
}
