mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
// 使用use关键字进行导入
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}