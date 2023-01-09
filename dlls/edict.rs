use std::ffi::{c_short, c_void};

const MAX_ENT_LEAFS: i32 = 48;

extern "C" {
  static leafnums: &[i32];
  type entvars_t;
}

struct edict_s
{
  free: bool,
  serialnumber: i32,
  area: link_t, // linked to a division node or leaf
  headnode: i32, // -1 to use normal leaf check
  num_leafs: i32,
  leafnums: c_short[MAX_ENT_LEAFS],
  freetime: f32, // sv.time when the object was freed
  pvPrivateData: c_void, // Alloced and freed by engine, used by DLLs
  v: entvars_t, // C exported fields from progs
};