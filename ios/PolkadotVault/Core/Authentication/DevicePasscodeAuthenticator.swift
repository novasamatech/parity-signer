//
//  DevicePasscodeAuthenticator.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 28/12/2023.
//

import Foundation
import LocalAuthentication

// sourcery: AutoMockable
protocol DevicePasscodeAuthenticatorProtocol {
    /// This methods trigger passcode screen to authenticate user, if failed, will bring back "Unlock app" screen
    ///
    /// As we can't use `LAContext` to properly evaluate user authentication, as this would trigger biometry check
    /// if set up. Hence we need to rely on fetching some seed from Keychain and with assumption that authentication
    /// is used only when some seed is present this satisfies our requirements
    /// - Returns: whether user is authenticated or not
    func authenticateUser() -> Bool
}

final class DevicePasscodeAuthenticator: DevicePasscodeAuthenticatorProtocol {
    private let seedsMediator: SeedsMediating

    init(
        seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
    ) {
        self.seedsMediator = seedsMediator
    }

    func authenticateUser() -> Bool {
        guard let seedName = seedsMediator.seedNames.first else { return true }
        return !seedsMediator.getSeed(seedName: seedName).isEmpty
    }
}
