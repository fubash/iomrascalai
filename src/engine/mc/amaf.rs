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
use config::Config;
use game::Game;
use patterns::Matcher;
use playout::Playout;
use playout::PlayoutResult;
use super::Engine;
use super::McEngine;
use super::MoveStats;

use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

pub struct AmafMcEngine {
    config: Arc<Config>,
    playout: Arc<Playout>,
}

impl AmafMcEngine {

    pub fn new(config: Arc<Config>, matcher: Arc<Matcher>) -> AmafMcEngine {
        AmafMcEngine {
            config: config.clone(),
            playout: Arc::new(Playout::new(config.clone(), matcher))
        }
    }

}

impl Engine for AmafMcEngine {

    fn gen_move(&mut self, color: Color, game: &Game, sender: Sender<Move>, receiver: Receiver<()>) {
        super::gen_move::<AmafMcEngine>(self.config.clone(), self.playout.clone(), color, game, sender, receiver);
    }

    fn engine_type(&self) -> &'static str {
        "amaf"
    }

}

impl McEngine for AmafMcEngine {

    fn record_playout(stats: &mut MoveStats, playout: &PlayoutResult, won: bool) {
        for m in playout.moves().iter() {
            if won {
                stats.record_win(*m);
            } else {
                stats.record_loss(*m);
            }
        }
    }

}
