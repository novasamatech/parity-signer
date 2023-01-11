//
//  ConnectivityAlertOverlay.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 28/12/2022.
//

import SwiftUI

struct ConnectivityAlertOverlay: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var connectivityMediator: ConnectivityMediator
    @EnvironmentObject private var data: SignerDataModel

    var body: some View {
        VStack(alignment: .trailing) {
            if viewModel.isConnectivityAlertOn {
                HStack {
                    Spacer()
                    ConnectivityAlertButton(
                        action: viewModel.onTapAction,
                        isConnectivityOn: connectivityMediator.isConnectivityOn
                    )
                    .padding(Spacing.medium)
                }
            } else {
                EmptyView()
            }
        }
        .onAppear {
            viewModel.use(data: data)
            viewModel.use(connectivityMediator: connectivityMediator)
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
        private weak var connectivityMediator: ConnectivityMediator!
        private weak var data: SignerDataModel!
        private let resetWarningAction: ResetConnectivtyWarningsAction
        private let cancelBag = CancelBag()

        init(resetWarningAction: ResetConnectivtyWarningsAction) {
            self.resetWarningAction = resetWarningAction
        }

        func use(data: SignerDataModel) {
            self.data = data
            data.$alert.sink {
                self.isConnectivityAlertOn = $0
            }.store(in: cancelBag)
        }

        func use(connectivityMediator: ConnectivityMediator) {
            self.connectivityMediator = connectivityMediator
            connectivityMediator.$isConnectivityOn.sink {
                guard $0 else { return }
                self.isConnectivityAlertOn = $0
            }.store(in: cancelBag)
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
            resetWarningAction.resetConnectivityWarnings()
        }
    }
}
