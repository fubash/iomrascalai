/************************************************************************
 *                                                                      *
 * Copyright 2015 Urban Hafner                                          *
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

use board::Board;
use board::Move;
use board::Pass;
use game::Game;

use core::ops::IndexMut;
use std::num::Float;

pub struct Node {
    children: Vec<Node>,
    game: Game,
    m: Option<Move>,
    plays: usize,
    wins: usize,
}

impl Node {

    pub fn new(game: Game, m: Move) -> Node {
        Node {
            children: vec!(),
            game: game,
            m: Some(m),
            plays: 0,
            wins: 0,
        }
    }

    pub fn root(game: &Game) -> Node {
        let mut root = Node {
            children: vec!(),
            game: game.clone(),
            m: None,
            plays: 0,
            wins: 0,
        };
        root.expand();
        root
    }

    pub fn expand(&mut self) {
        let pass = Pass(self.game.next_player());
        let new_game = self.game.play(pass).unwrap();
        self.children = vec!(Node::new(new_game, pass));
        for &m in self.game.legal_moves_without_eyes().iter() {
            let new_game = self.game.play(m).unwrap();
            self.children.push(Node::new(new_game, m));
        }
    }

    // Returns the index of the child to visit next according to the
    // UCT formula
    pub fn next_child_to_try_index(&self) -> usize {
        0
    }

    pub fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

    pub fn best(&self) -> &Node {
        let mut best = &self.children[0];
        for n in self.children.iter() {
            if n.plays > best.plays {
                best = n;
            }
        }
        best
    }

    pub fn child(&mut self, index: usize) -> &mut Node {
        self.children.index_mut(index)
    }

    pub fn record_win(&mut self) {
        self.wins += 1;
    }

    pub fn record_play(&mut self) {
        self.plays += 1;
    }

    pub fn win_ratio(&self) -> f32 {
        if self.plays == 0 {
            0f32
        } else {
            (self.wins as f32) / (self.plays as f32)
        }
    }

    pub fn board(&self) -> Board {
        self.game.board()
    }

    pub fn m(&self) -> Option<Move> {
        self.m
    }

    pub fn next_uct_child_index(&self) -> usize {
        let mut index = 0;
        for i in 0..self.children.len() {
            if self.children[i].uct_value(self.plays) > self.children[index].uct_value(self.plays) {
                index = i;
            }
        }
        index
    }

    fn uct_value(&self, parent_plays: usize) -> f32 {
        self.win_ratio() + self.c() * self.confidence(parent_plays)
    }

    fn confidence(&self, parent_plays: usize) -> f32 {
        // Is it log10 or loge?
        ((parent_plays as f32).log10()/(self.plays as f32)).sqrt()
    }

    fn c(&self) -> f32 {
        0.44 // sqrt(1/5)
    }
}
