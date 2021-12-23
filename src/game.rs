#[derive(Clone, Copy)]
pub enum Choice {
    X,
    Y,
}

#[derive(Clone, Copy)]
pub struct Actor {
    pub choice: Choice,
    pub money: i32,
}

impl Default for Actor {
    fn default() -> Self {
        Self {
            choice: Choice::X,
            money: 200,
        }
    }
}

#[derive(Clone)]
pub struct Bank(pub [Actor; 4]);

impl Bank {
    pub fn payout(&mut self) {
        let (mut count_x, mut count_y) = (0, 0);

        for actor in &self.0 {
            match actor.choice {
                Choice::X => count_x += 1,
                Choice::Y => count_y += 1,
            }
        }

        let (pay_x, pay_y) = match (count_x, count_y) {
            (4, 0) => (-10, -10),
            (3, 1) => ( 10, -30),
            (2, 2) => ( 20, -20),
            (1, 3) => ( 30, -10),
            (0, 4) => ( 10,  10),
            _ => unreachable!()
        };

        for actor in &mut self.0 {
            actor.money += match actor.choice {
                Choice::X => pay_x,
                Choice::Y => pay_y,
            }
        }
    }
}

impl Default for Bank {
    fn default() -> Self {
        Self([Actor::default(); 4])
    }
}