//
//  AuthenticatedScreenContainer.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 05/08/2022.
//

import Combine
import SwiftUI

struct AuthenticatedScreenContainer: View {
    @EnvironmentObject private var navigation: NavigationCoordinator
    @StateObject var viewModel: ViewModel

    var body: some View {
        switch viewModel.viewState {
        case let .keyDetails(initialKeyName):
            KeyDetailsView(
                viewModel: .init(
                    initialKeyName: initialKeyName,
                    onDeleteCompletion: viewModel.updateViewState
                )
            )
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
        case .noKeys:
            NoKeySetsView(viewModel: .init(onCompletion: viewModel.onKeySetAddCompletion(_:)))
        case .loading:
            EmptyView()
        }
    }
}

extension AuthenticatedScreenContainer {
    enum ViewState {
        case loading
        case keyDetails(String)
        case noKeys

        var isKeyDetails: Bool {
            switch self {
            case .keyDetails:
                true
            default:
                false
            }
        }
    }

    final class ViewModel: ObservableObject {
        @Published var viewState: ViewState = .loading
        @Published var isSnackbarPresented: Bool = false
        var snackbarViewModel: SnackbarViewModel = .init(title: "")
        private let seedsMediator: SeedsMediating
        private let cancelBag = CancelBag()

        init(seedsMediator: SeedsMediating = ServiceLocator.seedsMediator) {
            self.seedsMediator = seedsMediator
            updateViewState()

            // 	We monitor seed changes for keyDetails state only since
            // 	for onboarding onKeySetAddCompletion used
            seedsMediator.seedNamesPublisher
                .sink { [weak self] _ in
                    self?.updateKeyDetailsState()
                }.store(in: cancelBag)
        }

        func onKeySetAddCompletion(_ completionAction: CreateKeysForNetworksView.OnCompletionAction) {
            updateViewState()
            let message: String =
                switch completionAction {
                case let .createKeySet(seedName):
                    Localizable.CreateKeysForNetwork.Snackbar.keySetCreated(seedName)
                case let .recoveredKeySet(seedName):
                    Localizable.CreateKeysForNetwork.Snackbar.keySetRecovered(seedName)
                }
            snackbarViewModel = .init(
                title: message,
                style: .info
            )
            isSnackbarPresented = true
        }

        func updateViewState() {
            if let initialKeyName = seedsMediator.seedNames.first {
                viewState = .keyDetails(initialKeyName)
            } else {
                viewState = .noKeys
            }
        }

        func updateKeyDetailsState() {
            guard viewState.isKeyDetails else { return }

            updateViewState()
        }
    }
}
