/************************************************************************
 *                                                                      *
 * Copyright 2014 Urban Hafner                                          *
 * Copyright 2015 Urban Hafner, Thomas Poinsot                          *
 *                                                                      *
 * This file is part of Iomrascálaí.                                    *
 *                                                                      *
 * Iomrascálaí is free software: you can redistribute it and/or modify  *
 * it under the terms of the GNU General Public License as published by *
 * the Free Software Foundation, either version 3 of the License, or    *
 * (at your option) any later version.                                  *
 *                                                                      *
 * Iomrascálaí is distributed in the hope that it will be useful,       *
 * but WITHOUT ANY WARRANTY; without even the implied warranty of       *
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the        *
 * GNU General Public License for more details.                         *
 *                                                                      *
 * You should have received a copy of the GNU General Public License    *
 * along with Iomrascálaí.  If not, see <http://www.gnu.org/licenses/>. *
 *                                                                      *
 ************************************************************************/

use board::Color;
use board::Move;
use board::Pass;

use std::collections::HashMap;

mod test;

pub struct MoveStats<'a> {
    color: Color,
    stats: HashMap<&'a Move, MoveStat>,
}

impl<'a> MoveStats<'a> {

    pub fn new(moves: &Vec<Move>, color: Color) -> MoveStats {
        let mut stats = HashMap::new();
        for m in moves.iter() {
            stats.insert(m, MoveStat::new());
        }
        MoveStats {
            color: color,
            stats: stats,
        }
    }

    pub fn record_win(&mut self, m: &Move) {
        let mut stat = self.stats.get_mut(&m).unwrap();
        stat.won();
    }

    pub fn record_loss(&mut self, m: &Move) {
        let mut stat = self.stats.get_mut(&m).unwrap();
        stat.lost();
    }

    pub fn all_losses(&self) -> bool {
        self.stats.values().all(|stat| stat.all_losses())
    }

    pub fn all_wins(&self) -> bool {
        self.stats.values().all(|stat| stat.all_wins())
    }

    pub fn best(&self) -> Move {
        let mut m = Pass(self.color);
        let mut move_stats = MoveStat::new();
        for (m_new, ms) in self.stats.iter() {
            if ms.win_ratio() > move_stats.win_ratio() {
                m = **m_new;
                move_stats = *ms;
            }
        }
        m
    }
}


#[derive(Copy)]
struct MoveStat {
    wins: usize,
    plays: usize
}

impl MoveStat {
    pub fn new() -> MoveStat {
        MoveStat { wins: 0, plays: 0 }
    }

    pub fn won(&mut self) {
        self.wins = self.wins + 1;
        self.plays = self.plays + 1;
    }

    pub fn lost(&mut self) {
        self.plays = self.plays + 1;
    }

    pub fn all_wins(&self) -> bool {
        self.wins == self.plays
    }

    pub fn all_losses(&self) -> bool {
        self.wins == 0
    }

    pub fn win_ratio(&self) -> f32 {
        if self.plays == 0 {
            0f32
        } else {
            (self.wins as f32) / (self.plays as f32)
        }
    }
}
