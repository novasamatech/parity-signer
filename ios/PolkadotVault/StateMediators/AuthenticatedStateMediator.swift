//
//  AuthenticatedStateMediator.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 13/03/2023.
//

import Foundation

final class AuthenticatedStateMediator: ObservableObject {
    @Published var authenticated: Bool = false
}
