#[derive(Debug)]
pub enum Error{
	Parse(http::uri::InvalidUri),
	Fetch(hyper::Error),
	Stringify(std::string::FromUtf8Error),
	Json(serde_json::Error),
	File(std::io::Error),
	TomlDecode(toml::de::Error),
	TomlEncode(toml::ser::Error),
	Option(()),
}

impl From<http::uri::InvalidUri> for Error {
    fn from(err: http::uri::InvalidUri) -> Error {
        Error::Parse(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Fetch(err)
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Error {
        Error::Stringify(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::File(err)
    }
}

impl From<()> for Error {
    fn from(err: ()) -> Error {
        Error::Option(())
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Error {
        Error::TomlDecode(err)
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Error {
        Error::TomlEncode(err)
    }
}
