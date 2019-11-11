// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity.

// Parity is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity.  If not, see <http://www.gnu.org/licenses/>.

#[macro_use(shards)]
extern crate reed_solomon_erasure;

use reed_solomon_erasure::galois_8::ReedSolomon;

pub fn encode (data: &[u8]) -> {
  let r = ReedSolomon::new(3, 2).unwrap(); // 3 data shards, 2 parity shards

  let mut master_copy = shards!(
      [0, 1,  2,  3],
      [4, 5,  6,  7],
      [8, 9, 10, 11],
      [0, 0,  0,  0], // last 2 rows are parity hards
      [0, 0,  0,  0]
  );

  // Construct the parity shards
  r.encode(&mut master_copy).unwrap();
}

pub fn decode() -> {

}

#[cfg(test)]
pub mod test {
  use super::*;

  #[test]
  fn test_encode() {
    let data = hex!("")

    encode()
  }

  #[test]
  fn test_decode() {

  }
}