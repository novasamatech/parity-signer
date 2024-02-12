//
//  CreateDerivedKeyNameService.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 23/06/2023.
//

import Foundation

// sourcery: AutoMockable
protocol CreateDerivedKeyNameServicing: AnyObject {
    func defaultDerivedKeyName(_ keySet: MKeysNew, network: MmNetwork) -> String
}

final class CreateDerivedKeyNameService: CreateDerivedKeyNameServicing {
    func defaultDerivedKeyName(_ keySet: MKeysNew, network: MmNetwork) -> String {
        let currentPaths = keySet.set.map(\.key.address.path)
        let isPrimaryPathPresent = currentPaths.contains { $0 == network.pathId }
        if !isPrimaryPathPresent {
            return network.pathId
        }
        let numericPaths = currentPaths.compactMap {
            var trimmedValue = $0
            if let range = trimmedValue.range(of: "\(network.pathId)\(DerivationPathComponent.hard.description)"),
               range.lowerBound == trimmedValue.startIndex {
                trimmedValue.removeSubrange(range)
                return Int(trimmedValue)
            }
            return nil
        }.sorted()
        var lowestAvailableNumericPath = 0
        for path in numericPaths {
            if path != lowestAvailableNumericPath {
                break
            }
            lowestAvailableNumericPath += 1
        }
        return "\(network.pathId)\(DerivationPathComponent.hard.description)\(lowestAvailableNumericPath)"
    }
}
