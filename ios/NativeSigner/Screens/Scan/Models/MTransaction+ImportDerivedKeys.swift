//
//  MTransaction+ImportDerived.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 02/01/2023.
//

import Foundation

extension Array where Element == MTransaction {
    /// Informs whether there are any valid keys to be imported in `[MTransaction]` payload
    var hasImportableKeys: Bool {
        reduce(false) { $0 || $1.hasImportableKeys }
    }

    /// Extracts list of all `SeedKeysPreview` that are within given `[MTransaction]`
    var importableKeys: [SeedKeysPreview] {
        reduce(into: []) { $0 += $1.importableKeys }
    }
}

extension MTransaction {
    /// Informs whether there are any valid keys to be imported in `MTransaction` payload
    var hasImportableKeys: Bool {
        switch ttype {
        case .importDerivations:
            var hasValidKeys: Bool = false
            sortedValueCards().forEach {
                if case let .derivationsCard(keys) = $0.card {
                    hasValidKeys = keys
                        .reduce(hasValidKeys) { $0 || $1.derivedKeys.contains { $0.status == .importable }}
                }
            }
            return hasValidKeys
        default:
            return false
        }
    }

    /// Extracts list of all `SeedKeysPreview` that are within given `MTransaction`
    var importableKeys: [SeedKeysPreview] {
        switch ttype {
        case .importDerivations:
            var importableKeys: [SeedKeysPreview] = []
            sortedValueCards().forEach {
                if case let .derivationsCard(keys) = $0.card {
                    importableKeys.append(contentsOf: keys)
                }
            }
            return importableKeys
        default:
            return []
        }
    }
}
