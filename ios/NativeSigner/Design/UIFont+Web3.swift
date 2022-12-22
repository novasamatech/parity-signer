//
//  UIFont+Web3.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 08/12/2022.
//

import SwiftUI

extension SwiftUI.Font {
    /// Web3 font used to display networks logo as icons
    static var web3: SwiftUI.Font {
        FontFamily.Web3.regular.swiftUIFont(size: 16)
    }
}
