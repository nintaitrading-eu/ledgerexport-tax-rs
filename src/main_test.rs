use super::*;

#[test]
fn test_daterange_from_quarter()
{
    assert_eq!(get_daterange_from_quarter(1, 2019, true), "2019-01-01");
    assert_eq!(get_daterange_from_quarter(1, 2019, false), "2019-04-01");
    assert_eq!(get_daterange_from_quarter(4, 2020, true), "2020-10-01");
    assert_eq!(get_daterange_from_quarter(4, 2020, false), "2021-01-01");
}

#[test]
fn test_add_output_suffix()
{
    assert_eq!(add_output_suffix("myoutputfilename", &ot::OutputType::Pdf), "myoutputfilename");
    // assert_eq!(add_output_suffix("", "default-value"));
}

#[test]
fn test_ext_from_output_type()
{
    assert_eq!(ext_from_output_type(ot::OutputType::Pdf), "pdf");
    assert_eq!(ext_from_output_type(ot::OutputType::Txt), "txt");
    assert_eq!(ext_from_output_type(ot::OutputType::Stdout), "");
}
