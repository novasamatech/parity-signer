//
//  Animations.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 07/09/2022.
//

import SwiftUI

enum Animations {
    /// Convinience method that enables to chain two animations with standard duration and delay.
    /// Uses `easeIn` for animation curve
    /// - Parameters:
    ///   - firstAnimationClosure: closure to be called when first animation is finished
    ///   - delayedAnimationClosure: closure to be called when second, delayed animation is finished
    static func chainAnimation(
        _ firstAnimationClosure: @autoclosure () -> Void,
        delayedAnimationClosure: @escaping @autoclosure () -> Void
    ) {
        withAnimation(
            Animation.easeIn(duration: AnimationDuration.standard)
        ) {
            firstAnimationClosure()
        }
        DispatchQueue.main.asyncAfter(deadline: .now() + AnimationDuration.standard) {
            withAnimation(
                Animation.easeIn(duration: AnimationDuration.standard)
            ) {
                delayedAnimationClosure()
            }
        }
    }
}
