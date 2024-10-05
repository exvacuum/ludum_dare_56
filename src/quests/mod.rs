use core::panic;

use bevy::{prelude::*, utils::{HashMap, HashSet}};


pub struct QuestsPlugin;

impl Plugin for QuestsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Quests::default())
            .add_systems(Update, update_quest_status);
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct Quests(HashMap<&'static str, Quest>);


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Quest {
    pub complete: bool,
    pub predicate: fn(&mut World) -> bool,
}

impl Quest {
    pub fn new(predicate: fn(&mut World) -> bool) -> Self {
        Self {
            complete: false,
            predicate,
        }
    }
}

pub fn update_quest_status(world: &mut World) {
    if let Some(mut quests) = world.remove_resource::<Quests>() {
        for (name, quest) in quests.iter_mut() {
            if !quest.complete && (quest.predicate)(world) {
                quest.complete = true;
                println!("Quest complete: {}", name);
            }
        }
        world.insert_resource(quests);
    } else {
        panic!("Quests resource not found");
    }
}
