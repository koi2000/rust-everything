mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
// 使用pub use导出，对外仍是可用的
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
