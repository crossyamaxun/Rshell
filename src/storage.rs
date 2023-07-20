use json::{JsonValue, object};


pub struct Storage{
    pub config:JsonValue
}

impl Default for Storage{
    fn default() -> Self {
        let path="/Users/wangsanfei/temp/Rshell/store.json".to_string();
        let config:JsonValue= match std::fs::read_to_string(&path) {
            Ok(str)=> {
                if let Ok(co)=json::parse(&str){
                    co
                }else{
                    panic!("配置文件读取异常");
                }
            } ,
            Err(_)=>{
                println!("读取失败");
                if let Err(_)=std::fs::File::create(&path) {
                    panic!("不能创建配置文件");
                }
                object!()
            }
        };

        Self {
            config
        }
    }
}

impl Storage{
    pub fn save_item(&mut self,key:String,value:JsonValue){
        self.config[key]=value;
    }

    pub fn get_item(&mut self,key:String) -> &mut JsonValue {
        &mut self.config[key]
    }

}