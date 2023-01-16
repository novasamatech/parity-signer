//
//  ImportKeysAction.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 13/12/2022.
//

import SwiftUI

final class ImportKeysAction {
    private let databaseMediator: DatabaseMediating

    init(
        databaseMediator: DatabaseMediating = DatabaseMediator()
    ) {
        self.databaseMediator = databaseMediator
    }

    func importKeys(_ keys: [SeedKeysPreview]) {
        importDerivations(dbname: databaseMediator.databaseName, seeds: [:], seedDerivedKeys: keys)
    }
}
