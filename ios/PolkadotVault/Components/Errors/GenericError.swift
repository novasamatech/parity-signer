//
//  GenericError.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 5/10/2022.
//

import Foundation

final class GenericErrorViewModel: ObservableObject {
    @Published var errorMessage: String = ""
    @Published var isPresented: Bool = false
}
