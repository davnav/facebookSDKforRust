
///Rust library for the facebook Platform
/// This is a client library designed for the Graph API's in Rust.
/// Read more about the Graph API at https://developers.facebook.com/docs/graph-api 
///

///libraries used for the project
use std::str;
use tokio::io::{self, AsyncWriteExt as _};
use std::time::Duration;
use tokio::task;
use tokio::time;
use tokio::prelude::*;
use tokio_test::block_on;
use std::env;
use serde_json::{ Value};
use serde::{ Serialize,Deserialize };
use anyhow::Context;
use anyhow::Result;

mod error;
use error::FacebookError;

///static variables using
static FACEBOOK_GRAPH_URL:&str = "https://graph.facebook.com";
static FACEBOOK_WWW_URL:&str = "https://www.facebook.com";
static FACEBOOK_OAUTH_DIALOG_PATH:&str = "dialog/oauth?";
static VALID_API_VERSIONS:[&str;7] = ["3.1","3.2","3.3","4.0","5.0","6.0","7.0"];
static VALID_SEARCH_TYPES:[&str;2] = ["place","placetopic"];


///A client for Facebook Graph API.
///Graph API made up of objects in Facebook(eg. people,photos,pages,events ..etc)
/// This Rust client provides access to those primitive types in a generic way.
/// For example, for a given Oauth token, the below code fetch the profile of the 
/// active user permissions
///  graph = 
/// let graph = GraphAPI::new();
/// let g = graph
///            .with_acces_token(Some(env::var("FACEBOOK_ACCESSTOKEN").unwrap().to_string()))
///            .with_version("v8.0".to_string())
///            .build();
/// tokio::block_on(g.get_permissions("me".to_string()))


#[derive(Serialize,Deserialize,Debug)]
pub struct Data{
    data:Vec<Permissions>,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Permissions{
    permission:String,
    status:String,
}

#[derive(Debug)]
pub struct GraphAPI{
    access_token:Option<String>,
    timeout: Option<f64>,
    version:String,
    proxies: Option<Vec<String>>,
    session:Option<String>,
    app_secret:Option<String>,
}
impl GraphAPI{
    pub fn new() -> Self{
        GraphAPI{
            access_token:None,
            timeout:None,
            version:"v8.0".to_string(),
            proxies:None,
            session:None,
            app_secret:None,
        }
    }
    pub fn with_acces_token(self,access_token:Option<String>) -> Self{
            GraphAPI{
                access_token,
                timeout:self.timeout,
                version:self.version,
                proxies:self.proxies,
                session:self.session,
                app_secret:self.app_secret,


            }
    }
    pub fn with_timeout(self,timeout:Option<f64>) -> Self{
            GraphAPI{
                access_token:self.access_token,
                timeout,
                version:self.version,
                proxies:self.proxies,
                session:self.session,
                app_secret:self.app_secret,


            }
        }
    pub fn with_version(self,version:String) -> Self{
            GraphAPI{
                access_token:self.access_token,
                timeout:self.timeout,
                version,
                proxies:self.proxies,
                session:self.session,
                app_secret:self.app_secret,


            }
    }
    pub fn with_proxies(self,proxies:Option<Vec<String>>) -> Self{
            GraphAPI{
                access_token:self.access_token,
                timeout:self.timeout,
                version:self.version,
                proxies,
                session:self.session,
                app_secret:self.app_secret,


            }
    }

    pub fn with_session(self,session:Option<String>) -> Self{
            GraphAPI{
                access_token:self.access_token,
                timeout:self.timeout,
                version:self.version,
                proxies:self.proxies,
                session,
                app_secret:self.app_secret,


            }
    }

    pub fn with_app_secret(self,app_secret:Option<String>) -> Self{
            GraphAPI{
                access_token:self.access_token,
                timeout:self.timeout,
                version:self.version,
                proxies:self.proxies,
                session:self.session,
                app_secret,


            }
    }

    pub fn build(self) -> Self{

        self 

    }


    //pub  async fn get_object(&self,user_id:String) -> Result<(Vec<Object>),FacebookError>{
    pub  async fn get_object(&self,user_id:String) -> Result<(),FacebookError>{

        let url = format!("{}/{}/{}?access_token={}",FACEBOOK_GRAPH_URL,self.version,user_id,self.access_token.clone().unwrap());
        println!("{:?}",url);

        let res = reqwest::get(&url).await?;
       let body = res.text().await?;
       println!("{}",body);
       Ok(())
    }     
    

    pub  async fn get_permissions(&self,user_id:String) -> Result<(Vec<Permissions>),FacebookError>{

        let url = format!("{}/{}/{}/permissions?access_token={}",FACEBOOK_GRAPH_URL,self.version,user_id,self.access_token.clone().unwrap());
        println!("{:?}",url);

        let res = reqwest::get(&url).await?;
        let body :Data = res.json().await?;
        Ok((body.data))
         
    }
   

}
mod test{
    use super::*;
    use tokio::prelude::*;
    #[test]
   
    fn get_permission_test() {

        let graph = GraphAPI::new();
        let g = graph
            .with_acces_token(Some(env::var("FACEBOOK_ACCESSTOKEN").unwrap().to_string()))
            .with_version("v8.0".to_string())
            .build();
            
                 let my_permissions = tokio_test::block_on(g.get_permissions("me".to_string()));
                 for i in my_permissions.unwrap().iter(){
                     println!("{}, {}",i.permission,i.status);
                 }
    }
    #[test]
    fn get_object_test() {

        let graph = GraphAPI::new();
        let g = graph
            .with_acces_token(Some(env::var("FACEBOOK_ACCESSTOKEN").unwrap().to_string()))
            .with_version("v8.0".to_string())
            .build();
            
                 let my_object = tokio_test::block_on(g.get_object("me".to_string()));
                //  for i in my_permissions.unwrap().iter(){
                //      println!("{}, {}",i.permission,i.status);
                //  }
    }





}