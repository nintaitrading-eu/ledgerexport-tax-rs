#![feature(associated_type_defaults)]
mod enums;

use docopt::Docopt;
use enums::{rt, ot, rt::ToLedgerParam};
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
    ledgerexport-tax --ledger_file=<ledger_file> --report_type=<bal|reg> --quarter=<1|2|3|4> [--year=<year>] [--output_type=<stdout|txt|pdf>] [--output_file=<output_file>]
    ledgerexport-tax (-h | --help)
    ledgerexport-tax --version

Options:
    --ledger_file=<ledger_file>  Ledger filename to use.
    --report_type=<bal|reg>  Specify bal (balance) or reg (register) report type.
    --output=<stdout|txt|pdf>  Specify the output, stdout is the default.
    --output_file=<output_file>  Output filename.
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

    let ledger_file = args.get_str("--ledger_file");
    if !(ledger_file.len() > 0) || !Path::new(ledger_file).exists()
    {
        println!("File {} not found.", ledger_file);
        std::process::exit(1);
    };

    let report_type = match args.get_str("--report_type").parse::<rt::ReportType>()
    {
        Ok(r) => r,
        Err(_) =>
        {
            println!("Invalid report type {}.", args.get_str("--report_type"));
            std::process::exit(1);
        }
    };

    let output_type = match args.get_str("--output_type").parse::<ot::OutputType>()
    {
        Ok(o) => o,
        Err(_) =>
        {
            println!("Invalid output type {}.", args.get_str("--output_type"));
            std::process::exit(1);
        }
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

    let mut output_file = args.get_str("--output_file").to_string();
    if !(output_file.len() > 0)
    {
        output_file = output_type.to_string();
    };
    output_file = add_output_suffix(&output_file, &report_type, &output_type, &quarter);

    let current_year: i32 = Utc::now().year();
    let year = match args.get_str("--year").parse::<i32>()
    {
        Ok(num) => num,
        Err(_) => current_year,
    };

    export_data(
        ledger_file,
        output_type,
        &output_file,
        report_type,
        quarter,
        year,
    );
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

#[pre(!aledger_file.is_empty(), "afile should not be empty")]
#[pre(aquarter > 0, "aquarter should be a positive integer")]
#[pre(ayear > 0, "ayear should be a positive integer")]
#[pre((areport_type == rt::ReportType::Balance) || (areport_type == rt::ReportType::Register), "areport_type should be one of the known values")]
fn export_data(
    aledger_file: &str,
    aoutput_type: ot::OutputType,
    aoutput_file: &str,
    areport_type: rt::ReportType,
    aquarter: i32,
    ayear: i32,
)
{
    let report_type = areport_type.to_ledger_param();
    let output = Command::new("ledger")
        .arg("-f")
        .arg(aledger_file)
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
    if aoutput_type == ot::OutputType::Pdf
    {
        generate_pdf(&output_string, aoutput_file);
    }
}

#[pre(!aoutput_file.is_empty(), "Output file should not be empty.")]
#[pre(!areport_type.to_string().is_empty(), "ReportType should not be empty.")]
#[pre(!aoutput_type.to_string().is_empty(), "OutputType should not be empty.")]
#[pre((1..5).contains(aquarter), "Quarter should be valid.")]
#[post(!ret.is_empty(), "Should return a non-empty string.")]
fn add_output_suffix(
    aoutput_file: &str,
    areport_type: &rt::ReportType,
    aoutput_type: &ot::OutputType,
    aquarter: &i32,
) -> String
{
    // TODO: determine extension, based on OutputType

    // TODO: add suffix _v1_YYYYMMDD.ext
    // TODO: remove extension from filename
    format!(
        "{}_{}_v1_{}_Q{}",
        areport_type.to_ledger_param(),
        "YYYYMMDD", /* TODO: current date */
        aoutput_file.to_string(),
        aquarter
    )
}

#[pre(!aoutput_type.to_string().is_empty(), "OutputType should not be empty.")]
#[post(ret == "pdf" || ret == "txt" || ret == "", "Returned extension should be one of txt, pdf or an empty string.")]
fn ext_from_output_type(aoutput_type: ot::OutputType) -> String
{
    match aoutput_type
    {
        ot::OutputType::Pdf => "pdf",
        ot::OutputType::Txt => "txt",
        ot::OutputType::Stdout => "",
    }
    .to_string()
}

fn generate_pdf(aoutput: &str, aoutput_file: &str)
{
    let (doc, page1, layer1) =
        PdfDocument::new("report", Mm(DIMENSION_X), Mm(DIMENSION_Y), "layer_1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_external_font(File::open(FONT).unwrap()).unwrap();
    current_layer.begin_text_section();
    current_layer.set_font(&font, FONTSIZE);
    current_layer.set_text_cursor(Mm(CURSOR_X), Mm(CURSOR_Y));
    write_lines_to_pdf(&current_layer, &font, aoutput.split("\n").collect());
    doc.save(&mut BufWriter::new(File::create(aoutput_file).unwrap()))
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
