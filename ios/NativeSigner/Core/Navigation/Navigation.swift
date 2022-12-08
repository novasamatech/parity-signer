//
//  Navigation.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

import Foundation

/// Model representing single instance of navigation within app
struct Navigation: Equatable {
    /// Action to be performed in Rust backend
    let action: Action
    /// Additional data to be send along with `action`
    let details: String
    /// Seed phrase required to be sent along with some `action`s
    let seedPhrase: String

    init(
        action: Action,
        details: String? = nil,
        seedPhrase: String? = nil
    ) {
        self.action = action
        self.details = details ?? ""
        self.seedPhrase = seedPhrase ?? ""
    }
}
