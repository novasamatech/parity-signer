//
//  AuthenticatedScreenContainer.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

import SwiftUI

struct AuthenticatedScreenContainer: View {
    @EnvironmentObject private var connectivityMediator: ConnectivityMediator
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var appState: AppState
    @StateObject var viewModel: ViewModel
    @State private var isShowingQRScanner: Bool = false

    var body: some View {
        ZStack {
            KeyDetailsView(viewModel: .init())
        }
        .animation(.default, value: AnimationDuration.standard)
        .fullScreenModal(
            isPresented: $navigation.genericError.isPresented
        ) {
            ErrorBottomModal(
                viewModel: .alertError(message: navigation.genericError.errorMessage),
                isShowingBottomAlert: $navigation.genericError.isPresented
            )
            .clearModalBackground()
        }
    }
}

extension AuthenticatedScreenContainer {
    final class ViewModel: ObservableObject {}
}
