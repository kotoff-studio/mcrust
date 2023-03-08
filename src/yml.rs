use serde_yaml;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = std::fs::File::open("plugin.yml")?;
    let d: String = serde_yaml::from_reader(cfg);
    println!("YML Info: {}", d);
    Ok(());
}
