//
//  AccessControlProvider.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 25/08/2022.
//

import Security

// sourcery: AutoMockable
/// Protocol for generating access control when accessing Keychain
protocol AccessControlProviding: AnyObject {
    /// Creates access control
    /// - Returns: `SecAccessControl` with default security parameters
    func accessControl() throws -> SecAccessControl
}

/// Class to be used for access flags in production environment
///
/// Due to `SecAccessControl` having no public properties, we are not able to provide unit tests
final class AccessControlProvider: AccessControlProviding {
    func accessControl() throws -> SecAccessControl {
        if let accessControl = SecAccessControlCreateWithFlags(
            kCFAllocatorDefault,
            kSecAttrAccessibleWhenPasscodeSetThisDeviceOnly,
            .devicePasscode,
            nil
        ) {
            return accessControl
        } else {
            throw KeychainError.accessControlNotAvailable
        }
    }
}

/// Class to be used only when working in dev environment
final class SimulatorAccessControlProvider: AccessControlProviding {
    func accessControl() throws -> SecAccessControl {
        if let accessControl = SecAccessControlCreateWithFlags(
            kCFAllocatorDefault,
            kSecAttrAccessibleWhenUnlockedThisDeviceOnly,
            [],
            nil
        ) {
            return accessControl
        } else {
            throw KeychainError.accessControlNotAvailable
        }
    }
}
