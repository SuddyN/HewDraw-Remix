use super::*;
use globals::*;

// This file contains code for respawn platforms

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            sub_rebirth_uniq_process_exit,
        );
    }
}

#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_rebirth_uniq_process_exit)]
unsafe fn sub_rebirth_uniq_process_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let ret = call_original!(fighter);
    VarModule::on_flag(fighter.battle_object, vars::common::status::IS_RESPAWN_INVINCIBLE);
    return ret;
}