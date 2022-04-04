//
//  MRecoverSeedPhrase.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 16.12.2021.
//

import Foundation

struct MRecoverSeedPhrase: Decodable, Equatable {
    var keyboard: Bool
    var seed_name: String
    var user_input: String
    var guess_set: [String]
    var draft: [SeedWord]
    var ready_seed: String?
}

struct SeedWord: Decodable, Equatable {
    var order: Int
    var content: String
}

extension MRecoverSeedPhrase {
    func draftPhrase() -> String {
        return self.draft
            .sorted{word1, word2 in
                return word1.order < word2.order
            }.map{guessWord in
                return guessWord.content
            }.joined(separator: " ")
    }
}
