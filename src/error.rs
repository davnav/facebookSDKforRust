use std::fmt;
//use thiserror::Error;
//type NewResult<()> = Result<(),Box<dyn FacebookError>>
//#[derive(Error,Debug)]
#[derive(Debug)]
pub enum FacebookError{
    SerdeError,
    //         #[error("serde error")]
    //         SerdeError ( serde_json::Error) ,
    // #[error("invalid URI")]
    // HyperInvalidUri ( #[from] hyper::http::uri::InvalidUri ),

    HyperInvalidUri,
    // #[error("hyper error")]
    // HyperError (#[from]  hyper::error::Error),

     ReqwestError,
   // #[error("Uri error")]
   // HyperUriErr( #[from] <hyper::Uri as std::str>::Error),


  //  #[error("std error")]
  //  StdError( #[from] std::error::Error)
  //  #[error("other")]
  //  Other(#[from] anyhow::Error),
}

impl std::error::Error for FacebookError{}

impl fmt::Display for FacebookError{
    fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result{
        match self{
            FacebookError::SerdeError => { write!( f ,"Serde Error") } ,
            FacebookError::HyperInvalidUri => { write!(f,"Hyper uri error")},
            FacebookError::ReqwestError => { write!(f,"Reqwest error")},
//            FacebookError::

        }
    }
}

impl From<serde_json::Error> for FacebookError{
    fn from(_:serde_json::Error) -> Self{
            FacebookError::SerdeError
    }
}
impl From<reqwest::Error> for FacebookError{
    fn from(_:reqwest::Error) -> Self{
            FacebookError::ReqwestError
    }
}


