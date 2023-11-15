//
//  BottoEdgeOverlay.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 09/09/2022.
//

import SwiftUI

extension AnyTransition {
    static var moveFromBottomAndFade: AnyTransition {
        .asymmetric(
            insertion: .move(edge: .bottom).combined(with: .opacity),
            removal: .move(edge: .bottom)
        )
    }
}

struct BottomEdgeOverlay<T: View>: ViewModifier {
    @Binding var isPresented: Bool
    let overlayView: T

    func body(content: Content) -> some View {
        ZStack(alignment: .bottom) {
            content
            if isPresented {
                overlayView
                    .zIndex(1) // This fixes SwiftUI issue of dismiss animation on Z-axis
                    .transition(.moveFromBottomAndFade)
            }
        }
        .animation(.easeInOut(duration: AnimationDuration.overlayPresentation), value: isPresented)
    }
}

extension View {
    /// Presents given `overlayView` over bottom edge with opacity transition. Dismiss view with bottom edge transition
    /// - Parameters:
    ///   - overlayView: view to be presented as overlay
    ///   - isPresented: action controller in form of `Bool`
    /// - Returns: view that modifier is applied to
    func bottomEdgeOverlay(overlayView: some View, isPresented: Binding<Bool>) -> some View {
        modifier(BottomEdgeOverlay(isPresented: isPresented, overlayView: overlayView))
    }
}
