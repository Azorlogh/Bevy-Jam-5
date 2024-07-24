use std::convert::identity;

use bevy::{color::palettes::css::BLUE, prelude::*};

use crate::loddy::{ring::Ring, ChunkReady, ChunkVisibility};

pub struct Lod2dPlugin;

impl Plugin for Lod2dPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Lod2dTree::default())
            .insert_resource(InvalidSlots(vec![]))
            .add_systems(Startup, setup)
            .add_systems(
                PostUpdate,
                (update_lod, spawn_chunks, swap_chunks).chain(),
            );
    }
}

#[derive(Default, Resource)]
pub struct Lod2dTree {
    pub map: Ring<Ring<Slot, LOD_GRID_LEN>, LOD_GRID_LEN>,
    pub pos: Vec2,
    pub prev_pos: Vec2,
}

pub const LOD_GRID_EXTENT: u32 = 2;
pub const LOD_GRID_LEN: usize = (LOD_GRID_EXTENT as usize) * 2 + 1;

impl Lod2dTree {
    fn shift_up(&mut self) -> [Slot; LOD_GRID_LEN] {
        self.map.shift_right().to_array()
    }

    fn shift_down(&mut self) -> [Slot; LOD_GRID_LEN] {
        self.map.shift_left().to_array()
    }

    fn shift_right(&mut self) -> [Slot; LOD_GRID_LEN] {
        self.map
            .iter_mut()
            .map(|d| d.shift_right())
            .collect::<Vec<Slot>>()
            .try_into()
            .unwrap()
    }

    fn shift_left(&mut self) -> [Slot; LOD_GRID_LEN] {
        self.map
            .iter_mut()
            .map(|d| d.shift_left())
            .collect::<Vec<Slot>>()
            .try_into()
            .unwrap()
    }
}

pub const LOD_LEVELS_PER_CHUNK_EXTENT: u32 = 2;
pub const LOD_LEVELS_PER_CHUNK: usize = (LOD_LEVELS_PER_CHUNK_EXTENT as usize) * 2 + 1;

#[derive(Debug, Default, Clone, Copy)]
pub struct Slot(pub Ring<Option<Entity>, LOD_LEVELS_PER_CHUNK>);

fn render_lod(lod: ResMut<Lod2dTree>, mut gizmos: Gizmos) {
    let center = lod.pos.round();
    for r in 0..LOD_GRID_LEN {
        for c in 0..LOD_GRID_LEN {
            let offset_r = (r as i32 - LOD_GRID_LEN as i32 / 2) as f32;
            let offset_c = (c as i32 - LOD_GRID_LEN as i32 / 2) as f32;
            gizmos.rect_2d(center - Vec2::new(offset_c, offset_r), 0.0, Vec2::ONE, BLUE)
        }
    }
}

fn setup(mut invalid_slots: ResMut<InvalidSlots>) {
    for r in 0..LOD_GRID_LEN {
        for c in 0..LOD_GRID_LEN {
            let offset = IVec2::new(
                c as i32 - LOD_GRID_EXTENT as i32,
                r as i32 - LOD_GRID_EXTENT as i32,
            );
            invalid_slots.0.push(offset);
        }
    }
}

#[derive(Resource)]
pub struct InvalidSlots(Vec<IVec2>);

pub fn update_lod(
    mut cmds: Commands,
    mut lod: ResMut<Lod2dTree>,
    // mut q_chunks: Query<&mut ChunkVisibility>,
    mut invalid_slots: ResMut<InvalidSlots>,
) {
    let prev_anchor = lod.prev_pos.round().as_ivec2();
    let new_anchor = lod.pos.round().as_ivec2();

    let offset = new_anchor - prev_anchor;

    let mut despawn_slots = |slots: [Slot; LOD_GRID_LEN]| {
        for e in slots.into_iter().flat_map(|s| s.0).filter_map(identity) {
            cmds.entity(e).despawn_recursive();
        }
    };

    for _ in offset.y..0 {
        despawn_slots(lod.shift_up());
    }
    for _ in 0..offset.y {
        despawn_slots(lod.shift_down());
    }
    for _ in offset.x..0 {
        despawn_slots(lod.shift_right());
    }
    for _ in 0..offset.x {
        despawn_slots(lod.shift_left());
    }

    if offset != IVec2::ZERO {
        for r in 0..LOD_GRID_LEN {
            for c in 0..LOD_GRID_LEN {
                let offset = IVec2::new(
                    c as i32 - LOD_GRID_EXTENT as i32,
                    r as i32 - LOD_GRID_EXTENT as i32,
                );

                let prev_dist = (offset - (prev_anchor - new_anchor)).abs().max_element() as usize;
                let new_dist = offset.abs().max_element() as usize;

                let lod_offset = new_dist as i32 - prev_dist as i32;

                for _ in lod_offset..0 {
                    lod.map[r][c]
                        .0
                        .shift_right()
                        .map(|e| cmds.entity(e).despawn_recursive());
                }
                for _ in 0..lod_offset {
                    lod.map[r][c]
                        .0
                        .shift_left()
                        .map(|e| cmds.entity(e).despawn_recursive());
                }

                if lod_offset != 0 {
                    invalid_slots.0.push(offset);
                }
            }
        }
    }

    lod.prev_pos = lod.pos;
}

#[derive(Component)]
pub struct Chunk {
    pub coord: IVec2,
    pub lod: u32,
}

pub fn spawn_chunks(mut cmds: Commands, mut lod: ResMut<Lod2dTree>) {
    for r in 0..LOD_GRID_LEN as i32 {
        for c in 0..LOD_GRID_LEN as i32 {
            let offset = IVec2::new(c - LOD_GRID_EXTENT as i32, r - LOD_GRID_EXTENT as i32);

            let coord = lod.pos.round().as_ivec2() + offset;

            let dist = offset.abs().max_element() as usize;
            let slot = &mut lod.map[r as usize][c as usize];
            if slot.0[LOD_LEVELS_PER_CHUNK_EXTENT as usize].is_none() {
                slot.0[LOD_LEVELS_PER_CHUNK_EXTENT as usize] = Some(
                    cmds.spawn((
                        ChunkVisibility::Hidden,
                        Chunk {
                            coord,
                            lod: dist as u32,
                        },
                    ))
                    .id(),
                );
            }
        }
    }
}

pub fn swap_chunks(
    tree: ResMut<Lod2dTree>,
    mut q_chunk: Query<&mut ChunkVisibility>,
    q_ready: Query<(), With<ChunkReady>>,
    mut invalid_slots: ResMut<InvalidSlots>,
) {
    invalid_slots.0.retain(|offset| {
        let [c, r] = [
            offset.x + LOD_GRID_EXTENT as i32,
            offset.y + LOD_GRID_EXTENT as i32,
        ];

        if r < 0 || r >= LOD_GRID_LEN as i32 || c < 0 || c >= LOD_GRID_LEN as i32 {
            return false;
        }

        let slot = tree.map[r as usize][c as usize];

        if slot.0[LOD_LEVELS_PER_CHUNK_EXTENT as usize]
            .map(|e| q_ready.contains(e))
            .unwrap_or(false)
        {
            for (i, e) in slot
                .0
                .iter()
                .enumerate()
                .filter_map(|(i, e)| e.map(|e| (i, e)))
            {
                let mut vis = q_chunk.get_mut(e).unwrap();
                *vis = if i as u32 == LOD_LEVELS_PER_CHUNK_EXTENT {
                    ChunkVisibility::Visible
                } else {
                    ChunkVisibility::Hidden
                }
            }
            false
        } else {
            true
        }
    });
}

// #[derive(Event)]
// pub struct ChunkReady(pub IVec2);

// pub fn swap_chunks(
// 	tree: ResMut<Lod2dTree>,
// 	mut ev_chunk_ready: EventReader<ChunkReady>,
// 	mut q_chunks: Query<&mut ChunkVisibility>,
// ) {
// 	for ev in ev_chunk_ready.read() {
// 		let offset = ev.0 - tree.pos.round().as_ivec2();
// 		let [c, r] = [
// 			offset.x + LOD_GRID_EXTENT as i32,
// 			offset.y + LOD_GRID_EXTENT as i32,
// 		];

// 		if r < 0 || r >= LOD_GRID_LEN as i32 || c < 0 || c >= LOD_GRID_LEN as i32 {
// 			continue;
// 		}

// 		let slot = tree.map[r as usize][c as usize];
// 		for (i, e) in slot
// 			.0
// 			.iter()
// 			.enumerate()
// 			.filter_map(|(i, e)| e.map(|e| (i, e)))
// 		{
// 			let mut vis = q_chunks.get_mut(e).unwrap();
// 			*vis = if i as u32 == LOD_LEVELS_PER_CHUNK_EXTENT {
// 				ChunkVisibility::Visible
// 			} else {
// 				ChunkVisibility::Hidden
// 			}
// 		}
// 	}
// }
