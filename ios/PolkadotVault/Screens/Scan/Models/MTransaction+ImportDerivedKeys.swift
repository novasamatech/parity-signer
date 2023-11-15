//
//  MTransaction+ImportDerivedKeys.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 02/01/2023.
//

import Foundation

extension [MTransaction] {
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

    /// Rust error state for import derived keys is different comparing to UI requirements,
    /// hence we need this support function to find out what is the proper UI error to show
    /// if there are no importable keys left
    var dominantImportError: DerivedKeyError? {
        guard !hasImportableKeys else { return nil }
        let importableKeys = allImportDerivedKeys

        let allErrors: [DerivedKeyError] = importableKeys
            .flatMap(\.derivedKeys)
            .compactMap {
                if case let .invalid(errors) = $0.status {
                    errors
                } else {
                    nil
                }
            }
            .flatMap { $0 }
        let mostCommonError = Dictionary(
            uniqueKeysWithValues: [.networkMissing, .keySetMissing, .badFormat]
                .map { error -> (DerivedKeyError, Int) in (
                    error,
                    allErrors.filter { $0 == error }.count
                ) }
        ).max(by: { $0.value < $1.value })

        return (mostCommonError?.value ?? 0) > 0 ? mostCommonError?.key : nil
    }
}

extension MTransaction {
    /// Informs whether there are any valid keys to be imported in `MTransaction` payload
    var hasImportableKeys: Bool {
        switch ttype {
        case .importDerivations:
            var hasImportableKeys: Bool = false
            sortedValueCards().forEach {
                if case let .derivationsCard(keys) = $0.card {
                    hasImportableKeys = keys
                        .reduce(hasImportableKeys) { $0 || $1.derivedKeys.contains { $0.status == .importable }}
                }
            }
            return hasImportableKeys
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
