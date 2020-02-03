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

fn get_daterange_from_quarter(aquarter: i32, ayear: i32) -> String
{
    match aquarter {
        1 => format!("-b {}-01-01 -e {}-04-01", ayear, ayear),
        2 => format!("-b {}-04-01 -e {}-07-01", ayear, ayear),
        3 => format!("-b {}-07-01 -e {}-10-01", ayear, ayear),
        4 => format!("-b {}-10-01 -e {}-01-01", ayear, ayear + 1),
        _ => panic!("The function get_daterange_from_quarter is not supposed to have a wrong quarter, something is wrong."),
    }
}

fn export_data(afile: &str, aquarter: i32, ayear: i32)
{
    // TODO: add the following command.
    // Find a good way to use pipes?
    // Or do the sorting in rust?
    //--strict -X -EUR -H {daterange} reg | sort -n
    let daterange = get_daterange_from_quarter(aquarter, ayear);
    println!("Daterange = {}", daterange);
    /*let command: String = format!(
        //"ledger -f ./{} --strict -X -EUR -H {} reg",
        afile, daterange
    );
    println!("Command = {:?}", command);*/
    let output = Command::new("ledger -f ./test.dat")
        .arg("-lh")
        .output()
        .expect("Failed to execute process.");
    println!("output = {}", String::from_utf8(output.stdout).unwrap());
}
