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

use codec::Encode;

#[derive(serde::Serialize, serde::Deserialize, autarkie::Grammar, Debug, Clone)]
pub struct FuzzMetadataProof {
    pub proof: grammar_fuzzer::merkle_tree::FuzzProof,
    pub extrinsic: grammar_fuzzer::types::FuzzExtrinsicMetadata,
    pub extra_info: grammar_fuzzer::ExtraInfo,
    pub call: Vec<u8>,
}

#[derive(Encode, Debug, Clone)]
pub struct MetadataProof {
    pub proof: grammar_fuzzer::Proof,
    pub extrinsic: grammar_fuzzer::types::ExtrinsicMetadata,
    pub extra_info: grammar_fuzzer::ExtraInfo,
    pub call: Vec<u8>,
}
autarkie::fuzz_afl!(FuzzMetadataProof, |data: &FuzzMetadataProof| -> Vec<u8> {
    let converted = MetadataProof {
        proof: data.proof.clone().into(),
        extrinsic: data.extrinsic.clone().into(),
        extra_info: data.extra_info.clone(),
        call: data.call.clone(),
    };
    converted.encode()
});
