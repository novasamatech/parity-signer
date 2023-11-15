//
//  FullscreenModal.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 03/05/2023.
//

import SwiftUI

struct FullscreenModal<ModalContent: View>: ViewModifier {
    @Binding var isPresented: Bool
    let onDismiss: () -> Void
    let modalContent: () -> ModalContent

    func body(content: Content) -> some View {
        content
            .fullScreenCover(isPresented: $isPresented, onDismiss: onDismiss, content: modalContent)
    }
}

extension View {
    func fullScreenModal(
        isPresented: Binding<Bool>,
        onDismiss: @escaping () -> Void = {},
        @ViewBuilder modalContent: @escaping () -> some View
    ) -> some View {
        modifier(FullscreenModal(
            isPresented: isPresented,
            onDismiss: onDismiss,
            modalContent: modalContent
        ))
    }
}
