use std::{
    path::{Path, PathBuf},
    process::Command,
    io,
    fs,
};
use clap::ArgMatches;

fn ovmf_files(dir: &str) -> Option <(PathBuf, PathBuf)> {
    let dir = Path::new(dir);
    let code = dir.join(crate::arch::ovmf_code());
    let vars = dir.join(crate::arch::ovmf_vars());
    if code.exists() && vars.exists() {
        Some((code, vars))
    } else {
        None
    }
}

pub fn qemu(matches: &ArgMatches) {
    let metadata = crate::metadata::metadata();

    let target_dir = metadata.target_directory.as_std_path();
    let arch = crate::arch::arch();
    let profile = if matches.values_of("pass").unwrap_or_default().into_iter().find(|x| x == &"--release").is_some() {
        "release"
    } else {
        "debug"
    };
    let package = &metadata.packages.last().unwrap().name;
    let build_dir = target_dir.join(arch).join(profile);

    let built_file = build_dir.join(package).with_extension("efi");
    let esp_dir = build_dir.join("esp");
    let boot_dir = esp_dir.join("EFI").join("Boot");

    match fs::create_dir_all(&boot_dir) {
        Ok(_) => (),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => (),
        Err(e) => panic!("failed to create dir: {}", e)
    }

    let output_file = boot_dir.join(crate::arch::efi_output_file());
    fs::copy(&built_file, &output_file).unwrap();

    let (ovmf_code, ovmf_vars) = match matches.value_of("ovmf") {
        // if path is specified in the args, use it
        Some(dir) => ovmf_files(dir).expect("OVMF files not found in specified ovmf dir"),
        // check whether current package contains the files
        None => match ovmf_files(metadata.workspace_root.as_str()) {
            Some(files) => files,
            None => {
                #[cfg(target_os = "linux")]
                let possible_paths = ["/usr/share/OVMF", "/usr/share/ovmf/x64"];

                #[cfg(not(target_os = "linux"))]
                let possible_paths = [];

                'outer: loop {
                    for path in possible_paths {
                        match ovmf_files(path) {
                            Some(files) => break 'outer files,
                            None => ()
                        }
                    }

                    panic!("OVMF files not found anywhere")
                }
            }
        }
    };

    // the OVMF implementation for AArch64 won't boot unless
    // the vars file is writable
    let ovmf_vars_readonly = if cfg!(target_arch = "aarch64") {
        "off"
    } else {
        "on"
    };

    let qemu_ovmf_code = format!("if=pflash,format=raw,file={},readonly=on", ovmf_code.to_str().unwrap());
    let qemu_ovmf_vars = format!("if=pflash,format=raw,file={},readonly={}", ovmf_vars.to_str().unwrap(), ovmf_vars_readonly);
    let qemu_fat_esp = format!("format=raw,file=fat:rw:{}", esp_dir.to_str().unwrap());
    let qemu_vga = match matches.value_of("vga") {
        Some(vga) => vga,
        None => "std"
    };
    let mut qemu_flags = vec![
        // disable defaulg devices
        // QEMU by default enables tons of devices which slow down boot
        "-nodefaults",

        // OVMF
        "-drive", qemu_ovmf_code.as_str(),
        "-drive", qemu_ovmf_vars.as_str(),

        // mount a local directory as a FAT partition
        "-drive", qemu_fat_esp.as_str(),

        // connect the serial port to the host
        "-serial", "stdio",
    ];

    qemu_flags.append(&mut if qemu_vga == "-" { vec![] } else { vec!["-vga", qemu_vga] });
    for dev in matches.values_of("device").unwrap_or_default() {
        qemu_flags.push("-device");
        qemu_flags.push(dev)
    }

    cfg_if::cfg_if! {
        if #[cfg(target_arch = "x86_64")] {
            let exit_port = format!("isa-debug-exit,iobase={},iosize=4", matches.value_of("exit_port").unwrap_or("0xF4"));
            qemu_flags.append(&mut vec![
                // use a modern machine
                "-machine", "q35",

                // cores
                "-smp", matches.value_of("cores").unwrap_or("4"),

                // allocate some memory
                "-m", matches.value_of("mem").unwrap_or("256M"),
                "--enable-kvm",
                "-device", exit_port.as_str()
            ])
        } else if #[cfg(target_arch = "aarch64")] {
            qemu_flags.append(&mut vec![
                // generic ARM environment
                "-machine", "virt",

                // A72 is a very generic 64-bit ARM CPU in the wild
                "-cpu", "cortex-a72"
            ])
        }
    }

    let port = format!("tcp::{}", matches.value_of("debug-port").unwrap_or("1234"));
    if matches.is_present("debug") {
        qemu_flags.append(&mut vec!["-gdb", port.as_str(), "-S"]);
    }

    crate::pretty_output::command("     Running ", built_file.to_str().unwrap());

    let output = Command::new(crate::arch::qemu_binary())
        .args(&qemu_flags)
        .output()
        .unwrap();

    if !output.status.success() {
        crate::pretty_output::error(format!("qemu: {}", core::str::from_utf8(&output.stderr).unwrap()).as_str())
    }
    let warning_pattern = format!("{}: warning:", crate::arch::qemu_binary());
    for line in core::str::from_utf8(&output.stderr).unwrap().lines() {
        if line.starts_with(&warning_pattern) {
            crate::pretty_output::warning(&line[warning_pattern.len()..])
        }
    }

    if matches.is_present("pass-output") {
        println!("{}", core::str::from_utf8(&output.stdout).unwrap())
    }
}
