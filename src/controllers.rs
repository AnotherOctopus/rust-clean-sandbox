use crate::{entities::Named, usecases};
pub struct Saveable {
    pub data: String,
}
pub fn create(user: String, pass: String, admin: bool) -> Saveable {
    let mut ret = Saveable {
        data: "".to_string(),
    };
    if admin {
        let admin = usecases::create_admin(user, pass);
        admin.sayName();
        ret.data = admin.stringify();
    } else {
        let user = usecases::create_user(user, pass);
        user.sayName();
        ret.data = user.stringify();
    }
    return ret;
}
