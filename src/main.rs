static mut WINDOW_DIMENSIONS: (u32, u32) = (100, 100);
static mut GRID_SIZE: (u32, u32) = (100, 100);
static mut TICKRATE: u32 = 60;

fn main() -> Result<(), String> {
    env_logger::init();
    handle_args()?;

    Ok(())
}

fn handle_args() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--size" | "-s" => parse_size(&mut args)?,
            "--window" | "-w" => parse_window(&mut args)?,
            "--tick" | "-t" => parse_tick(&mut args)?,
            "--help" | "-h" => { help(); break },
            s => {
                help();
                return Err(format!("Unsupported argument : {}", s));
            },
        }
    }
    Ok(())
}

fn parse_size(args: &mut std::iter::Skip<std::env::Args>) -> Result<(), String> {
    if let (Some(x), Some(y)) = (args.next(), args.next()) {
        if let (Ok(x), Ok(y)) = (x.parse::<u32>(), y.parse::<u32>()) {
            unsafe { GRID_SIZE = (x, y) };
            Ok(())
        } else {
            Err("Could not parse the numbers after size".to_owned())
        }
    } else {
        Err("Expected two integers after size".to_owned())
    }
}

fn parse_window(args: &mut std::iter::Skip<std::env::Args>) -> Result<(), String> {
    if let (Some(x), Some(y)) = (args.next(), args.next()) {
        if let (Ok(x), Ok(y)) = (x.parse::<u32>(), y.parse::<u32>()) {
            unsafe { WINDOW_DIMENSIONS = (x, y) };
            Ok(())
        } else {
            Err("Could not parse the numbers after window".to_owned())
        }
    } else {
        Err("Expected two integers after window".to_owned())
    }
}

fn parse_tick(args: &mut std::iter::Skip<std::env::Args>) -> Result<(), String> {
    if let Some(tick) = args.next() {
        unsafe {
            if let Ok(tick) = tick.parse::<u32>() {
                TICKRATE = tick;
            } else {
                return Err("Could not parse tick".to_owned())
            }
        }
    } else {
        return Err("Expected an integer after tick".to_owned());
    }
    Ok(())
}

fn help() {
    println!("Usage: ./pgol [--size x y] [--window x y] [--tick x]");
}
