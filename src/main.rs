extern crate docopt;
extern crate chrono;

//#[macro_use]
//mod enums;

use docopt::Docopt;
//use enums::quarter;
use std::process::Command;
use chrono::prelude::{Utc, Datelike};

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
        let year = match args.get_str("--year").parse::<i32>() {
            Ok(num) => num,
            Err(_) => current_year,
        };

        if file.len() > 0
        {
            let quarter = match args.get_str("--quarter").parse::<i32>() {
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
    }
    std::process::exit(1);
}

fn get_daterange_from_quarter(aquarter: i32, ayear: i32)
{
    println!("aquarter = {}, ayear = {}", aquarter, ayear);
}

fn export_data(afile: &str, aquarter: i32, ayear: i32)
{
    println!("TEST - prepare_data: {} for plot {:?} and year {:?}", afile, aquarter, ayear);
    get_daterange_from_quarter(aquarter, ayear);
    /*if aplot_type == plot::PlotType::IncomeVsExpenses
    {
      println!("PlotType enum = {:?}", aplot_type);
      // TODO: period must be a parameter
      // TODO: The below does not work.
      let output = Command::new(format!(CMD_INCOMEVSEXPENSES_INCOME, file=afile, period="--startyear=2014 --endyear=2019"))
          //.arg("Hello world")
          .output()
          .expect("Failed to execute ledger command.");
      println!("After command");
      assert_eq!(b"Hello world\n", output.stdout.as_slice());
      println!("After2 command");
    }*/
}
