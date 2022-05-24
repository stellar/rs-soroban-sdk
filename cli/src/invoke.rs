use std::{error::Error, fmt::Debug, fmt::Display, fs};

use clap::Parser;
use stellar_contract_env_host::{
    xdr::{Error as XdrError, ScVal, ScVec},
    Host, Vm,
};

use crate::strval::{self, StrValError};

#[derive(Parser, Debug)]
pub struct Invoke {
    #[clap(long, parse(from_os_str))]
    file: std::path::PathBuf,
    #[clap(long = "fn")]
    function: String,
    #[clap(long = "arg", multiple_occurrences = true)]
    args: Vec<String>,
}

#[derive(Debug)]
pub enum InvokeError {
    Error(Box<dyn Error>),
    StrValError(StrValError),
    XdrError(XdrError),
}

impl Error for InvokeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Error(e) => e.source(),
            Self::StrValError(e) => e.source(),
            Self::XdrError(e) => e.source(),
        }
    }
}

impl Display for InvokeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invoke error: ")?;
        Ok(match self {
            Self::Error(e) => std::fmt::Display::fmt(&e, f)?,
            Self::StrValError(e) => std::fmt::Display::fmt(&e, f)?,
            Self::XdrError(e) => std::fmt::Display::fmt(&e, f)?,
        })
    }
}

impl From<Box<dyn Error>> for InvokeError {
    fn from(e: Box<dyn Error>) -> Self {
        Self::Error(e)
    }
}

impl From<StrValError> for InvokeError {
    fn from(e: StrValError) -> Self {
        Self::StrValError(e)
    }
}

impl From<XdrError> for InvokeError {
    fn from(e: XdrError) -> Self {
        Self::XdrError(e)
    }
}

impl Invoke {
    pub fn run(&self) -> Result<(), InvokeError> {
        let contents = fs::read(&self.file).unwrap();
        let mut h = Host::default();
        let vm = Vm::new(&h, &contents).unwrap();
        let args = self
            .args
            .iter()
            .map(|a| strval::from_string(&h, a))
            .collect::<Result<Vec<ScVal>, StrValError>>()?;
        let res = vm.invoke_function(&mut h, &self.function, &ScVec(args.try_into()?))?;
        println!("{:?}", res);
        Ok(())
    }
}
