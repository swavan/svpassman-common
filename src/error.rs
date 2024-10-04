use std::fmt::Display;
use serde::Deserialize;
use thiserror::Error;
use validator::ValidationErrorsKind;
use validator::ValidationErrors;


#[derive(Error, Debug, Deserialize, Clone)]
pub enum PassManConfigEnde{
    #[error("failed to generate key: {0}")]
    Key(String)
}

impl serde::Serialize for PassManConfigEnde {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}


#[derive(Error, Debug, Deserialize, Clone)]
pub enum PassManFormat{
    #[error("Unable to decode data: {0}")]
    Decode(String),

    #[error("Unable to encode data into byte: {0}")]
    Encode(String),

    #[error("invalid key: {0}")]
    Decryption(String),

    #[error("encryption error: {0}")]
    Encryption(String),

}

#[derive(Error, Debug, Deserialize, Clone)]
pub enum PassManFile{

    #[error("Unable to create file {0}")]
    FileCreate(String),

    #[error("Config file does not exist {0}")]
    FileDoesNotExist(String),

    #[error("Unable to decode string fron give file path")]
    FilePathDecode,

    #[error("Unable to open file because of {0}")]
    FileOpen(String),

    #[error("Unable to read file content because of {0}")]
    FileRead(String),

    #[error("Unable to write file content because of {0}")]
    FileWrite(String),

}


impl serde::Serialize for PassManFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Error, Debug, Deserialize, Clone)]
pub enum PassManConfigError {
    #[error("{0}")]
    File(PassManFile),

    #[error("{0}")]
    Folder(String)

}

impl serde::Serialize for PassManConfigError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Error, Debug, Deserialize, Clone)]
pub enum PassManAuthError {
    #[error("User {0} not found")]
    UserNotFound(String),

    #[error("{0} not available")]
    UserAlreadyExists(String),

    #[error("User/Password does not match")]
    UserPasswordDoesNotMatch,

}

#[derive(Error, Debug, Deserialize, Clone)]
pub enum PassManConnectionError {
    #[error("local database connection not found")]
    NoLocalDatabaseAvailable,

    #[error("local api call instance not found")]
    NoAPICallAvailable

}

impl serde::Serialize for PassManConnectionError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Error, Debug, Deserialize, Clone)]
pub enum PassManRequestError {
    #[error("Adding request header error {0}")]
    Header(String),

    #[error("Unable to make api call because of {0}")]
    Reason(String),

    #[error("Server taking long time to respond")]
    Timeout,

    #[error("Passman server is unreachable, Please check your internet connection")]
    Connection,
}

impl serde::Serialize for PassManRequestError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}


#[derive(Error, Debug, Deserialize, Clone)]
pub enum PassManJsonError {

    #[error("Unable to encode {0}")]
    Encode(String),

    #[error("Unable to decode {0}")]
    Decode(String),
}

impl serde::Serialize for PassManJsonError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Error, Debug, Deserialize, Clone)]
pub enum PassManValidation {

    #[error("{0} does not match")]
    NotMatch(String),

    #[error("{0} not found")]
    NotFound(String),

    #[error("We found duplicate {0}")]
    Duplicate(String),

    #[error("{0} data is missing")]
    RequiredField(String),

    #[error("data is invalid {0}")]
    InvalidData(String),

}

impl serde::Serialize for PassManValidation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[derive(Error, Debug, Deserialize, Clone)]
pub enum PassManTokenError {  

    #[error("Token expired")]
    Expired,

    #[error("Unable to validate token")]
    InvalidorMissingClaim,

    #[error("Token being used to early")]
    TokenUsedTooEarly,

    #[error("Token is missing")]
    MissingToken,
}

#[derive(Error, Debug, Deserialize, Clone)]
pub enum PassManDatabaseError {
    #[error("unable to extract data {0}")]
    Fetch(String),

    #[error("unable to save data {0}")]
    Save(String),

    #[error("unable to remove data {0}")]
    Remove(String),

    #[error("unable to commit data {0}")]
    Commit(String),

    #[error("unable to run migration script {0}")]
    Migration(String),

    #[error("unable to run script {0}")]
    Execution(String),

    #[error("Database is not initialized")]
    Uninialize,

    #[error("Connection error {0}")]
    UnableToConnect(String),

    #[error("Unknown error {0}")]
    Unknown(String),
}

impl serde::Serialize for PassManTokenError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(self.to_string().as_ref())
    }
}



#[derive(Error, Debug, Deserialize, Clone)]
pub enum PassManError {

    #[error("{0}")]
    Config(#[from] PassManConfigError),

    #[error("{0}")]
    AuthError(#[from] PassManAuthError),

    #[error("Something went wrong, please try again")]
    Unknown,

    #[error("{0}")]
    Request(#[from] PassManRequestError),

    #[error("{0}")]
    Json(#[from] PassManJsonError),

    #[error("{0}")]
    Format(#[from] PassManFormat),

    #[error("{0}")]
    Token(#[from] PassManTokenError),

    #[error("{0}")]
    Validation(#[from] PassManValidation),

    #[error("{0}")]
    Database(#[from] PassManDatabaseError),

    #[error("{0}")]
    From(String),
}

impl PassManError {
    pub fn from_validation(input: ValidationErrors) -> Self {
        let mut errors = Vec::new();
        for (_, value) in input.errors() {
            match value {
                ValidationErrorsKind::Field(v) => v
                    .into_iter()
                    .for_each(|f| {
                            let msg = f.message.clone().unwrap_or_default();
                            let msg = msg.to_string().replace("`", "");
                            if !msg.is_empty()  {
                                errors.push(msg);
                            }
                        }
                    ),
                _ => {}
            }
        };
        if errors.len() > 0 {
            return PassManError::From(errors[0].clone())
        }
        PassManError::Unknown
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
#[serde(tag = "kind", content = "message")]
#[serde(rename_all = "camelCase")]
pub enum PassManErrorKind {

    ConfigFileCreate(String),
    ConfigFileDoesNotExist(String),
    ConfigFilePathDecode(String),
    ConfigFileOpen(String),
    ConfigFileRead(String),
    ConfigFileWrite(String),
    ConfigFolder(String),
    AuthUserNotFound(String),
    AuthUserAlreadyExists(String),
    AuthUserPasswordDoesNotMatch(String),
    RequestHeader(String),
    RequestReason(String),
    RequestTimeout(String),
    RequestConnection(String),
    
    JsonEncode(String),
    JsonDecode(String),

    FormatDecode(String),
    FormatEncode(String),
    FormatDecryption(String),
    FormatEncryption(String),

    TokenExpired(String),
    TokenInvalidorMissingClaim(String),
    TokenTokenUsedTooEarly(String),
    TokenMissingToken(String),

    ValidationNotMatch(String),
    ValidationNotFound(String),
    ValidationDuplicate(String),
    ValidationRequiredField(String),
    ValidationInvalidData(String),

    DatabaseFetch(String),
    DatabaseSave(String),
    DatabaseRemove(String),
    DatabaseCommit(String),
    DatabaseMigration(String),
    DatabaseExecution(String),
    DatabaseUninialize(String),
    DatabaseUnableToConnect(String),
    DatabaseUnknown(String),

    // File(String),
    From(String),
    Unknown,

}

impl serde::Serialize for PassManError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: serde::ser::Serializer,
    {
      let error_message = self.to_string();
      let error_kind = match self {
        PassManError::Config(v) => match v{
            PassManConfigError::File(v) => match v {
                PassManFile::FileCreate(_) => PassManErrorKind::ConfigFileCreate(error_message),
                PassManFile::FileDoesNotExist(_) => PassManErrorKind::ConfigFileDoesNotExist(error_message),
                PassManFile::FilePathDecode => PassManErrorKind::ConfigFilePathDecode(error_message),
                PassManFile::FileOpen(_) => PassManErrorKind::ConfigFileOpen(error_message),
                PassManFile::FileRead(_) => PassManErrorKind::ConfigFileRead(error_message),
                PassManFile::FileWrite(_) => PassManErrorKind::ConfigFileWrite(error_message),
            },
            PassManConfigError::Folder(_) => PassManErrorKind::ConfigFolder(error_message),
        },
        PassManError::AuthError(v) => match v {
            PassManAuthError::UserNotFound(_) => PassManErrorKind::AuthUserNotFound(error_message),
            PassManAuthError::UserAlreadyExists(_) => PassManErrorKind::AuthUserAlreadyExists(error_message),
            PassManAuthError::UserPasswordDoesNotMatch => PassManErrorKind::AuthUserPasswordDoesNotMatch(error_message),
        },
        PassManError::Unknown => PassManErrorKind::Unknown,
        PassManError::Request(v) => match v {
            PassManRequestError::Header(_) => PassManErrorKind::RequestHeader(error_message),
            PassManRequestError::Reason(_) => PassManErrorKind::RequestReason(error_message),
            PassManRequestError::Timeout => PassManErrorKind::RequestTimeout(error_message),
            PassManRequestError::Connection => PassManErrorKind::RequestConnection(error_message),
        },
        PassManError::Json(v) => match v{
            PassManJsonError::Encode(_) => PassManErrorKind::JsonEncode(error_message),
            PassManJsonError::Decode(_) => PassManErrorKind::JsonDecode(error_message),
        },
        PassManError::Format(v) => match v {
            PassManFormat::Decode(_) => PassManErrorKind::FormatDecode(error_message),
            PassManFormat::Encode(_) => PassManErrorKind::FormatEncode(error_message),
            PassManFormat::Decryption(_) => PassManErrorKind::FormatEncode(error_message),
            PassManFormat::Encryption(_) => PassManErrorKind::FormatEncryption(error_message),
        },
        PassManError::Token(v) => match v {
            PassManTokenError::Expired => PassManErrorKind::TokenExpired(error_message),
            PassManTokenError::InvalidorMissingClaim => PassManErrorKind::TokenInvalidorMissingClaim(error_message),
            PassManTokenError::TokenUsedTooEarly => PassManErrorKind::TokenTokenUsedTooEarly(error_message),
            PassManTokenError::MissingToken => PassManErrorKind::TokenMissingToken(error_message),
        },
        PassManError::Validation(v) => match v {
            PassManValidation::NotMatch(_) => PassManErrorKind::ValidationNotMatch(error_message),
            PassManValidation::NotFound(_) => PassManErrorKind::ValidationNotFound(error_message),
            PassManValidation::Duplicate(_) => PassManErrorKind::ValidationDuplicate(error_message),
            PassManValidation::RequiredField(_) => PassManErrorKind::ValidationRequiredField(error_message),
            PassManValidation::InvalidData(_) => PassManErrorKind::ValidationInvalidData(error_message),
        },
        PassManError::Database(v) => match v {
            PassManDatabaseError::Fetch(_) => PassManErrorKind::DatabaseFetch(error_message),
            PassManDatabaseError::Save(_) => PassManErrorKind::DatabaseSave(error_message),
            PassManDatabaseError::Remove(_) => PassManErrorKind::DatabaseRemove(error_message),
            PassManDatabaseError::Commit(_) => PassManErrorKind::DatabaseCommit(error_message),
            PassManDatabaseError::Migration(_) => PassManErrorKind::DatabaseMigration(error_message),
            PassManDatabaseError::Execution(_) => PassManErrorKind::DatabaseExecution(error_message),
            PassManDatabaseError::Uninialize => PassManErrorKind::DatabaseUninialize(error_message),
            PassManDatabaseError::UnableToConnect(_) => PassManErrorKind::DatabaseUnableToConnect(error_message),
            PassManDatabaseError::Unknown(_) => PassManErrorKind::DatabaseUnknown(error_message),
        },
        PassManError::From(_) => PassManErrorKind::From(error_message),
        };
      error_kind.serialize(serializer)
    }
}

impl Display for PassManErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
        PassManErrorKind::Unknown => write!(f, "Unknown"),
        PassManErrorKind::From(val) => write!(f, "{}", val),
        PassManErrorKind::TokenExpired(val) => write!(f, "{}", val),
        PassManErrorKind::TokenInvalidorMissingClaim(val) => write!(f, "{}", val),
        PassManErrorKind::TokenTokenUsedTooEarly(val) => write!(f, "{}", val),
        PassManErrorKind::TokenMissingToken(val) =>  write!(f, "{}", val),
        PassManErrorKind::AuthUserNotFound(val) =>  write!(f, "{}", val),
        PassManErrorKind::AuthUserAlreadyExists(val) =>  write!(f, "{}", val),
        PassManErrorKind::AuthUserPasswordDoesNotMatch(val) =>  write!(f, "{}", val),
        PassManErrorKind::JsonEncode(val) =>  write!(f, "{}", val),
        PassManErrorKind::JsonDecode(val) =>  write!(f, "{}", val),
        PassManErrorKind::ValidationNotMatch(val) =>  write!(f, "{}", val),
        PassManErrorKind::ValidationNotFound(val) =>  write!(f, "{}", val),
        PassManErrorKind::ValidationDuplicate(val) =>  write!(f, "{}", val),
        PassManErrorKind::ValidationRequiredField(val) =>  write!(f, "{}", val),
        PassManErrorKind::ValidationInvalidData(val) =>  write!(f, "{}", val),
        PassManErrorKind::FormatDecode(val) =>  write!(f, "{}", val),
        PassManErrorKind::FormatEncode(val) =>  write!(f, "{}", val),
        PassManErrorKind::FormatDecryption(val) =>  write!(f, "{}", val),
        PassManErrorKind::FormatEncryption(val) =>  write!(f, "{}", val),
        PassManErrorKind::DatabaseFetch(val) =>  write!(f, "{}", val),
        PassManErrorKind::DatabaseSave(val) =>  write!(f, "{}", val),
        PassManErrorKind::DatabaseRemove(val) =>  write!(f, "{}", val),
        PassManErrorKind::DatabaseCommit(val) =>  write!(f, "{}", val),
        PassManErrorKind::DatabaseMigration(val) =>  write!(f, "{}", val),
        PassManErrorKind::DatabaseExecution(val) =>  write!(f, "{}", val),
        PassManErrorKind::DatabaseUninialize(val) =>  write!(f, "{}", val),
        PassManErrorKind::DatabaseUnableToConnect(val) =>  write!(f, "{}", val),
        PassManErrorKind::DatabaseUnknown(val) =>  write!(f, "{}", val),
        PassManErrorKind::RequestHeader(val) => write!(f, "{}", val),
        PassManErrorKind::RequestReason(val) => write!(f, "{}", val),
        PassManErrorKind::RequestTimeout(val) => write!(f, "{}", val),
        PassManErrorKind::RequestConnection(val) => write!(f, "{}", val),
        PassManErrorKind::ConfigFileCreate(val) => write!(f, "{}", val),
        PassManErrorKind::ConfigFileDoesNotExist(val) => write!(f, "{}", val),
        PassManErrorKind::ConfigFilePathDecode(val) => write!(f, "{}", val),
        PassManErrorKind::ConfigFileOpen(val) => write!(f, "{}", val),
        PassManErrorKind::ConfigFileRead(val) => write!(f, "{}", val),
        PassManErrorKind::ConfigFileWrite(val) => write!(f, "{}", val),
        PassManErrorKind::ConfigFolder(val) => write!(f, "{}", val),
        }
    }
}

impl PassManErrorKind {
    pub fn to(&self) -> PassManError {
        match self {
            PassManErrorKind::ConfigFileCreate(v) => PassManError::Config(PassManConfigError::File(PassManFile::FileCreate(v.to_string()))),
            PassManErrorKind::ConfigFileDoesNotExist(v) => PassManError::Config(PassManConfigError::File(PassManFile::FileDoesNotExist(v.to_string()))),
            PassManErrorKind::ConfigFilePathDecode(_) => PassManError::Config(PassManConfigError::File(PassManFile::FilePathDecode)),
            PassManErrorKind::ConfigFileOpen(v) => PassManError::Config(PassManConfigError::File(PassManFile::FileOpen(v.to_string()))),
            PassManErrorKind::ConfigFileRead(v) => PassManError::Config(PassManConfigError::File(PassManFile::FileRead(v.to_string()))),
            PassManErrorKind::ConfigFileWrite(v) => PassManError::Config(PassManConfigError::File(PassManFile::FileWrite(v.to_string()))),
            PassManErrorKind::ConfigFolder(v) => PassManError::Config(PassManConfigError::Folder(v.to_string())),
            PassManErrorKind::AuthUserNotFound(v) => PassManError::AuthError(PassManAuthError::UserNotFound(v.to_string())),
            PassManErrorKind::AuthUserAlreadyExists(v) => PassManError::AuthError(PassManAuthError::UserAlreadyExists(v.to_string())),
            PassManErrorKind::AuthUserPasswordDoesNotMatch(_) => PassManError::AuthError(PassManAuthError::UserPasswordDoesNotMatch),
            PassManErrorKind::RequestHeader(v) => PassManError::Request(PassManRequestError::Header(v.to_string())),
            PassManErrorKind::RequestReason(v) => PassManError::Request(PassManRequestError::Reason(v.to_string())),
            PassManErrorKind::RequestTimeout(_) => PassManError::Request(PassManRequestError::Timeout),
            PassManErrorKind::RequestConnection(_) => PassManError::Request(PassManRequestError::Connection),
            PassManErrorKind::JsonEncode(v) => PassManError::Json(PassManJsonError::Encode(v.to_string())),
            PassManErrorKind::JsonDecode(v) => PassManError::Json(PassManJsonError::Decode(v.to_string())),
            PassManErrorKind::FormatDecode(v) => PassManError::Format(PassManFormat::Decode(v.to_string())),
            PassManErrorKind::FormatEncode(v) => PassManError::Format(PassManFormat::Encode(v.to_string())),
            PassManErrorKind::FormatDecryption(v) => PassManError::Format(PassManFormat::Decryption(v.to_string())),
            PassManErrorKind::FormatEncryption(v) => PassManError::Format(PassManFormat::Encryption(v.to_string())),

            PassManErrorKind::TokenExpired(_) => PassManError::Token(PassManTokenError::Expired),
            PassManErrorKind::TokenInvalidorMissingClaim(_) => PassManError::Token(PassManTokenError::MissingToken),
            PassManErrorKind::TokenTokenUsedTooEarly(_) => PassManError::Token(PassManTokenError::TokenUsedTooEarly),
            PassManErrorKind::TokenMissingToken(_) => PassManError::Token(PassManTokenError::MissingToken),

            PassManErrorKind::ValidationNotMatch(v) => PassManError::Validation(PassManValidation::NotMatch(v.to_string())),
            PassManErrorKind::ValidationNotFound(v) => PassManError::Validation(PassManValidation::NotFound(v.to_string())),
            PassManErrorKind::ValidationDuplicate(v) => PassManError::Validation(PassManValidation::Duplicate(v.to_string())),
            PassManErrorKind::ValidationRequiredField(v) => PassManError::Validation(PassManValidation::RequiredField(v.to_string())),
            PassManErrorKind::ValidationInvalidData(v) => PassManError::Validation(PassManValidation::InvalidData(v.to_string())),

            PassManErrorKind::DatabaseFetch(v) => PassManError::Database(PassManDatabaseError::Fetch(v.to_string())),
            PassManErrorKind::DatabaseSave(v) => PassManError::Database(PassManDatabaseError::Save(v.to_string())),
            PassManErrorKind::DatabaseRemove(v) => PassManError::Database(PassManDatabaseError::Remove(v.to_string())),
            PassManErrorKind::DatabaseCommit(v) => PassManError::Database(PassManDatabaseError::Commit(v.to_string())),
            PassManErrorKind::DatabaseMigration(v) => PassManError::Database(PassManDatabaseError::Migration(v.to_string())),
            PassManErrorKind::DatabaseExecution(v) => PassManError::Database(PassManDatabaseError::Execution(v.to_string())),
            PassManErrorKind::DatabaseUninialize(_) => PassManError::Database(PassManDatabaseError::Uninialize),
            PassManErrorKind::DatabaseUnableToConnect(v) => PassManError::Database(PassManDatabaseError::UnableToConnect(v.to_string())),
            PassManErrorKind::DatabaseUnknown(v) => PassManError::Database(PassManDatabaseError::Unknown(v.to_string())),

            PassManErrorKind::From(_) => todo!(),
            PassManErrorKind::Unknown => todo!(),
        }
    }
}
