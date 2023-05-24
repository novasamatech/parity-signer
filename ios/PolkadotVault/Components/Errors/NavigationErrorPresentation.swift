//
//  NavigationErrorPresentation.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 5/10/2022.
//

import Foundation

final class NavigationErrorPresentation: ObservableObject {
    @Published var errorMessage: String = ""
    @Published var isPresented: Bool = false
}
