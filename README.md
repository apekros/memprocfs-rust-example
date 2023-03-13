# MemProcFS Rust Example

## Building
```bash
git clone https://github.com/apekros/memprocfs-rust-example.git
cd memprocfs-rust-example
cargo build --release
```

## Run
Connect your PCILEECH / alternative device and download your platforms required Vmm and Leechcore files from [here](https://github.com/ufrisk/MemProcFS/releases/tag/v5.4).

Extract them into the working directory of the project and then use cargo to run the project:
```bash
cargo run
```