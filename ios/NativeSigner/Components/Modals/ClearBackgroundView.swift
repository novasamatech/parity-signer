//
//  ClearBackgroundView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 25/08/2022.
//

import SwiftUI

/// Workaround for current limitations in `SwiftUI` as to modal sheet presentation
/// Currently there is no system way to present modal with transparent / semi transparent background
struct ClearBackgroundView: UIViewRepresentable {
    func makeUIView(context _: Context) -> some UIView {
        let view = UIView()
        DispatchQueue.main.async {
            view.superview?.superview?.backgroundColor = .clear
        }
        return view
    }

    func updateUIView(_: UIViewType, context _: Context) {}
}

struct ClearBackgroundViewModifier: ViewModifier {
    func body(content: Content) -> some View {
        content
            .background(ClearBackgroundView())
    }
}

extension View {
    func clearModalBackground() -> some View {
        modifier(ClearBackgroundViewModifier())
    }
}
