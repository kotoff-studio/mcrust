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

    fn registerCommand(command: &str, perm: &str) {
        Command::new("?ev?_commandSetExecutor_ZN12CommandExecutor4registerCv")
            .args([command, perm])
            .output();
    }

    fn testCommand() {
        let mut for_show = "%cmd_executor%";
        Self::registerCommand("/testcmd", "minecraft.tp"); // first of all - registering our command
        Command::new("?es?_sendMessage_ZN15ObjectMessageSend4initCc")
        .args(["{#00ff00} Hello", for_show])
        .output();
    }

    fn onEnable() {
        println!("[+] Enabled, starting . . .");
        Self::join();
        Self::testCommand();
    }

    fn onDisable() {
        println!("[-] Stopping . . .");
    }
}

fn main() {
    WelcomeMessage::onEnable();
    println!("[+] Started!");
}
