// Copyright © 2022 Gustavo Noronha <gustavo@noronha.dev.br>
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU Affero General Public License as
//  published by the Free Software Foundation, either version 3 of the
//  License, or (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU Affero General Public License for more details.
//
//  You should have received a copy of the GNU Affero General Public License
//  along with this program.  If not, see <http://www.gnu.org/licenses/>.
use async_trait::async_trait;

mod aws_ssm;
mod file;

pub use aws_ssm::*;
pub use file::*;

#[async_trait]
pub trait GunterStorage {
    async fn last_seen(&self) -> u64;
    async fn save_last_seen(&self, timestamp: u64);

    async fn last_search(&self) -> u64;
    async fn save_last_search(&self, timestamp: u64);
}
