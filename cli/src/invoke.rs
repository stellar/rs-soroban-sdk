use std::{error::Error, fmt::Debug, fmt::Display, fs};

use clap::Parser;
use stellar_contract_env_host::{
    xdr::{Error as XdrError, ScVal, ScVec},
    Host, Vm,
};

#[derive(Parser, Debug)]
pub struct Invoke {
    #[clap(long, parse(from_os_str))]
    file: std::path::PathBuf,
    #[clap(long = "fn")]
    function: String,
}

#[derive(Debug)]
pub enum InvokeError {
    Error(Box<dyn Error>),
    XdrError(XdrError),
}

impl Error for InvokeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Error(e) => e.source(),
            Self::XdrError(e) => e.source(),
        }
    }
}

impl Display for InvokeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invoke error: ", ());
        Ok(match self {
            Self::Error(e) => std::fmt::Display::fmt(&e, f)?,
            Self::XdrError(e) => std::fmt::Display::fmt(&e, f)?,
        })
    }
}

impl From<Box<dyn Error>> for InvokeError {
    fn from(e: Box<dyn Error>) -> Self {
        Self::Error(e)
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
        let params = Vec::<ScVal>::new();
        let res = vm.invoke_function(&mut h, &self.function, &ScVec(params.try_into()?))?;
        println!("{:?}", res);
        Ok(())
    }
}
