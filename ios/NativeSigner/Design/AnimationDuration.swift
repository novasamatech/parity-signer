//
//  AnimationDuration.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 24/08/2022.
//

import UIKit

/// Set of possible values for `Animation` duration or delay
enum AnimationDuration {
    /// Standard, 0.3, should align with most system animations
    static let standard: CGFloat = 0.3
    /// To be used with different `Overlay` presenrtation. 0.8, should allow for smooth transition
    static let overlayPresentation: CGFloat = 0.8
}
