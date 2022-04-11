use actix_web::HttpRequest;

pub fn get_default_template(req: HttpRequest) -> (String, bool) {
    // получаем папку шаблона и проверяем на хост-админа
    let mut _type = "".to_string();
    for header in req.headers().into_iter() {
        if header.0 == "user-agent" {
            let _val = format!("{:?}", header.1);
            if _val.contains("Mobile"){
                _type = "mobile/".to_string();
            } else {
                _type = "desctop/".to_string();
            };
        }
    };

    let mut _is_host_admin : bool = false;
    let _val = format!("{:?}", Some(req.peer_addr()));
    if _val.contains(&"91.239.184.81".to_string()) {
        _is_host_admin = true;
    };
    (_type, _is_host_admin)
}
