use std::io;
use std::io::Write;
use std::process;

fn main() {
    loop {
        println!("====================");
        println!("=       MENU       =");
        println!("====================");
        println!("1. Convert °C to °F");
        println!("2. Convert °F to °C");
        println!("3. Quit");
        println!();

        // Get menu choice
        let convert_to_c = loop {
            print!("Menu selection: ");
            io::stdout().flush().expect("Failed to write output");

            let mut menu_choice = String::new();

            io::stdin()
                .read_line(&mut menu_choice)
                .expect("Failed to read line");

            match menu_choice.trim().parse::<u8>() {
                Ok(num) => {
                    if num == 1 {
                        break false;
                    } else if num == 2 {
                        break true;
                    } else if num == 3 {
                        process::exit(0);
                    } else {
                        continue;
                    }
                }
                Err(_) => continue,
            };
        };

        println!();
        // Get temperature
        loop {
            println!(
                "Enter temperature in {}:",
                if convert_to_c { "°F" } else { "°C" }
            );

            let mut temp = String::new();

            io::stdin()
                .read_line(&mut temp)
                .expect("Failed to read line");

            let temp: f64 = match temp.trim().parse::<f64>() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let res = if convert_to_c { f_to_c(temp) } else { c_to_f(temp) };

            println!("{:.2}{}", res, if convert_to_c { "°C" } else { "°F" });
            println!();
            println!();
            break;
        }
    }
}

fn c_to_f(original_temp: f64) -> f64 {
    original_temp * 1.8 + 32.0
}

fn f_to_c(original_temp: f64) -> f64 {
    (original_temp - 32.0) / 1.8
}
