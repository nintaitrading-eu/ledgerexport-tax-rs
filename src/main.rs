mod enums;

use docopt::Docopt;
use enums::rt;
use std::process::Command;
use chrono::prelude::{Utc, Datelike};
use std::path::Path;
#[allow(unused_imports)]
use contracts::{pre, post};

const VERSION: &'static str = "0.1.0";
const USAGE: &'static str = "
Ledgerexport-tax

Usage:
    ledgerexport-tax --file=<file_name> --report=<bal|reg> --quarter=<1|2|3|4> [--year=<year>]
    ledgerexport-tax (-h | --help)
    ledgerexport-tax --version

Options:
    --file=<file_name>  Ledger dat filename to use.
    --report=<bal|reg>  Specify bal (balance) or reg (register) report type.
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
        std::process::exit(0);
    }

    let file = args.get_str("--file");
    if !(file.len() > 0) || !Path::new(file).exists()
    {
        println!("File {} not found.", file);
        std::process::exit(1);
    };

    let report_type = match args.get_str("--report").parse::<rt::ReportType>()
    {
        Ok(r) => r,
        Err(_) =>
        {
            println!("Invalid report type {}.", args.get_str("--report"));
            std::process::exit(1);
        }
    };

    let current_year: i32 = Utc::now().year();
    let year = match args.get_str("--year").parse::<i32>()
    {
        Ok(num) => num,
        Err(_) => current_year,
    };

    let quarter = match args.get_str("--quarter").parse::<i32>()
    {
        Ok(num) => num,
        Err(_) =>
        {
            println!("Invalid quarter {}.", args.get_str("--quarter"));
            std::process::exit(1);
        }
    };
    if !(1..5).contains(&quarter)
    {
        println!("Invalid quarter: {}", quarter);
        std::process::exit(1);
    }

    export_data(file, report_type, quarter, year);
    std::process::exit(0);
}

#[pre(aquarter > 0, "aquarter should be a positive integer")]
#[pre(ayear > 0, "ayear should be a positive integer")]
#[post(!ret.is_empty(), "get_daterange_from_quarter should return a non-empty string")]
fn get_daterange_from_quarter(aquarter: i32, ayear: i32, a_is_first_part: bool) -> String
{
    let arr_month_begin = ["01", "04", "07", "10"];
    let arr_month_end = ["04", "07", "10", "01"];
    let mut year = ayear;
    if (aquarter == 4) && !a_is_first_part
    {
        year = ayear + 1;
    }
    if a_is_first_part
    {
        format!("{}-{}-01", year, arr_month_begin[(aquarter - 1) as usize])
    }
    else
    {
        format!("{}-{}-01", year, arr_month_end[(aquarter - 1) as usize])
    }
}

fn export_data(afile: &str, areport_type: rt::ReportType, aquarter: i32, ayear: i32)
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
        .arg(areport_type.to_string())
        .arg("-b")
        .arg(get_daterange_from_quarter(aquarter, ayear, true))
        .arg("-e")
        .arg(get_daterange_from_quarter(aquarter, ayear, false))
        .output()
        .expect("Failed to execute process.");
    println!("output = {}", String::from_utf8(output.stdout).unwrap());
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_daterange_from_quarter()
    {
        assert_eq!(get_daterange_from_quarter(1, 2019, true), "2019-01-01");
        assert_eq!(get_daterange_from_quarter(1, 2019, false), "2019-04-01");
        assert_eq!(get_daterange_from_quarter(4, 2020, true), "2020-10-01");
        assert_eq!(get_daterange_from_quarter(4, 2020, false), "2021-01-01");
    }
}
