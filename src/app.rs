use memprocfs::{Vmm, VmmMapPoolEntry};
use std::env;

/// App structure which holds all data related to this instance
pub struct App {}

impl App {
    /// Instantiates the App
    pub fn new() -> App {
        return App {};
    }

    /// Runs the DMA app, finding kernel version and allocated pools
    pub fn run(&self, args: Vec<&str>) {
        // MemProcFS Rust requires full path to vmm.dll/so so use current directory
        let vmm_path: String;
        if let Ok(current_dir) = env::current_dir() {
            let current_dir_str = current_dir.to_str().unwrap();
            if cfg!(windows) {
                vmm_path = format!("{}\\vmm.dll", current_dir_str);
            } else {
                vmm_path = format!("{}/vmm.so", current_dir_str);
            }
        } else {
            println!("App: Unable to get current directory.");
            return;
        }

        // Initialize Vmm on passed parameters, always expect this to be ok, so panic if it's not
        let vmm = Vmm::new(vmm_path.as_str(), &args).unwrap();

        // Find current Windows version (useful to enable/disable certain things!)
        println!("Windows version: {}", vmm.kernel().build());

        // Loop all pools, example is targeting vgk's 'obfuscated regions' which are pools.
        if let Ok(pool_all) = vmm.map_pool(false) {
            let pool_proc_all: Vec<&VmmMapPoolEntry> =
                pool_all.iter().filter(|e| e.cb == 0x200000).collect();
            println!(
                "Number of pool potential allocations: {}.",
                pool_proc_all.len()
            );
            for pool_proc in &*pool_proc_all {
                println!("{pool_proc} ");
            }
        }
    }
}
