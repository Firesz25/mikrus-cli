use anyhow::Result;
use crate::config::Config;
use crate::opt::Opt;
pub struct ApiClient(reqwest::Client);

pub fn get_config(opt: Opt) -> Result<Config> {
    if opt.srv.is_some() && opt.key.is_some() {
        Ok(Config {
            srv: opt.srv.unwrap(),
            key: opt.key.unwrap(),
        })
    } else {
        let config = Config::from_file().unwrap();
        Ok(config)
    }
}

impl ApiClient {
    pub fn new() -> Self {
        ApiClient(reqwest::Client::builder().user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"))).timeout(std::time::Duration::from_secs(60)).build().unwrap())
    }

    pub async fn info(&self, srv: String, key: String) -> Result<String> {
        let res = self.0.post("https://api.mikr.us/info").form(&[("srv", srv.as_str()), ("key", key.as_str())]).send().await.unwrap();
        let json = res.text().await.unwrap();
        Ok(json)
    }

    pub async fn serwery(&self, srv: String, key: String) -> Result<String> {
        let res = self.0.post("https://api.mikr.us/serwery").form(&[("srv", srv.as_str()), ("key", key.as_str())]).send().await.unwrap();
        let json = res.text().await.unwrap();
        Ok(json)
    }

    pub async fn restart(&self, srv: String, key: String) -> Result<String> {
        let res = self.0.post("https://api.mikr.us/restart").form(&[("srv", srv.as_str()), ("key", key.as_str())]).send().await.unwrap();
        let json = res.text().await.unwrap();
        Ok(json)
    }
    
    pub async fn logs(&self, srv: String, key: String) -> Result<String> {
        let res = self.0.post("https://api.mikr.us/logs").form(&[("srv", srv.as_str()), ("key", key.as_str())]).send().await.unwrap();
        let json = res.text().await.unwrap();
        Ok(json)
    }

    pub async fn log_id(&self, srv: String, key: String, id: &str) -> Result<String> {
        let res = self.0.post(format!("https://api.mikr.us/logs/{}", id)).form(&[("srv", srv.as_str()), ("key", key.as_str())]).send().await.unwrap();
        let json = res.text().await.unwrap();
        Ok(json)
    }

    pub async fn amfetamina(&self, srv: String, key: String) -> Result<String> {
        let res = self.0.post("https://api.mikr.us/amfetamina").form(&[("srv", srv.as_str()), ("key", key.as_str())]).send().await.unwrap();
        let json = res.text().await.unwrap();
        Ok(json)
    }

    pub async fn db(&self, srv: String, key: String) -> Result<String> {
        let res = self.0.post("https://api.mikr.us/db").form(&[("srv", srv.as_str()), ("key", key.as_str())]).send().await.unwrap();
        let json = res.text().await.unwrap();
        Ok(json)
    }

    pub async fn exec(&self, srv: String, key: String, cmd: &str) -> Result<String> {
        let res = self.0.post("https://api.mikr.us/exec").form(&[("srv", srv.as_str()), ("key", key.as_str()), ("cmd", cmd)]).send().await.unwrap();
        let json = res.text().await.unwrap();
        Ok(json)
    }

    pub async fn stats(&self, srv: String, key: String) -> Result<String> {
        let res = self.0.post("https://api.mikr.us/stats").form(&[("srv", srv.as_str()), ("key", key.as_str())]).send().await.unwrap();
        let json = res.text().await.unwrap();
        Ok(json)
    }
    
    pub async fn porty(&self, srv: String, key: String) -> Result<String> {
        let res = self.0.post("https://api.mikr.us/porty").form(&[("srv", srv.as_str()), ("key", key.as_str())]).send().await.unwrap();
        let json = res.text().await.unwrap();
        Ok(json)
    }
}



#[tokio::test]
async fn test() {
    reqwest::get("https://api.mikr.us").await.unwrap();
}