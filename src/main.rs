#![allow(dead_code)]
#![allow(unused_imports)]
/// Simple Vim plugin written in Rust and implemented with rust-plug
use std::env;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

use simple_logger::SimpleLogger;
use vii::plugin::Plugin;
use vii::plugin::PluginConfig;

fn main() {
    // From environ var
    let port = env::var("VII_PLUGIN_PORT").unwrap();
    let mut plugin = MyPlugin {
        ip: "127.0.0.1".to_string(),
        port,
    };

    plugin.run();
}

/// Vim Plugin
///
/// Set a global variable
struct MyPlugin {
    ip: String,
    port: String,
}

impl Plugin for MyPlugin {
    fn get_config(&self) -> PluginConfig {
        //PluginConfig::new("127.0.0.1".to_string(), "8765".to_string())
        PluginConfig::new(self.ip.clone(), self.port.clone())
    }
    fn plugin(&mut self, stream: &mut TcpStream) -> Result<(), String> {
        use vii::channel::ChannelCommand;
        use vii::channel::ExCommand;

        use chrono;

        // Vars
        let variable = "rust_plug_plugin_poc";
        let dt_string = chrono::offset::Local::now().format("%H-%M-%S");
        let value = format!("Hello Vim! - AT {}", dt_string);

        // Set Global Variable
        let command: String = format!(r#"let g:{variable} = '{value}'"#);
        let ex = ChannelCommand::Ex(ExCommand { command });
        let channel_command = ex.to_string();

        log::info!("Sending Command: {:?}", channel_command);
        stream.write(channel_command.as_bytes()).unwrap();
        stream.flush().unwrap();

        Ok(())
    }
}
