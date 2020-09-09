

struct GraphAPI{
    access_token:Option<String>,
    timeout: u32,
    version:Option<String>,
    proxies: Option<String>,
    session:Option<String>,
    app_secret:Option<String>,
}

mod test{
    #[test]
    fn it_works(){
        assert_eq!(1+2,3);
    }
}