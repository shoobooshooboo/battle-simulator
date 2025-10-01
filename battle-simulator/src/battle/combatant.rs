use crate::battle::Battle;

pub struct Timer{
    current: f32,
    max: f32,
}

impl Timer{
    fn new(max: f32) -> Self{
        Self { 
            current: max,
            max
        }
    }

    fn reset(&mut self){
        self.current = self.max;
    }

    fn change_max(&mut self, max: f32){
        self.max = max;
    }

    fn reduce(&mut self, dt: f32){
        self.current = (self.current - dt).max(0.0);
    }

    fn current(&self) -> f32{
        self.current
    }
}

pub trait Combatant{
    fn get_timer(&self) -> &Timer;
    fn get_timer_mut(&mut self) -> &mut Timer;
    fn take_turn(&mut self, battle: &mut Battle); 
    fn take_damage(&mut self, dmg: f32);
    fn is_alive(&self) -> bool;
}