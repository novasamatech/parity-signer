//
//  ErrorDisplayed+Show.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 03/08/2022.
//

import Foundation

/// Maybe this could show errors?
extension ErrorDisplayed {
    func show() {
        guard case let .Str(payload) = self else { return }
        print(payload)
    }
}
