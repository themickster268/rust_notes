pub fn run(){
    /* let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    } */

    let config_max: Option<u8> = Some(3u8);

    // if let - an alternative to match to match one particular case and ignore the rest
    if let Some(max) = config_max { 
        println!("The maximum is configured to be {}", max);
    } else {
        println!("No maximum configured");
    }
}