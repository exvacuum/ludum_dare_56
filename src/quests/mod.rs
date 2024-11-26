use core::panic;

use bevy::{ecs::system::SystemState, prelude::*, reflect::List, utils::HashMap};
use bevy_egui::egui::TextBuffer;

use crate::GameplaySet;

pub struct QuestsPlugin;

impl Plugin for QuestsPlugin {
    fn build(&self, app: &mut App) {
        let initial_state: SystemState<EventReader<QuestEvents>> =
            SystemState::new(app.world_mut());
        app.insert_resource(Quests(HashMap::from([(
            "ant_quest",
            Quest::new(|world| false, None, None),
        )])))
        .insert_resource(CachedSystemState {
            event_state: initial_state,
        })
        .add_event::<QuestEvents>()
        .add_systems(Update, update_quest_status.in_set(GameplaySet));
    }
}

#[derive(Resource)]
struct CachedSystemState {
    event_state: SystemState<EventReader<'static, 'static, QuestEvents>>,
}

#[derive(Resource, Deref, DerefMut)]
pub struct Quests(HashMap<&'static str, Quest>);

// Quest Events Enum
#[derive(Event)]
pub enum QuestEvents {
    StartQuest(String),
    CompleteQuest(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Quest {
    pub start: bool,
    pub complete: bool,
    pub setup: Option<fn(&mut World)>,
    pub teardown: Option<fn(&mut World)>,
    pub predicate: fn(&mut World) -> bool,
}

impl Quest {
    pub fn new(
        predicate: fn(&mut World) -> bool,
        setup: Option<fn(&mut World)>,
        teardown: Option<fn(&mut World)>,
    ) -> Self {
        Self {
            start: false,
            complete: false,
            predicate,
            setup,
            teardown,
        }
    }
}

pub fn update_quest_status(world: &mut World) {
    let mut quests = world.remove_resource::<Quests>().unwrap();
    let mut setup_fns = vec![];
    // Handle quest initiation
    world.resource_scope(|world, mut cached_state: Mut<CachedSystemState>| {
        let mut events = cached_state.event_state.get_mut(world);
        for event in events.read() {
            if let QuestEvents::StartQuest(quest_name) = event {
                if let Some(quest) = quests.get_mut(quest_name.as_str()) {
                    quest.start = true;
                    println!("Quest started: {}", quest_name);
                    if let Some(setup) = quest.setup {
                        setup_fns.push(setup);
                    }
                }
            }
        }
    });
    for setup in setup_fns.iter() {
        setup(world);
    }

    // Handle quest completion
    for (name, quest) in quests.iter_mut() {
        if !quest.complete && (quest.predicate)(world) && quest.start {
            quest.complete = true;
            info!("Quest complete: {}", name);
            if let Some(teardown) = quest.teardown {
                teardown(world);
            }
            world.send_event(QuestEvents::CompleteQuest(name.to_string()));
        }
    }
    world.insert_resource(quests);
}

// Read StartQuest event and start the quest
// pub fn start_quests(mut quests: ResMut<Quests>, mut ev_reader: EventReader<QuestEvents>) {
//     for event in ev_reader.read() {
//         match event {
//             QuestEvents::StartQuest(name) => {
//                 if let Some(quest) = quests.get_mut(name.as_str()) {
//                     quest.start = true;
//                     println!("Quest started: {}", name);
//                 }
//             }
//             _ => {}
//         }
//     }
// }

