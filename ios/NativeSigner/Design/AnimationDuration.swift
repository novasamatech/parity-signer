//
//  AnimationDuration.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 24/08/2022.
//

import UIKit

/// Set of possible values for `Animation` duration or delay
enum AnimationDuration {
    /// Short, 0.15
    static let short: CGFloat = 0.15
    /// Standard, 0.3, should align with most system animations
    static let standard: CGFloat = 0.3
    /// To be used with different `Overlay` presenrtation. 0.4, should allow for smooth transition
    static let overlayPresentation: CGFloat = 0.4
}
