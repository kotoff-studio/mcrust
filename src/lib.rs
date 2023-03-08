use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

struct WelcomeMessage;
impl WelcomeMessage {

    fn join() {
        let mut e = "%join_event_player%";
        Command::new("?es?_sendMessage_ZN15ObjectMessageSend4initCc")
            .args(["{#ff0000} [Server] {#00ff00} Welcome", e])
            .output();
    }

    fn onEnable() {
        println!("[+] Enabled, starting . . .");
        Self::join();
    }

    fn onDisable() {
        println!("[-] Stopping . . .");
    }
}

fn main() {
    WelcomeMessage::onEnable();
    println!("[+] Started!");
}