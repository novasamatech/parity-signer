//
//  ServiceLocator.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 09/09/2022.
//

import Foundation

/// We use this anti-pattern to work around some limitations of both SwiftUI and old architecture in Signer app
enum ServiceLocator {
    /// We store this in `ServiceLocator` as singleton, to be able to use it outside SwiftUI views which could use `@EnvironmentalObject`
    static var bottomSnackbarPresentation: BottomSnackbarPresentation = BottomSnackbarPresentation()
}
