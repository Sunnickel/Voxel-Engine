use crate::config::{BiomRegistry, BlockRegistry};
use crate::world::utils::Rgba;
use bevy::log::info;
use bevy::prelude::{Commands, Component};
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
    let temp_w = 2.0;
    let wet_w = 1.5;
    let height_w = 0.7;
    let cont_w = 1.2;

    (a.temperature - b.temperature).powi(2) * temp_w +
        (a.wetness - b.wetness).powi(2) * wet_w +
        (a.height - b.height).powi(2) * height_w +
        (a.continentalness - b.continentalness).powi(2) * cont_w
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

pub fn load_bioms(commands: &mut Commands, block_registry: &BlockRegistry) {
    info!("Registering bioms");
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
        registry.defs = raw_defs.clone();
        raw_defs.push(def);
    }

    registry.plains = registry.get_or_plains("plains");
    commands.insert_resource(registry);
}
