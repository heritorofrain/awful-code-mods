#![feature(lazy_cell, ptr_sub_ptr)]
use unity::{prelude::*};
use unity::system::List;
use engage::gamedata::unit::Unit;
use engage::hub::*;

#[unity::class("App", "HubLocatorGroup.CreateCharacter")]
pub struct CreateCharacter {
    pub active_group_root: u64,
    pub unit_list: u64,
    pub access_list: u64,
    pub active: bool,
    pub system_active: bool,
    pub event_active: bool,
    pub inactive_objects: u64,
    pub loading_chara_count: i32,
    pub hash_table: u64,
}

#[unity::class("App", "HubLocatorGroup.CreateCharacter")]
//#[skyline::hook(offset=0x028ae890)]
pub fn hubunit_makespawn(this: &mut CreateCharacter, pid: &Il2CppString, locator: &Il2CppArray, access: &HubAccessData, callback: u64, method_info: OptionalMethod) {
    call_original!(this, pid, locator, access, callback, method_info);
    println!("plugin did not crash, good job on the bare minimum lmao")
}
#[skyline::main(name = "hooks")]
pub fn main() {
    skyline::install_hook!(hubunit_makespawn);
}
