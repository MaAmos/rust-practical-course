use ctrlc;

pub fn ctrl_c_handler(running: std::sync::Arc<std::sync::atomic::AtomicBool>) {
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("Ctrl+C received, shutting down...");
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");
}