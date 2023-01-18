//
//  UIFont+Roboto.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 08/12/2022.
//

import SwiftUI

extension SwiftUI.Font {
    static var robotoMonoBold: SwiftUI.Font {
        FontFamily.RobotoMono.bold.swiftUIFont(size: 13)
    }

    /// Web3 font used to display networks logo as icons
    static var robotoMonoRegular: SwiftUI.Font {
        FontFamily.RobotoMono.regular.swiftUIFont(size: 13)
    }
}
