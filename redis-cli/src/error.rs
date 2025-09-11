use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct Er {
    msg: &'static str
}

impl Er {
    pub fn new(msg: &'static str) -> Box<Er> {
        Box::new(Er{ msg })
    }
}

impl Display for Er {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg)
    }
}

impl Error for Er {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
