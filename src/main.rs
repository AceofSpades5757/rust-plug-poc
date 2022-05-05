/// Simple Vim plugin written in Rust and implemented with rust-plug
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

use simple_logger::SimpleLogger;

fn main() {
    SimpleLogger::new().init().unwrap();

    // Config
    let ip = "127.0.0.1";
    let port = "8765";

    let listener = TcpListener::bind(format!("{ip}:{port}")).unwrap();

    match listener.accept() {
        Ok((mut stream, addr)) => {
            log::info!("New Client: {:?}", addr);
            log::info!("New Client: {:?}", stream);
            log::info!("Starting Plugin.");
            plugin(&mut stream);
            log::info!("Plugin Run Successfully.");
        }
        Err(err) => log::warn!("Unable to get client: {:?}", err),
    }
}

/// Vim Plugin
///
/// Set a global variable
fn plugin(stream: &mut TcpStream) {
    use vii::channel::ChannelCommand;
    use vii::channel::ExCommand;

    // Vars
    let variable = "rust_plug_plugin_poc";
    let value = "Hello Vim!".to_string();

    // Set Global Variable
    let command: String = format!(r#"let g:{variable} = '{value}'"#);
    let ex = ChannelCommand::Ex(ExCommand { command });
    let channel_command = ex.to_string();

    log::info!("Sending Command: {:?}", channel_command);
    stream.write(channel_command.as_bytes()).unwrap();
    stream.flush().unwrap();
}
