use fmod_sys::*;

#[derive(Clone, PartialEq, Eq)]
pub enum Error {
    Fmod(FMOD_RESULT), // FIXME make FMOD_RESULT be a NonZero
    NulError(std::ffi::NulError),
    EnumFromPrivitive { name: &'static str, primitive: i64 },
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("Error");
        match self {
            Self::Fmod(code) => debug_struct
                .field("code", code)
                .field("message", &error_code_to_str(*code))
                .finish(),
            Self::NulError(e) => debug_struct.field("nul error", e).finish(),
            Self::EnumFromPrivitive { name, primitive } => debug_struct
                .field("name", name)
                .field("primitive", primitive)
                .finish(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fmod(code) => f.write_str(error_code_to_str(*code)),
            Self::NulError(error) => error.fmt(f),
            Self::EnumFromPrivitive { name, primitive } => f.write_fmt(format_args!(
                "No discriminant in enum `{name}` matches the value `{primitive:?}"
            )),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

impl From<FMOD_RESULT> for Error {
    fn from(value: FMOD_RESULT) -> Self {
        Self::Fmod(value)
    }
}

impl From<std::ffi::NulError> for Error {
    fn from(value: std::ffi::NulError) -> Self {
        Self::NulError(value)
    }
}

impl<T> From<num_enum::TryFromPrimitiveError<T>> for Error
where
    T: num_enum::TryFromPrimitive,
    T::Primitive: Into<i64>,
{
    fn from(value: num_enum::TryFromPrimitiveError<T>) -> Self {
        Self::EnumFromPrivitive {
            name: T::NAME,
            primitive: value.number.into(),
        }
    }
}

impl PartialEq<FMOD_RESULT> for Error {
    fn eq(&self, other: &FMOD_RESULT) -> bool {
        match self {
            Self::Fmod(code) => code == other,
            _ => false,
        }
    }
}

impl From<Error> for FMOD_RESULT {
    fn from(val: Error) -> Self {
        match val {
            Error::Fmod(code) => code,
            Error::NulError(_) | Error::EnumFromPrivitive { .. } => {
                FMOD_RESULT::FMOD_ERR_INVALID_PARAM
            }
        }
    }
}

pub(crate) trait FmodResultExt {
    fn to_result(self) -> Result<()>;

    fn to_error(self) -> Option<Error>;

    fn from_result<T>(result: Result<T>) -> Self;
}

impl FmodResultExt for FMOD_RESULT {
    fn to_result(self) -> Result<()> {
        if matches!(self, FMOD_RESULT::FMOD_OK) {
            Ok(())
        } else {
            Err(Error::Fmod(self))
        }
    }

    fn to_error(self) -> Option<Error> {
        self.to_result().err()
    }

    fn from_result<T>(result: Result<T>) -> Self {
        match result {
            Ok(_) => FMOD_RESULT::FMOD_OK,
            Err(e) => e.into(),
        }
    }
}
