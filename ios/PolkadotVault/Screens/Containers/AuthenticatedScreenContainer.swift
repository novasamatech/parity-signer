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

    var body: some View {
        ZStack {
            switch viewModel.viewState {
            case .keyDetails:
                KeyDetailsView(viewModel: .init(onDeleteCompletion: viewModel.updateViewState))
            case .noKeys:
                NoKeySetsView(viewModel: .init(onCompletion: viewModel.onKeySetAddCompletion(_:)))
            case .loading:
                EmptyView()
            }
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
        .bottomSnackbar(
            viewModel.snackbarViewModel,
            isPresented: $viewModel.isSnackbarPresented
        )
    }
}

extension AuthenticatedScreenContainer {
    enum ViewState {
        case loading
        case keyDetails
        case noKeys
    }

    final class ViewModel: ObservableObject {
        @Published var viewState: ViewState = .loading
        @Published var isSnackbarPresented: Bool = false
        var snackbarViewModel: SnackbarViewModel = .init(title: "")
        private let seedsMediator: SeedsMediating

        init(seedsMediator: SeedsMediating = ServiceLocator.seedsMediator) {
            self.seedsMediator = seedsMediator
            updateViewState()
        }

        func onKeySetAddCompletion(_ completionAction: CreateKeysForNetworksView.OnCompletionAction) {
            updateViewState()
            let message: String
            switch completionAction {
            case let .createKeySet(seedName):
                message = Localizable.CreateKeysForNetwork.Snackbar.keySetCreated(seedName)
            case let .recoveredKeySet(seedName):
                message = Localizable.CreateKeysForNetwork.Snackbar.keySetRecovered(seedName)
            }
            snackbarViewModel = .init(
                title: message,
                style: .info
            )
            isSnackbarPresented = true
        }

        func updateViewState() {
            viewState = seedsMediator.seedNames.isEmpty ? .noKeys : .keyDetails
        }
    }
}
