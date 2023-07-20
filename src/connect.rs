
#[derive(PartialEq,Debug)]
enum Method {
    Password,
    PublicKey,
    KeyboardInteractive
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Connect {
    name:String,
    host:String,
    port:String,
    remark:String,
    method:Method,
    user_name:String,
    password:String,
    cert:String
}

pub struct ConnectList(Vec<Connect>);

impl Default for Connect {
    fn default() -> Self {
        Self {
            name:"".to_string(),
            host:"".to_string(),
            port:"".to_string(),
            remark:"".to_string(),
            method:Method::Password,
            user_name:"".to_string(),
            password:"".to_string(),
            cert:"".to_string()
        }
    }
}


impl super::RShellUIComponent for Connect {

    fn name(&self) -> &'static str {
        "connect"
    }

    fn ui(&mut self, ui: &mut egui::Ui) {
        
        self.ui_1(ui);
        ui.add_space(10.0);
        //ui.separator();

        self.ui_2(ui);
        ui.add_space(10.0);
        //ui.separator();

        // self.ui_3(ui);
        // ui.add_space(10.0);
        // ui.separator();

        self.ui_btn(ui);
        ui.add_space(10.0);

    }

    fn show(&mut self, ctx: &egui::Context, open: &mut bool) {

        let window=egui::Window::new(self.name())
            .vscroll(false)
            .resizable(false)
            .collapsible(false)
            .open(open);

        window.show(ctx, |ui| {
            self.ui(ui);
        });

    }
}

impl Connect {
    fn ui_1(&mut self, ui: &mut egui::Ui){

        let Self{name,host,port,remark,..} =self;

        ui.heading("常规");
        ui.group(|ui| {
            ui.horizontal(|ui: &mut egui::Ui|{
                ui.label("名称");
                ui.add(egui::TextEdit::singleline(name).hint_text("连接名称"));
            });
            ui.horizontal(|ui: &mut egui::Ui|{
                ui.label("主机");
                ui.add(
                    egui::TextEdit::singleline(host)
                    .id(egui::Id::from("12"))
                    .desired_width(150.0)
                    .hint_text("主机")
                );
                ui.add_space(3.0);
                ui.label("端口");
                ui.add(
                    egui::TextEdit::singleline(port)
                    .id(egui::Id::from("13"))
                    .desired_width(80.0)
                    .hint_text("端口")
                );
            });
        
            ui.horizontal(|ui: &mut egui::Ui|{
                ui.label("备注");
                ui.text_edit_multiline(remark);
            });
        });

        

        
    }

    fn ui_2(&mut self, ui: &mut egui::Ui){

        let Self{method,user_name,password,cert,..} =self;

        ui.heading("认证");
        ui.group(|ui| {
            ui.horizontal(|ui: &mut egui::Ui|{
                ui.label("方法");
                
                egui::ComboBox::from_label("请选择认证方法")
                    .selected_text(format!("{:?}", method))
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        ui.set_min_width(60.0);
                        ui.selectable_value(method, Method::Password, "密码");
                        ui.selectable_value(method, Method::PublicKey, "公钥");
                        ui.selectable_value(method, Method::KeyboardInteractive, "Keyboard Interactive");
                    })
            });
            ui.horizontal(|ui: &mut egui::Ui|{
                ui.label("用户");
                ui.add(egui::TextEdit::singleline(user_name).hint_text("用户名"));
            });
            ui.horizontal(|ui: &mut egui::Ui|{
                ui.label("密码");
                ui.add(egui::TextEdit::singleline(password).password(true).hint_text("密码"));
            });
            ui.horizontal(|ui: &mut egui::Ui|{
                ui.label("证书");
                ui.text_edit_multiline(cert);
            });
            
        });
       
    }

    // fn ui_3(&mut self, ui: &mut egui::Ui){

    // }

    fn ui_btn(&mut self, ui: &mut egui::Ui){

        ui.horizontal(|ui: &mut egui::Ui|{
            if ui.button("保存").clicked() {
                
            }
    
            if ui.button("取消").clicked() {
                
            }
        });

    }


}

