pub mod rt
{
    use std::str::FromStr;
    use std::string::ParseError;
    use std::fmt::{self, Debug};

    pub trait ToLedgerParam
    {
        type Err = ParseError;
        fn to_ledger_param(&self) -> String;
    }

    #[derive(Debug, PartialEq)]
    pub enum ReportType
    {
        Balance,
        Register,
    }

    impl FromStr for ReportType
    {
        type Err = ();

        fn from_str(astr: &str) -> Result<Self, Self::Err>
        {
            match astr
            {
                "balance" => Ok(ReportType::Balance),
                "bal" => Ok(ReportType::Balance),
                "register" => Ok(ReportType::Register),
                "reg" => Ok(ReportType::Register),
                _ => Err(()),
            }
        }
    }

    /* The strings "bal" and "reg" are used in the ledger cli application
     * as a cli parameter. We use this trait to convert the enum to
     * these exact strings.
     */
    impl ToLedgerParam for ReportType
    {
        fn to_ledger_param(&self) -> String
        {
            match self
            {
                ReportType::Balance => "bal".to_string(),
                ReportType::Register => "reg".to_string(),
            }
        }
    }

    impl fmt::Display for ReportType
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
        {
            fmt::Debug::fmt(self, f)
        }
    }
}

pub mod ot
{
    use std::str::FromStr;
    use std::fmt::{self, Debug};

    #[derive(Debug, PartialEq)]
    pub enum OutputType
    {
        Stdout,
        Txt,
        Pdf,
    }

    impl FromStr for OutputType
    {
        type Err = ();

        fn from_str(a_str: &str) -> Result<Self, Self::Err>
        {
            match a_str
            {
                "stdout" => Ok(OutputType::Stdout),
                "txt" => Ok(OutputType::Txt),
                "pdf" => Ok(OutputType::Pdf),
                _ => Err(()),
            }
        }
    }

    impl fmt::Display for OutputType
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
        {
            fmt::Debug::fmt(self, f)
        }
    }

    /* The output-types map to a file extension.
     * Pdf = .pdf
     * Txt = .txt
     * All others = empty string
     */
    impl ToFileExtension for OutputType
    {
        fn to_file_extension(&self) -> String
        {
            match self
            {
                OutputType::Pdf => ".pdf".to_string(),
                OutputType::Txt => ".txt".to_string(),
                _ => "".to_string(),
            }
        }
    }
}
