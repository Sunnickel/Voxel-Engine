use crate::config::{BiomRegistry, BlockRegistry, GenerationNoise};
use crate::world::utils::Rgba;
use bevy::log::info;
use bevy::prelude::{Commands, Component, Res};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Component, Debug, Clone)]
pub struct Biom {
    pub id: u16,
}

pub struct Climate {
    pub(crate) temperature: f32,
    pub(crate) wetness: f32,
    pub(crate) height: f32,
    pub(crate) continentalness: f32,
}

#[derive(Debug, Clone)]
pub struct BiomDef {
    pub id: String,
    pub display_name: String,
    pub wetness: f32,
    pub height: f32,
    pub temperature: f32,
    pub continentalness: f32,
    pub erosion: f32,
    pub height_multiplier: f32,
    pub tint: Rgba,

    pub surface: u16,
    pub underground: u16,
    pub underwater: u16,
    pub top_layer_threshold: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct RawBiomDef {
    pub id: String,
    pub display_name: String,
    pub wetness: f32,
    pub height: f32,
    pub temperature: f32,
    pub continentalness: f32,
    pub erosion: f32,
    pub height_multiplier: f32,
    pub tint: Rgba,

    pub surface: String,
    pub underground: String,
    pub underwater: String,
    pub top_layer_threshold: i32,
}

fn biome_distance(a: &Climate, b: &BiomDef) -> f32 {
    (a.temperature - b.temperature).powi(2) * 3.0 +
        (a.wetness - b.wetness).powi(2) * 2.0 +
        (a.continentalness - b.continentalness).powi(2) * 0.5 +
        (a.height - b.height).powi(2) * 0.3
}

pub fn pick_biome<'a>(climate: &Climate, biomes: &'a [BiomDef]) -> &'a BiomDef {
    assert!(!biomes.is_empty(), "BiomRegistry has no biomes loaded — check assets/bioms/");

    biomes
        .iter()
        .min_by(|a, b| {
            biome_distance(climate, a)
                .partial_cmp(&biome_distance(climate, b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap()
}

pub fn load_bioms(commands: &mut Commands, block_registry: &BlockRegistry, noise: Res<GenerationNoise>) {
    info!("Registering bioms");
    for i in [0, 100, 500, 1000, 2000] {
        info!(
        "x={} temp={:.3} wet={:.3} height={:.3} cont={:.3}",
        i,
        noise.temperature(i, 0),
        noise.wetness(i, 0),
        noise.height(i, 0),
        noise.continentalness(i, 0),
    );
    }

    let mut registry = BiomRegistry::default();

    let dir = Path::new("assets/bioms");
    let entries = fs::read_dir(dir).expect("bioms folder missing");

    let mut raw_defs: Vec<BiomDef> = Vec::new();

    for entry in entries {
        let path = entry.unwrap().path();
        info!("Loading bioms file {}", path.display());
        if path.extension().and_then(|s| s.to_str()) != Some("ron") {
            continue;
        }

        let text = fs::read_to_string(&path).expect("failed reading biom ron");
        let raw: RawBiomDef = ron::from_str(&text)
            .unwrap_or_else(|_| panic!("invalid biom ron: {:?}", path));

        let def = BiomDef {
            surface:     block_registry.get_or_air(&raw.surface),
            underground: block_registry.get_or_air(&raw.underground),
            underwater:  block_registry.get_or_air(&raw.underwater),
            id: raw.id,
            display_name: raw.display_name,
            wetness: raw.wetness,
            height: raw.height,
            temperature: raw.temperature,
            continentalness: raw.continentalness,
            erosion: raw.erosion,
            height_multiplier: raw.height_multiplier,
            tint: raw.tint,
            top_layer_threshold: raw.top_layer_threshold,
        };

        let id = raw_defs.clone().len() as u16;
        registry.name_to_id.insert(def.id.clone(), id);
        raw_defs.push(def);
        registry.defs = raw_defs.clone();
    }

    registry.plains = registry.get_or_plains("plains");
    commands.insert_resource(registry);
}
