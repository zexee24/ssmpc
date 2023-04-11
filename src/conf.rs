use serde::Deserialize;
use crate::auth::Key;

static CONF :&str = r#"{
	"hostIp": "0.0.0.0",
	"hostPort": 8008,
	"hostKey": "music1sBest",
	"keys": [
		{
			"key": "basic",
			"permissions": ["Add", "Download", "Info"]
		},
		{
			"key": "master",
			"permissions": ["All"]
		}
	]
}"#;

#[derive(Deserialize, PartialEq, Debug, Clone)]
pub struct Configuration{
    #[serde(alias = "hostIp")]
    pub host_ip: String,
    #[serde(alias = "hostPort")]
    pub host_port: i16,
    #[serde(alias = "hostKey")]
    pub host_key: String,
    pub keys: Vec<Key>
}

impl Configuration{
    pub fn new() -> Result<Self, String>{
            let mut conf: Configuration = serde_json::from_str(CONF).map_err(|e| e.to_string())?;
            conf.keys = conf.keys.iter().map(|k|{
                k.convert_all()
            }).collect();
            Ok(conf)
    }
    pub fn host_url(&self) -> String{
        format!("http://{}:{}",self.host_ip, self.host_port)
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_reading_conf(){
        println!("{:?}", Configuration::new());
    }
}
