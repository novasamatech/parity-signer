//
//  MTransaction+ImportDerivedKeys.swift
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
    var allImportDerivedKeys: [SeedKeysPreview] {
        reduce(into: []) { $0 += $1.allImportDerivedKeys }
    }

    var importableKeysCount: Int {
        reduce(0) { $0 + $1.importableKeysCount }
    }

    /// Extracts list of importable `SeedKeysPreview` that are within given `[MTransaction]`
    var importableSeedKeysPreviews: [SeedKeysPreview] {
        reduce(into: []) { $0 += $1.importableSeedKeysPreviews }
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
    var allImportDerivedKeys: [SeedKeysPreview] {
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

    var importableKeysCount: Int {
        switch ttype {
        case .importDerivations:
            var importableKeysCount = 0
            sortedValueCards().forEach {
                if case let .derivationsCard(keys) = $0.card {
                    importableKeysCount += keys.reduce(0) { $0 + $1.importableKeysCount }
                }
            }
            return importableKeysCount
        default:
            return 0
        }
    }

    /// Extracts list of importable `SeedKeysPreview` that are within given `MTransaction`
    var importableSeedKeysPreviews: [SeedKeysPreview] {
        switch ttype {
        case .importDerivations:
            var importableSeedKeysPreviews: [SeedKeysPreview] = []
            sortedValueCards().forEach {
                if case let .derivationsCard(keys) = $0.card {
                    importableSeedKeysPreviews += keys.filter(\.isImportable)
                }
            }
            return importableSeedKeysPreviews
        default:
            return []
        }
    }
}
