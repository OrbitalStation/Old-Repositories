# ⚠️ WARNING!!! ⚠️
## This crate is no longer supported and is here only because I cannot fully pull it out of crates.io(yanking doesn't count)


# Cargo-vika

## Special tool to ease UEFI development

### WARNING!!! Only x86_64 and AArch64 support for now!

Cargo-vika is a cargo command that can be obtained by executing
    "cargo install cargo-vika".

Can be used like this: "cargo vika &#91;COMMAND&#93; &#91;ARGS&#93;"

It supports few commands:
#### run qemu:
builds & runs application in <a href="https://www.qemu.org/">QEMU</a>

Args:
<ul>
<li>--ovmf &lt;DIR&gt; <-- specify directory where <a href="https://www.linux-kvm.org/page/OVMF">OVMF</a> files are located(default current dir and then system paths)</li>
<li>--exitp &lt;PORT&gt; <-- specify port that will be used as QEMU exit port(default 0xF4) </li>
<li>--cores &lt;CORES&gt; <-- specify number of cores to emulate(default 4)</li>
<li>--mem &lt;MEM&gt; <-- specify amount of RAM to emulate(default 256M)</li>
<li>--vga &lt;VGA&gt; <-- specify QEMU which VGA to use("-" to use QEMU's default, default std) </li>
<li>--dev &lt;DEVICE&gt; <-- can be specified multiple times; specify QEMU extra device to use</li>
<li>--features &lt;FEATURES&gt; <-- app <a href="https://doc.rust-lang.org/cargo/reference/features.html">features</a> to enable, e.g. to enable features foo, bar & baz, pass --features foo,bar,baz</li>
<li>--debug <-- tell QEMU to wait until <a href="https://www.gnu.org/software/gdb/">GDB</a> will connect(check <a href="https://qemu.readthedocs.io/en/latest/system/gdb.html">this</a> too)</li>
<li>--debug-port &lt;PORT&gt; <-- use together with --debug, specify GDB and QEMU which TCP port to use to connect by</li>
<li>--pass-output <-- everything that will be printed to QEMU using UEFI.stdout() will be printed to console too</li>
</ul>

#### new &lt;NAME&gt;
creates new package with template inside, just like "cargo new"

#### build

builds application

Args:
<ul>
<li>--features &lt;FEATURES&gt; <-- same as run.features</li>
</ul>

P.S. I really don't mind if you send to my email(check Cargo.toml.authors) your own variant of this README,
'cause I'm not a good designer :)
