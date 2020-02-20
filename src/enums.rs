pub mod rt
{
    use std::str::FromStr;
    use std::fmt::{self, Debug};

    #[derive(Debug, PartialEq)]
    pub enum ReportType
    {
        Balance,
        Register,
    }

    impl FromStr for ReportType
    {
        type Err = ();

        fn from_str(a_str: &str) -> Result<Self, Self::Err>
        {
            match a_str
            {
                "balance" => Ok(ReportType::Balance),
                "bal" => Ok(ReportType::Balance),
                "register" => Ok(ReportType::Register),
                "reg" => Ok(ReportType::Register),
                _ => Err(()),
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
        Pdf
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
}
