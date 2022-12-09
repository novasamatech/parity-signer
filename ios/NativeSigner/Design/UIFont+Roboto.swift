//
//  UIFont+Roboto.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 08/12/2022.
//

import SwiftUI

extension SwiftUI.Font {
    /// Web3 font used to display networks logo as icons
    static var robotoMono: SwiftUI.Font {
        FontFamily.RobotoMono.bold.swiftUIFont(size: 13)
    }
}
