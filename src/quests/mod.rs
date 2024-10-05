use core::panic;

use bevy::{prelude::*, utils::HashMap};

pub struct QuestsPlugin;

impl Plugin for QuestsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Quests::default())
            .add_event::<QuestEvents>()
            .add_systems(Update, (start_quests, update_quest_status));
    }
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct Quests(HashMap<&'static str, Quest>);

// Quest Events Enum
#[derive(Event)]
pub enum QuestEvents {
    StartQuest(&'static str),
    CompleteQuest(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Quest {
    pub start: bool,
    pub complete: bool,
    pub predicate: fn(&mut World) -> bool,
}

impl Quest {
    pub fn new(predicate: fn(&mut World) -> bool) -> Self {
        Self {
            start: false,
            complete: false,
            predicate,
        }
    }
}

pub fn update_quest_status(world: &mut World) {
    if let Some(mut quests) = world.remove_resource::<Quests>() {
        for (name, quest) in quests.iter_mut() {
            if !quest.complete && (quest.predicate)(world) && quest.start {
                quest.complete = true;
                println!("Quest complete: {}", name);
                world.send_event(QuestEvents::CompleteQuest(name));
            }
        }
        world.insert_resource(quests);
    } else {
        panic!("Quests resource not found");
    }
}

// Read StartQuest event and start the quest
pub fn start_quests(mut quests: ResMut<Quests>, mut ev_reader: EventReader<QuestEvents>) {
    for event in ev_reader.read() {
        match event {
            QuestEvents::StartQuest(name) => {
                if let Some(quest) = quests.get_mut(name) {
                    quest.start = true;
                    println!("Quest started: {}", name);
                }
            }
            _ => {}
        }
    }
}