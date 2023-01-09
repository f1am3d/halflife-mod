use crate::vector::Vector;

extern "C" {
  fn SetAnimation(anim: PlayerAnim);
}


enum PlayerAnim {
  PlayerIdle,
  PlayerWalk,
  PlayerJump,
  PlayerSuperjump,
  PlayerDie,
  PlayerAttack1,
}




struct TraceResult {
fAllSolid:   i32,	 // if true, plane is not valid
fStartSolid: i32 , // if true, the initial point was in a solid area
fInOpen: i32 ,
fInWater:   i32 ,
flFraction:   f32 , // time completed, 1.0 = didn't hit anything
vecEndPos: Vector , // final position
flPlaneDist: f32 ,
vecPlaneNormal:   Vector , // surface normal at impact
 pHit: &edict_t,		   // entity the surface is on
iHitgroup: i32 ,		   // 0 == generic, non zero is specific body part
}

#[no_mangle]
pub struct CBasePlayer {}
impl CBasePlayer {
  #[no_mangle]
  pub extern "C" fn Jump() {
    let vecWallCheckDir: Vector ; // direction we're tracing a line to find a wall when walljumping
    let vecAdjustedVelocity: Vector     ;
    let vecSpot: Vector    ;
    let tr: TraceResult    ;

    if FBitSet(pev.flags, FL_WATERJUMP) {
      return;
    }

    if pev.waterlevel >= 2 {
      return;
    }

    // jump velocity is sqrt( height * gravity * 2)

    // If this isn't the first frame pressing the jump button, break out.
    if !FBitSet(m_afButtonPressed, IN_JUMP) {
      return; // don't pogo stick
    }

    if (pev.flags & FL_ONGROUND) == 0 || !pev.groundentity {
      return;
    }

    // many features in this function use v_forward, so makevectors now.
    UTIL_MakeVectors(pev.angles);

    // ClearBits(pev->flags, FL_ONGROUND);		// don't stairwalk

    unsafe {
      SetAnimation(PlayerAnim::PlayerJump);
    }

    if m_fLongJump &&
      (pev.button & IN_DUCK) != 0 &&
      (pev.flDuckTime > 0) &&
      pev.velocity.Length() > 50
    {

      unsafe {
        SetAnimation(PLAYER_SUPERJUMP);
      }
    }

    // If you're standing on a conveyor, add it's velocity to yours (for momentum)
    entvars_t * pevGround = VARS(pev.groundentity);

    if pevGround && (pevGround.flags & FL_CONVEYOR) != 0
    {
      pev.velocity = pev.velocity + pev.basevelocity;
    }
  }
}
