
use hyper::Client;

static FACEBOOK_GRAPH_URL:&str = "http://graph.facebook.com";
static FACEBOOK_WWW_URL:&str = "http://www.facebook.com";
static FACEBOOK_OAUTH_DIALOG_PATH:&str = "dialog/oauth?";
static VALID_API_VERSIONS:[&str;7] = ["3.1","3.2","3.3","4.0","5.0","6.0","7.0"];
static VALID_SEARCH_TYPES:[&str;2] = ["place","placetopic"];



#[derive(Debug)]
struct GraphAPI{
    access_token:Option<String>,
    timeout: Option<f64>,
    version:Option<String>,
    proxies: Option<Vec<String>>,
    session:Option<String>,
    app_secret:Option<String>,
}
impl GraphAPI{
    fn new() -> Self{
        GraphAPI{
            access_token:None,
            timeout:None,
            version:Some("v8.0".to_string()),
            proxies:None,
            session:None,
            app_secret:None,
        }
    }
    fn with_acces_token(self,access_token:Option<String>) -> Self{
            GraphAPI{
                access_token,
                timeout:self.timeout,
                version:self.version,
                proxies:self.proxies,
                session:self.session,
                app_secret:self.app_secret,


            }
    }
    fn with_timeout(self,timeout:Option<f64>) -> Self{
            GraphAPI{
                access_token:self.access_token,
                timeout,
                version:self.version,
                proxies:self.proxies,
                session:self.session,
                app_secret:self.app_secret,


            }
        }
    fn with_version(self,version:Option<String>) -> Self{
            GraphAPI{
                access_token:self.access_token,
                timeout:self.timeout,
                version,
                proxies:self.proxies,
                session:self.session,
                app_secret:self.app_secret,


            }
    }
    fn with_proxies(self,proxies:Option<Vec<String>>) -> Self{
            GraphAPI{
                access_token:self.access_token,
                timeout:self.timeout,
                version:self.version,
                proxies,
                session:self.session,
                app_secret:self.app_secret,


            }
    }

    fn with_session(self,session:Option<String>) -> Self{
            GraphAPI{
                access_token:self.access_token,
                timeout:self.timeout,
                version:self.version,
                proxies:self.proxies,
                session,
                app_secret:self.app_secret,


            }
    }

    fn with_app_secret(self,app_secret:Option<String>) -> Self{
            GraphAPI{
                access_token:self.access_token,
                timeout:self.timeout,
                version:self.version,
                proxies:self.proxies,
                session:self.session,
                app_secret,


            }
    }



    fn execute(&self){

        println!("{:?}",self);
        todo!(); 

    }


}
mod test{
    use super::*;
    #[test]
    fn it_works(){
        let graph = GraphAPI::new();
        graph
            .with_acces_token(Some("asdasdasd".to_string()))
            .with_version(Some("v.8.0".to_string()))
            .execute();
    }
}