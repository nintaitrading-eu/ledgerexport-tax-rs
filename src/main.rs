mod enums;

use docopt::Docopt;
use enums::{rt, ot};
use std::process::Command;
use chrono::prelude::{Utc, Datelike};
use std::path::Path;
#[allow(unused_imports)]
use contracts::{pre, post};
use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

const VERSION: &'static str = "0.1.0";
const USAGE: &'static str = "
Ledgerexport-tax

Usage:
    ledgerexport-tax --file=<file_name> --report=<bal|reg> --quarter=<1|2|3|4> [--year=<year>] [--output=<stdout|txt|pdf>]
    ledgerexport-tax (-h | --help)
    ledgerexport-tax --version

Options:
    --file=<file_name>  Ledger dat filename to use.
    --report=<bal|reg>  Specify bal (balance) or reg (register) report type.
    --output=<stdout|txt|pdf>  Specify the output, stdout is the default.
    --quarter=<quarter>  Export data for the given quarter of the current or given year, should be 1, 2, 3 or 4.
    --year=<year>  Optional year. If no year is given, the current year is used.
    -h --help  Show this screen.
    --version  Show version.
";
const LINEHEIGHT: i64 = 20;
const CURSOR_X: f64 = 10.0;
const CURSOR_Y: f64 = 270.0;
const FONT: &'static str = "/usr/local/share/fonts/inconsolata/Inconsolata-Regular.ttf";
const FONTSIZE: i64 = 14;
const DIMENSION_X: f64 = 210.0;
const DIMENSION_Y: f64 = 297.0;
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

    let output_type = match args.get_str("--output").parse::<ot::OutputType>()
    {
        Ok(o) => o,
        Err(_) =>
        {
            println!("Invalid output type {}.", args.get_str("--output"));
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

    export_data(file, output_type, report_type, quarter, year);
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

#[pre(!afile.is_empty(), "afile should not be empty")]
#[pre(aquarter > 0, "aquarter should be a positive integer")]
#[pre(ayear > 0, "ayear should be a positive integer")]
#[pre((areport_type == rt::ReportType::Balance) || (areport_type == rt::ReportType::Register), "areport_type should be one of the known values")]
fn export_data(
    afile: &str,
    aoutput_type: ot::OutputType,
    areport_type: rt::ReportType,
    aquarter: i32,
    ayear: i32,
)
{
    let report_type = if areport_type == rt::ReportType::Balance
    {
        "bal"
    }
    else
    {
        "reg"
    };
    let output = Command::new("ledger")
        .arg("-f")
        .arg(afile)
        .arg("--strict")
        .arg("-X")
        .arg("-EUR")
        .arg("-H")
        .arg(report_type)
        .arg("-b")
        .arg(get_daterange_from_quarter(aquarter, ayear, true))
        .arg("-e")
        .arg(get_daterange_from_quarter(aquarter, ayear, false))
        .output()
        .expect("Failed to execute process.");
    let mut output_string = String::from_utf8(output.stdout).unwrap();
    generate_pdf(afile, &output_string);
}

fn generate_pdf(afile: &str, aoutput: &str)
{
    let (doc, page1, layer1) =
        PdfDocument::new("report", Mm(DIMENSION_X), Mm(DIMENSION_Y), "layer_1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_external_font(File::open(FONT).unwrap()).unwrap();
    current_layer.begin_text_section();
    current_layer.set_font(&font, FONTSIZE);
    current_layer.set_text_cursor(Mm(CURSOR_X), Mm(CURSOR_Y));
    write_lines_to_pdf(&current_layer, &font, aoutput.split("\n").collect());
    doc.save(&mut BufWriter::new(File::create("test.pdf").unwrap()))
        .unwrap();
}

fn write_lines_to_pdf(alayer: &PdfLayerReference, afont: &IndirectFontRef, alines: Vec<&str>)
{
    alayer.set_line_height(LINEHEIGHT);
    for i in 0..alines.len()
    {
        alayer.write_text(alines[i].clone(), &afont);
        alayer.add_line_break();
    }
}

#[cfg(test)]
mod main_test;
