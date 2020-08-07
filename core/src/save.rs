//! Handls data saving.

use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::io;
use std::num::NonZeroU32;
use std::path::Path;
use std::sync::atomic::{AtomicU32, Ordering};

use amethyst::ecs::{self, Entities, ReadStorage, WriteStorage};
use getset::*;
use serde::{Deserialize, Serialize};

use crate::{phy, units};

/// A resource for tracking global save count for the current run
#[derive(Debug)]
pub struct SaveCount {
    next: AtomicU32,
}

impl Default for SaveCount {
    fn default() -> Self {
        Self { next: 1.into() }
    }
}

impl SaveCount {
    /// Returns the next save count
    pub fn next(&self) -> u32 {
        self.next.fetch_add(1, Ordering::AcqRel)
    }
}

/// The persistent save ID of a `Saveable`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SaveId {
    generation: Option<NonZeroU32>, // `None` means created in current run, should be assigned a generation in saves
    index: u32,
}

impl SaveId {
    fn fill_generation(&mut self, generation: NonZeroU32) -> bool {
        if self.generation.is_none() {
            self.generation = Some(generation);
            true
        } else {
            false
        }
    }

    /// Generate a new SaveId
    pub fn generate(save_count: &SaveCount) -> Self {
        Self {
            generation: None,
            index: save_count.next(),
        }
    }
}

impl fmt::Display for SaveId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:08x}-{:08x}",
            self.generation.map_or(0, |i| i.get()),
            self.index
        )
    }
}

/// Component for entities with a unique ID.
///
/// `Saveable` entities are indexed in the `world.eviov` file.
/// Non-star bodies do not need to be saved if they are not referenced elsewhere.
///
/// Stars without the `Saveable` component will not be saved.
#[derive(Debug, Getters, Setters, MutGetters, CopyGetters)]
pub struct Saveable {
    /// The save ID of the entity
    #[getset(get = "pub", get_mut = "pub")]
    save_id: SaveId,
}

impl ecs::Component for Saveable {
    type Storage = ecs::storage::BTreeStorage<Self>;
}

/// The function that saved the world.
pub fn save_world(
    dir: impl AsRef<Path>,
    entities: &Entities<'_>,
    store_star: &ReadStorage<'_, phy::Star>,
    store_body: &ReadStorage<'_, phy::Body>,
    store_saveable: &mut WriteStorage<'_, Saveable>,
    t: units::GameInstant,
) -> io::Result<()> {
    use io::Seek;

    use ecs::Join;

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(dir.as_ref().join("world.eviov"))?;
    let _ = file.seek(io::SeekFrom::Start(0))?;
    let mut index: WorldIndex =
        rmp_serde::from_read(&mut file).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;

    index.generations += 1;
    let generation = NonZeroU32::new(index.generations).unwrap();

    let mut saves = vec![];
    for (entity, star, body, saveable) in
        (entities, store_star, store_body, &mut *store_saveable).join()
    {
        let _ = saveable.save_id.fill_generation(generation);
        let star_id = saveable.save_id_mut().clone();
        if let phy::Body::Root(_) = body {
            index.roots.insert(star_id.clone());
        }
        saves.push((entity, star_id));
    }

    for (entity, star_id) in saves {
        let star_assoc = index.stars.entry(star_id.clone()).or_default();
        save_star(
            dir.as_ref().join(&format!("{}.star", star_id)),
            store_star.get(entity).expect("just retrieved"),
            store_body,
            &mut *store_saveable,
            |id| {
                if id.fill_generation(generation) {
                    star_assoc.push(id.clone());
                }
            },
            t,
        )?;
    }

    let _ = file.seek(io::SeekFrom::Start(0))?;
    rmp_serde::encode::write_named(&mut file, &index)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    Ok(())
}

/// Writes the state of a star system to a file.
pub fn save_star(
    file: impl AsRef<Path>,
    star: &phy::Star,
    store_body: &ReadStorage<'_, phy::Body>,
    store_saveable: &mut WriteStorage<'_, Saveable>,
    fix_id: impl FnMut(&mut SaveId),
    t: units::GameInstant,
) -> io::Result<()> {
    let mut file = fs::File::create(file)?;

    #[derive(Serialize)]
    struct StarSer<I: Iterator<Item = BodySer> + Clone> {
        field_radius: units::Length,
        strength: units::Mass,
        #[serde(with = "serde_iter::seq")]
        children: I,
    }

    #[derive(Serialize)]
    struct BodySer {
        position: units::Position,
        velocity: units::Velocity,
        standing: bool,
    }

    fn body_ser(body: &phy::Body, star: &phy::Star, t: units::GameInstant) -> BodySer {
        BodySer {
            position: body.position(t),
            velocity: body.velocity(t, star.strength()),
            standing: matches!(body, phy::Body::Standing(_)),
        }
    }

    let children = star.index().all().map(|child| {
        body_ser(
            store_body
                .get(child)
                .expect("Star child without a Body component"),
            star,
            t,
        )
    });

    rmp_serde::encode::write_named(
        &mut file,
        &StarSer {
            field_radius: star.field_radius(),
            strength: star.strength(),
            children: serde_iter::CloneOnce::from(children),
        },
    )
    .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct WorldIndex {
    generations: u32,
    roots: HashSet<SaveId>,
    stars: HashMap<SaveId, Vec<SaveId>>,
}
