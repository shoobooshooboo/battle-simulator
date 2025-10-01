use bevy::prelude::*;

pub mod combatant;
use combatant::Combatant;

pub struct Battle{
    //controlled by player
    players: Vec<Box<dyn Combatant>>,
    //not controlled by player but on player-side
    allies: Vec<Box<dyn Combatant>>,
    //not controlled by player and not on player-side.
    enemies: Vec<Box<dyn Combatant>>,
}