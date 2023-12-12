struct Params {
    size: (u32, u32),
    window: (u32, u32),
    tick: u32
}

fn main() -> Result<(), String> {
    env_logger::init();

    let mut params = Params { size: (100, 100), window: (100, 100), tick: 60 };
    handle_args(&mut params)?;

    Ok(())
}

fn handle_args(params: &mut Params) -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--size" | "-s" => params.size = parse_size(&mut args)?,
            "--window" | "-w" => params.window = parse_window(&mut args)?,
            "--tick" | "-t" => params.tick = parse_tick(&mut args)?,
            "--help" | "-h" => { help(); break },
            s => {
                help();
                return Err(format!("Unsupported argument : {}", s));
            },
        }
    }
    Ok(())
}

fn parse_size(args: &mut std::iter::Skip<std::env::Args>) -> Result<(u32, u32), String> {
    if let (Some(x), Some(y)) = (args.next(), args.next()) {
        if let (Ok(x), Ok(y)) = (x.parse::<u32>(), y.parse::<u32>()) {
            Ok((x, y))
        } else {
            Err("Could not parse the numbers after size".to_owned())
        }
    } else {
        Err("Expected two integers after size".to_owned())
    }
}

fn parse_window(args: &mut std::iter::Skip<std::env::Args>) -> Result<(u32, u32), String> {
    if let (Some(x), Some(y)) = (args.next(), args.next()) {
        if let (Ok(x), Ok(y)) = (x.parse::<u32>(), y.parse::<u32>()) {
            Ok((x, y))
        } else {
            Err("Could not parse the numbers after window".to_owned())
        }
    } else {
        Err("Expected two integers after window".to_owned())
    }
}

fn parse_tick(args: &mut std::iter::Skip<std::env::Args>) -> Result<u32, String> {
    if let Some(tick) = args.next() {
        if let Ok(tick) = tick.parse::<u32>() {
            Ok(tick)
        } else {
            return Err("Could not parse tick".to_owned())
        }
    } else {
        return Err("Expected an integer after tick".to_owned())
    }
}

fn help() {
    println!("Usage: ./pgol [--size x y | --window x y | --tick x]");
}
