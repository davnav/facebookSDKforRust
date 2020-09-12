
use hyper::{Client, body::HttpBody as _};
use hyper_tls::HttpsConnector;
use tokio::io::{self, AsyncWriteExt as _};




static FACEBOOK_GRAPH_URL:&str = "https://graph.facebook.com";
static FACEBOOK_WWW_URL:&str = "https://www.facebook.com";
static FACEBOOK_OAUTH_DIALOG_PATH:&str = "dialog/oauth?";
static VALID_API_VERSIONS:[&str;7] = ["3.1","3.2","3.3","4.0","5.0","6.0","7.0"];
static VALID_SEARCH_TYPES:[&str;2] = ["place","placetopic"];



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

        //let mut url = 
        println!("{:?}",self);
        self 

    }


    pub fn get_permissions(&self,user_id:String) -> Vec<String>{

        let url = format!("{}/{}/{}/permissions",FACEBOOK_GRAPH_URL,self.version,user_id);
        let response = self.request(url);
        todo!()
    }
   // #[tokio::main]
    pub async fn request(&self,url:String) -> Result<(),Box<dyn std::error::Error>>{
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_,hyper::Body>(https);
        let mut res = client.get(url.parse()?).await?; 
        Ok(())
    }

}
mod test{
    use super::*;
    #[test]
    fn it_works(){
        let graph = GraphAPI::new();
        graph
            .with_acces_token(Some("asdasdasd".to_string()))
            .with_version("v.8.0".to_string())
            .build();
    }
}