//
//  ServiceLocator.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 09/09/2022.
//

import Foundation

/// We use this anti-pattern to work around some limitations of both SwiftUI and old architecture in the app
enum ServiceLocator {
    /// As long as we have `SharedDataModel` as tech debt, we need to have seeds mediator as singleton which is
    /// unfortunate but necessary for now; to be able to use it outside SwiftUI views it can't be `@EnvironmentalObject`
    static var seedsMediator: SeedsMediating = SeedsMediator()
    static var airgapMediator: AirgapMediating = AirgapMediatorAssembler().assemble()
    static var authenticationStateMediator: AuthenticatedStateMediator = .init()
    static var onboardingMediator: OnboardingMediating = OnboardingMediator()

    static var networkColorsGenerator = UnknownNetworkColorsGenerator()
    static var devicePasscodeAuthenticator: DevicePasscodeAuthenticatorProtocol = DevicePasscodeAuthenticator()
}
