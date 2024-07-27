use std::f32::consts::TAU;

use bevy::{
    audio::{PlaybackSettings, SpatialScale, Volume},
    math::prelude::*,
};
use rand::Rng;

pub fn poisson_disc_sampling(radius: f32, region_size: f32, n: usize) -> Vec<Vec2> {
    let cell_size = radius / 2f32.sqrt();

    let nb_cells = (region_size / cell_size).ceil() as usize;
    let mut grid: Vec<Option<usize>> = (0..nb_cells)
        .flat_map(|_| (0..nb_cells).map(|_| None))
        .collect();

    let mut points: Vec<Vec2> = vec![];
    let mut spawn_points = vec![Vec2::splat(region_size / 2.0)];

    let mut rng = rand::thread_rng();

    let is_valid = |grid: &[Option<usize>], points: &[Vec2], candidate: Vec2| -> bool {
        let cell = (candidate / cell_size).as_ivec2();
        if cell != cell.clamp(IVec2::splat(0), IVec2::splat(nb_cells as i32 - 1)) {
            return false;
        }

        let search_start = (cell - 2).max(IVec2::ZERO);
        let search_end = (cell + 2).min(IVec2::splat(nb_cells as i32 - 1));

        for x in search_start.x..=search_end.x {
            for y in search_start.y..=search_end.y {
                let x = x as usize;
                let y = y as usize;

                if let Some(idx) = grid[y * nb_cells + x] {
                    if (candidate - points[idx]).length_squared() < radius * radius {
                        return false;
                    }
                }
            }
        }
        true
    };

    'outer: while spawn_points.len() > 0 && points.len() < n {
        let spawn_idx = rng.gen_range(0..spawn_points.len());
        let spawn_center = spawn_points[spawn_idx];

        for _ in 0..30 {
            let dir = Vec2::from_angle(rng.r#gen::<f32>() * TAU);
            let candidate = spawn_center + dir * rng.gen_range(radius..2.0 * radius);
            if is_valid(&grid, &points, candidate) {
                points.push(candidate);
                spawn_points.push(candidate);
                let idx = (candidate.x / cell_size) as usize
                    + (candidate.y / cell_size) as usize * nb_cells;
                grid[idx] = Some(points.len() - 1);
                continue 'outer;
            }
        }
        spawn_points.remove(spawn_idx);
    }

    points
}

pub fn spatial_playback_remove(volume: f32, scale: f32) -> PlaybackSettings {
    PlaybackSettings::REMOVE
        .with_spatial(true)
        .with_volume(Volume::new(volume))
        .with_spatial_scale(SpatialScale::new(scale))
}
