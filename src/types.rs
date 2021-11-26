use crate::error::{Result, Error};

#[allow(dead_code)]
pub enum ContentType{
    ImageJpeg,
    ImagePng,
    TextPlain,
    TextHTML,
    ApplicationMSWord
}

impl std::fmt::Display for ContentType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error>{
        match self{
            ContentType::ImageJpeg => write!(f, "{}", "image/jpeg"),
            ContentType::ImagePng => write!(f, "{}", "image/png"),
            ContentType::TextPlain => write!(f, "{}", "text/plain"), 
            ContentType::TextHTML => write!(f, "{}", "text/html"),
            ContentType::ApplicationMSWord => write!(f, "{}", "application/msword"),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum StatusCodes{
    NoResponse,
    ServiceReady,       //220
    ServiceClosed,      //221
    AuthenticationSuceeded, //235
    Ok,                 //250
    ServerChallenge,    //334
    StartingMailInput,  //354
    TimedOutWaitingForClient, //451    
    CommandUnrecognised, //500    
    SyntaxError,        //501
    BadCommandSequence, // 503
    AuthenticationRequired, //530
    AuthenticationFailed, //535
    Unknown, // For status codes I have no implemented 
}

impl From<StatusCodes> for String{
    fn from(s: StatusCodes) -> Self {
        let v = match s {
            StatusCodes::NoResponse => "000",
            StatusCodes::ServiceReady => "220",
            StatusCodes::ServiceClosed => "221",
            StatusCodes::AuthenticationSuceeded => "235",
            StatusCodes::Ok => "250",
            StatusCodes::ServerChallenge => "334",
            StatusCodes::StartingMailInput => "354",
            StatusCodes::TimedOutWaitingForClient => "451",
            StatusCodes::CommandUnrecognised => "500",
            StatusCodes::SyntaxError => "501",
            StatusCodes::BadCommandSequence => "503",
            StatusCodes::AuthenticationRequired => "530",
            StatusCodes::AuthenticationFailed => "535",
            StatusCodes::Unknown => "999",
        };
        v.to_owned()
    }
}

impl StatusCodes {

    pub(crate) fn lookup(bytes: &Vec<u8>) -> Result<Self> {
        let msg_len = bytes.iter().position(| &i | i == '\x00' as u8).unwrap_or(0);
        let status = &bytes[0..3];
        let full_msg = std::str::from_utf8(&bytes[..msg_len as usize]).unwrap_or("Could not display message from server");

        // For debugging
        // println!("VERBOSE: {:?}", full_msg);
        
        match status{
            b"\x00\x00\x00" => Err(Error::SmtpStatus(Self::NoResponse,full_msg.to_string())),
            b"220" => Ok(Self::ServiceReady),
            b"221" => Ok(Self::ServiceClosed),
            b"235" => Ok(Self::AuthenticationSuceeded),
            b"250" => Ok(Self::Ok),
            b"334" => Ok(Self::ServerChallenge),
            b"354" => Ok(Self::StartingMailInput),
            b"451" => Err(Error::SmtpStatus(Self::TimedOutWaitingForClient,full_msg.to_string())),
            b"500" => Err(Error::SmtpStatus(Self::CommandUnrecognised,full_msg.to_string())),
            b"501" => Err(Error::SmtpStatus(Self::SyntaxError,full_msg.to_string())),
            b"503" => Err(Error::SmtpStatus(Self::BadCommandSequence,full_msg.to_string())),
            b"530" => Err(Error::SmtpStatus(Self::AuthenticationFailed,full_msg.to_string())),
            b"535" => Err(Error::SmtpStatus(Self::AuthenticationRequired,full_msg.to_string())),
            _ =>      Err(Error::SmtpStatus(Self::Unknown,full_msg.to_string()))
        }
    }
}