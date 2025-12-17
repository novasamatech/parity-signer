// Copyright 2025 Security Research Labs GmbH
//
// SPDX-License-Identifier: Apache-2.0
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License./ DEALINGS IN THE SOFTWARE.

use parity_scale_codec::Decode;

fn main() {
    ziggy::fuzz!(|data: &[u8]| {
        let mut data = data;
        let Ok((ref mut metadata_proof, ref mut call)) =
            <(parser::MetadataProof, Vec<u8>)>::decode(&mut data)
        else {
            return;
        };
				#[cfg(not(feature = "fuzzing"))]
        println!("{:#?}", (&metadata_proof, &call));
        let _ = parser::decode_call(&mut call.as_slice(), &metadata_proof);
    });
}
