//
//  ConnectivityAlertOverlay.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 28/12/2022.
//

import SwiftUI

struct ConnectivityAlertOverlay: View {
    @StateObject var viewModel: ViewModel

    var body: some View {
        VStack(alignment: .trailing) {
            if viewModel.isConnectivityAlertOn {
                HStack {
                    Spacer()
                    ConnectivityAlertButton(
                        action: viewModel.onTapAction,
                        isConnectivityOn: viewModel.connectivityMediator.isConnectivityOn
                    )
                    .padding(Spacing.medium)
                }
            } else {
                EmptyView()
            }
        }
        .fullScreenCover(
            isPresented: $viewModel.isPresentingConnectivityAlert
        ) {
            ErrorBottomModal(
                viewModel: viewModel.errorModalViewModel(),
                isShowingBottomAlert: $viewModel.isPresentingConnectivityAlert
            )
            .clearModalBackground()
        }
    }
}

extension ConnectivityAlertOverlay {
    final class ViewModel: ObservableObject {
        @Published var isPresentingConnectivityAlert = false
        @Published var isConnectivityAlertOn = false
        let connectivityMediator: ConnectivityMediator
        private let warningStateMediator: WarningStateMediator
        private let cancelBag = CancelBag()

        init(
            warningStateMediator: WarningStateMediator = ServiceLocator.warningStateMediator,
            connectivityMediator: ConnectivityMediator = ServiceLocator.connectivityMediator
        ) {
            self.warningStateMediator = warningStateMediator
            self.connectivityMediator = connectivityMediator
            listenToConnectivityUpdates()
        }

        func listenToConnectivityUpdates() {
            warningStateMediator.$alert.sink {
                self.isConnectivityAlertOn = $0
            }
            .store(in: cancelBag)
            connectivityMediator.$isConnectivityOn.sink {
                guard $0 else { return }
                self.isConnectivityAlertOn = $0
            }
            .store(in: cancelBag)
        }

        func errorModalViewModel() -> ErrorBottomModalViewModel {
            connectivityMediator.isConnectivityOn ?
                .connectivityOn() :
                // swiftformat:disable all
                .connectivityWasOn(continueAction: self.onTapContinueAction())
        }

        func onTapAction() {
            isPresentingConnectivityAlert = true
        }

        func onTapContinueAction() {
            warningStateMediator.resetConnectivityWarnings()
        }
    }
}
