extern crate docopt;
extern crate chrono;

//#[macro_use]
//mod enums;

use docopt::Docopt;
//use enums::quarter;
use std::process::Command;
use chrono::prelude::{Utc, Datelike};
use std::path::Path;

const VERSION: &'static str = "0.1.0";
const USAGE: &'static str = "
Ledgerexport-tax

Usage:
    ledgerexport-tax --file=<file_name> --quarter=<1|2|3|4> [--year=<year>]
    ledgerexport-tax (-h | --help)
    ledgerexport-tax --version

Options:
    --file=<file_name>  Ledger dat filename to use.
    --quarter=<quarter>  Export data for the given quarter of the current or given year, should be 1, 2, 3 or 4.
    --year=<year>  Optional year. If no year is given, the current year is used.
    -h --help  Show this screen.
    --version  Show version.
";
//const CMD_INCOMEVSEXPENSES_INCOME: &'static str = "ledger -f {file} --strict -j reg --real -X EUR -H ^income {period} --collapse --plot-amount-format=\"%(format_date(date, \"%Y-%m-%d\")) %(abs(quantity(scrub(display_amount))))\n";

fn main()
{
    let args = Docopt::new(USAGE)
        .and_then(|dopt| dopt.parse())
        .unwrap_or_else(|e| e.exit());
    println!("--------- BEGIN DEBUG ------------");
    println!("{:?}", args);
    println!("--------- END DEBUG ------------");

    if args.get_bool("--version")
    {
        println!("Ledgerexport-tax v{}", VERSION);
    }
    else
    {
        let file = args.get_str("--file");

        let current_year: i32 = Utc::now().year();
        let year = match args.get_str("--year").parse::<i32>()
        {
            Ok(num) => num,
            Err(_) => current_year,
        };

        if (file.len() > 0) && Path::new(file).exists()
        {
            let quarter = match args.get_str("--quarter").parse::<i32>()
            {
                Ok(num) => num,
                Err(_) => -1,
            };
            if (1..5).contains(&quarter)
            {
                export_data(file, quarter, year);
                std::process::exit(0);
            }
            else
            {
                println!("Invalid quarter: {}", quarter)
            }
        }
        else
        {
            println!("File {} not found.", file);
        }
    }
    std::process::exit(1);
}

fn get_daterange_from_quarter(aquarter: i32, ayear: i32, a_is_first_part: bool) -> String
{
    match aquarter {
        1 =>
            if a_is_first_part
            {
                format!("{}-01-01", ayear)
            }
            else
            {
                format!("{}-04-01", ayear)
            },
        2 =>
            if a_is_first_part
            {
                format!("{}-04-01", ayear)
            }
            else
            {
                format!("{}-07-01", ayear)
            },
        3 =>
            if a_is_first_part
            {
                format!("{}-07-01", ayear)
            }
            else
            {
                format!("{}-10-01", ayear)
            },
                4 =>
                    if a_is_first_part
                    {
                        format!("{}-10-01", ayear)
                    }
                    else
                    {
                        format!("{}-01-01", ayear + 1)
                    },
        _ => panic!("The function get_daterange_from_quarter is not supposed to have a wrong quarter, something is wrong."),
    }
}

fn export_data(afile: &str, aquarter: i32, ayear: i32)
{
    // TODO: add the following command.
    // Find a good way to use pipes?
    // Or do the sorting in rust?
    //--strict -X -EUR -H {daterange} reg | sort -n
    println!(
        "first-part: {}",
        get_daterange_from_quarter(aquarter, ayear, true)
    );
    println!(
        "first-part: {}",
        get_daterange_from_quarter(aquarter, ayear, false)
    );
    let output = Command::new("ledger")
        .arg("-f")
        .arg(afile)
        .arg("--strict")
        .arg("-X")
        .arg("-EUR")
        .arg("-H")
        .arg("reg")
        .arg("-b")
        .arg(get_daterange_from_quarter(aquarter, ayear, true))
        .arg("-e")
        .arg(get_daterange_from_quarter(aquarter, ayear, false))
        .output()
        .expect("Failed to execute process.");
    println!("output = {}", String::from_utf8(output.stdout).unwrap());
}
