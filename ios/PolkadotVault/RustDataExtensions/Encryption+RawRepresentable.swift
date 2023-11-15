//
//  Encryption+RawRepresentable.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 05/10/2022.
//

import Foundation

extension Encryption: RawRepresentable {
    /// Currently there is no native type bridging for enums available, this helps to keep possible enum values in sync
    /// while allowing of use of Swift's native `RawRepresentable` for that enum
    private enum EncryptionValues: String {
        case ecdsa
        case ed25519
        case sr25519
        case ethereum
    }

    public init?(rawValue: String) {
        switch rawValue {
        case EncryptionValues.ecdsa.rawValue:
            self = .ecdsa
        case EncryptionValues.ed25519.rawValue:
            self = .ed25519
        case EncryptionValues.sr25519.rawValue:
            self = .sr25519
        case EncryptionValues.ethereum.rawValue:
            self = .ethereum
        default:
            return nil
        }
    }

    public var rawValue: String {
        switch self {
        case .ecdsa:
            EncryptionValues.ecdsa.rawValue
        case .ed25519:
            EncryptionValues.ed25519.rawValue
        case .sr25519:
            EncryptionValues.sr25519.rawValue
        case .ethereum:
            EncryptionValues.ethereum.rawValue
        }
    }
}
