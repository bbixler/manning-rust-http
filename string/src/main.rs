fn main() {
    let art: &str = r#"
 █     █░ ▄▄▄        ██████  ███▄ ▄███▓▓█████ ▓█████▄   ▄████ ▓█████ 
▓█░ █ ░█░▒████▄    ▒██    ▒ ▓██▒▀█▀ ██▒▓█   ▀ ▒██▀ ██▌ ██▒ ▀█▒▓█   ▀ 
▒█░ █ ░█ ▒██  ▀█▄  ░ ▓██▄   ▓██    ▓██░▒███   ░██   █▌▒██░▄▄▄░▒███   
░█░ █ ░█ ░██▄▄▄▄██   ▒   ██▒▒██    ▒██ ▒▓█  ▄ ░▓█▄   ▌░▓█  ██▓▒▓█  ▄ 
░░██▒██▓  ▓█   ▓██▒▒██████▒▒▒██▒   ░██▒░▒████▒░▒████▓ ░▒▓███▀▒░▒████▒
░ ▓░▒ ▒   ▒▒   ▓▒█░▒ ▒▓▒ ▒ ░░ ▒░   ░  ░░░ ▒░ ░ ▒▒▓  ▒  ░▒   ▒ ░░ ▒░ ░
  ▒ ░ ░    ▒   ▒▒ ░░ ░▒  ░ ░░  ░      ░ ░ ░  ░ ░ ▒  ▒   ░   ░  ░ ░  ░
  ░   ░    ░   ▒   ░  ░  ░  ░      ░      ░    ░ ░  ░ ░ ░   ░    ░   
    ░          ░  ░      ░         ░      ░  ░   ░          ░    ░  ░
                                               ░                     
  "#;
    println!("{}", art);

    let hello: &str = "Hello WasmEdge!";
    let mut howdy: String = hello.replace("Hello", "Howdy");
    println!("{}", hello);
    println!("{}", howdy);

    howdy.push_str(" -- from Texas");
    println!("{}", howdy);
}
