//
//  HiddenTextEditorBackgroundModifier.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 18/01/2023.
//

import SwiftUI

struct HiddenTextEditorBackgroundModifier: ViewModifier {
    func body(content: Content) -> some View {
        if #available(iOS 16.0, *) {
            content
                .scrollContentBackground(.hidden)
        } else {
            content
                .onAppear {
                    UITextView.appearance().backgroundColor = .clear
                }
        }
    }
}

extension View {
    func hiddenTextEditorBackground() -> some View {
        modifier(HiddenTextEditorBackgroundModifier())
    }
}
