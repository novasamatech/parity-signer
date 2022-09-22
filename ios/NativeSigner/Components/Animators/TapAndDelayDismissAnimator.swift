//
//  TapAndDelayDismissAnimator.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 09/09/2022.
//

import SwiftUI

struct TapAndDelayDismissAnimator: ViewModifier {
    @State private var timer = Timer
        .publish(every: 1, on: .main, in: .common)
        .autoconnect()

    @State var autodismissCounter: TimeInterval
    @State private var retainedAutodismissCounter: TimeInterval
    private let isTapToDismissActive: Bool
    @Binding var isPresented: Bool

    init(
        autodismissCounter: TimeInterval,
        isTapToDismissActive: Bool,
        isPresented: Binding<Bool>
    ) {
        retainedAutodismissCounter = autodismissCounter
        self.isTapToDismissActive = isTapToDismissActive
        self.autodismissCounter = autodismissCounter
        _isPresented = isPresented
    }

    func body(content: Content) -> some View {
        content
            .onTapGesture {
                guard isTapToDismissActive else { return }
                withAnimation {
                    self.isPresented = false
                }
            }
            .onReceive(timer) { _ in
                autodismissCounter -= 1
                if autodismissCounter <= 0 {
                    self.isPresented = false
                    stop()
                }
            }
            .onChange(
                of: isPresented,
                perform: {
                    $0 ? start() : stop()
                }
            )
    }

    private func stop() {
        timer.upstream.connect().cancel()
    }

    private func start() {
        autodismissCounter = retainedAutodismissCounter
        timer = Timer.publish(every: 1, on: .main, in: .common).autoconnect()
    }
}

extension View {
    /// Adds `tap to dismiss` behaviour and allows for automatic dismiss after given delay
    /// - Parameters:
    ///   - autodismissCounter: after how many seconds view should autodismiss. Default to 3
    ///   - isPresented: action controller in form of `Bool`
    /// - Returns: view that modifier is applied to
    func tapAndDelayDismiss(
        autodismissCounter: TimeInterval = 3,
        isTapToDismissActive: Bool = true,
        isPresented: Binding<Bool>
    ) -> some View {
        modifier(TapAndDelayDismissAnimator(
            autodismissCounter: autodismissCounter,
            isTapToDismissActive: isTapToDismissActive,
            isPresented: isPresented
        ))
    }
}
