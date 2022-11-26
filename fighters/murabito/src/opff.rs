// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

unsafe fn respawn_invincibility(boma: &mut BattleObjectModuleAccessor, status_kind: i32) {
    if !VarModule::is_flag(boma.object(), vars::common::status::IS_RESPAWN_INVINCIBLE) {
        return;
    }
    if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_ALL){
        VarModule::off_flag(boma.object(), vars::common::status::IS_RESPAWN_INVINCIBLE);
        HitModule::set_invincible_frame_global(boma, 0, true, 0);
    }
}
 
pub unsafe fn moveset(boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    respawn_invincibility(boma, status_kind);
}

#[utils::macros::opff(FIGHTER_KIND_MURABITO )]
pub fn murabito_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		murabito_frame(fighter)
    }
}

pub unsafe fn murabito_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(&mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}