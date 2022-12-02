//
//  Address+DisplayablePath.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 02/12/2022.
//

import SwiftUI

extension Address {
    /// Returns either `path` or if password protected, available path with path delimeter and lock icon
    var displayablePath: String {
        hasPwd ?
            "\(path)\(Localizable.Address.Label.PasswordProtectedPath.pathDelimeter.string)\(Image(.lock))" :
            path
    }
}
