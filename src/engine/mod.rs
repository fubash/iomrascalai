/************************************************************************
 *                                                                      *
 * Copyright 2014-2015 Thomas Poinsot, Urban Hafner                     *
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

pub use self::controller::EngineController;
pub use self::uct::UctEngine;
use board::Color;
use board::Move;
use config::Config;
use game::Game;
use patterns::Matcher;

use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

mod controller;
mod test;
mod uct;

pub fn factory(config: Arc<Config>, matcher: Arc<Matcher>) -> Box<Engine> {
    Box::new(UctEngine::new(config, matcher))
}

pub trait Engine: Send + Sync {

    fn gen_move(&mut self, Color, &Game, sender: Sender<(Move,usize)>, receiver: Receiver<()>);
    fn reset(&mut self) {}

}
