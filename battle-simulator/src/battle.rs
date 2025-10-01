use bevy::prelude::*;

pub mod combatant;
use combatant::Combatant;

type CombatantList = Vec<Box<dyn Combatant>>;

pub struct Battle{
    //controlled by player
    players: CombatantList,
    //not controlled by player but on player-side
    allies: CombatantList,
    //not controlled by player and not on player-side.
    enemies: CombatantList,
}

impl Battle{
    pub fn new() -> Battle{}
    pub fn get_players(&self) -> &CombatantList{}
    pub fn get_players_mut(&mut self) -> &mut CombatantList{}
}