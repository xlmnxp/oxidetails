use config_manager::get_public_key;

mod config_manager;
mod encryption;
mod web_server;

fn main() {
    println!("Public Key: {}", get_public_key());
    println!("Server Name: {}", config_manager::get_value("server_name"));
    std::thread::spawn(|| {
        web_server::start();
    });

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
