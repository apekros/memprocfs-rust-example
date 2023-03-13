mod app;
use app::App;

fn main() {
    // Instantiate a new DMA app and run using the FPGA connector
    App::new().run(["", "-device", "fpga"].to_vec());
}
