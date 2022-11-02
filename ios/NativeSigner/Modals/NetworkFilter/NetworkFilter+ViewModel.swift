//
//  NetworkFilter+ViewModel.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 31/10/2022.
//

import SwiftUI

extension NetworkFilterView {
    final class ViewModel: ObservableObject {
        private weak var appState: AppState!
        let allNetworks: [Network]
        @Published var animateBackground: Bool = false
        @Published var selectedNetworks: [Network] = []
        @Binding var isPresented: Bool

        init(
            allNetworks: [Network],
            isPresented: Binding<Bool>
        ) {
            self.allNetworks = allNetworks
            _isPresented = isPresented
        }

        func set(appState: AppState) {
            self.appState = appState
        }

        func loadCurrentSelection() {
            selectedNetworks = appState.userData.selectedNetworks
        }

        func resetAction() {
            selectedNetworks = appState.userData.selectedNetworks
            animateDismissal()
        }

        func doneAction() {
            appState.userData.selectedNetworks = selectedNetworks
            animateDismissal()
        }

        func isSelected(_ network: Network) -> Bool {
            selectedNetworks.contains(network)
        }

        func toggleSelection(_ network: Network) {
            if selectedNetworks.contains(network) {
                selectedNetworks.removeAll { $0 == network }
            } else {
                selectedNetworks.append(network)
            }
        }

        func animateDismissal() {
            Animations.chainAnimation(
                animateBackground.toggle(),
                delayedAnimationClosure: { self.isPresented.toggle() }
            )
        }
    }
}
