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
use std::io::{Read, Write};
use super::GunterStorage;

pub struct FileStorage;

impl FileStorage {
    pub fn new() -> Self {
        FileStorage {}
    }

    fn read_from_file(&self, filename: &str) -> u64 {
        let mut data = String::new();

        std::fs::File::open(filename)
            .expect("Failed to open file")
            .read_to_string(&mut data)
            .expect("Failed to read from file");

        data.parse::<u64>().expect("Data read is not a valid u64")
    }

    fn write_to_file(&self, filename: &str, timestamp: u64) {
        std::fs::File::create(filename)
            .expect("Failed to open file for writing")
            .write_all(format!("{}", timestamp).as_bytes())
            .expect("Failed to write to file")
    }
}

#[async_trait]
impl GunterStorage for FileStorage {
    async fn last_seen(&self) -> u64 {
        self.read_from_file("last-seen")
    }

    async fn save_last_seen(&self, timestamp: u64) {
        self.write_to_file("last-seen", timestamp)
    }

    async fn last_search(&self) -> u64 {
        self.read_from_file("last-seen-search")
    }

    async fn save_last_search(&self, timestamp: u64) {
        self.write_to_file("last-seen-search", timestamp)
    }
}