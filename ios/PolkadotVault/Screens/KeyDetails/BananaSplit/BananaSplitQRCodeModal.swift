//
//  BananaSplitQRCodeModal.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 28/02/2024.
//

import Combine
import SwiftUI

struct BananaSplitQRCodeModalView: View {
    @StateObject var viewModel: ViewModel

    var body: some View {
        GeometryReader { geo in
            VStack(spacing: 0) {
                // Navigation bar
                NavigationBarView(
                    viewModel: .init(
                        leftButtons: [.init(type: .xmark, action: { viewModel.onCloseTap() })],
                        rightButtons: [.init(type: .more, action: viewModel.onMoreButtonTap)]
                    )
                )
                VStack(spacing: 0) {
                    // QR Code container
                    Spacer()
                    VStack(spacing: 0) {
                        AnimatedQRCodeView(
                            viewModel: Binding<AnimatedQRCodeViewModel>.constant(
                                .init(
                                    qrCodes: viewModel.bananaSplitBackup.qrCodes
                                )
                            )
                        )
                    }
                    .strokeContainerBackground()
                    // Info
                    AttributedInfoBoxView(text: Localizable.bananaSplitBackupQRCodeInfo())
                        .padding(.vertical, Spacing.extraSmall)
                    Spacer()
                    Spacer()
                    Spacer()
                }
                .padding(.horizontal, Spacing.large)
                .padding(.top, Spacing.extraSmall)
            }
            .frame(
                minWidth: geo.size.width,
                minHeight: geo.size.height
            )
            .background(.backgroundPrimary)
        }
        // Action sheet
        .fullScreenModal(
            isPresented: $viewModel.isPresentingActionSheet,
            onDismiss: {
                // iOS 15 handling of following .fullscreen presentation after dismissal, we need to dispatch this async
                DispatchQueue.main.async { viewModel.checkForActionsPresentation() }
            }
        ) {
            BananaSplitActionModal(
                viewModel: .init(
                    isPresented: $viewModel.isPresentingActionSheet,
                    shouldPresentDeleteBackupWarningModal: $viewModel.shouldPresentDeleteBackupWarningModal,
                    shouldPresentPassphraseModal: $viewModel.shouldPresentPassphraseModal
                )
            )
            .clearModalBackground()
        }
        // Passphrase
        .fullScreenModal(
            isPresented: $viewModel.isPresentingPassphraseModal
        ) {
            BananaSplitPassphraseModal(
                viewModel: .init(
                    seedName: viewModel.seedName,
                    isPresented: $viewModel.isPresentingPassphraseModal
                )
            )
            .clearModalBackground()
        }
        .fullScreenModal(isPresented: $viewModel.isPresentingDeleteBackupWarningModal) {
            HorizontalActionsBottomModal(
                viewModel: .bananaSplitDeleteBackup,
                mainAction: viewModel.onDeleteBackupTap(),
                isShowingBottomAlert: $viewModel.isPresentingDeleteBackupWarningModal
            )
            .clearModalBackground()
        }
    }
}

extension BananaSplitQRCodeModalView {
    enum OnCompletionAction: Equatable {
        case close
        case backupDeleted
    }

    final class ViewModel: ObservableObject {
        let seedName: String
        @Published var bananaSplitBackup: BananaSplitBackup
        @Published var isPresentingActionSheet = false
        @Published var isPresentingDeleteBackupWarningModal = false
        @Published var isPresentingPassphraseModal = false
        @Published var shouldPresentDeleteBackupWarningModal = false
        @Published var shouldPresentPassphraseModal = false
        @Published var isPresentingError: Bool = false
        @Published var presentableError: ErrorBottomModalViewModel = .alertError(message: "")
        private let bananaSplitMediator: KeychainBananaSplitAccessMediating
        private let onCompletion: (OnCompletionAction) -> Void

        init(
            seedName: String,
            bananaSplitBackup: BananaSplitBackup,
            bananaSplitMediator: KeychainBananaSplitAccessMediating = KeychainBananaSplitMediator(),
            onCompletion: @escaping (OnCompletionAction) -> Void
        ) {
            _bananaSplitBackup = .init(initialValue: bananaSplitBackup)
            self.seedName = seedName
            self.bananaSplitMediator = bananaSplitMediator
            self.onCompletion = onCompletion
        }

        func onMoreButtonTap() {
            isPresentingActionSheet = true
        }

        func onCloseTap() {
            onCompletion(.close)
        }

        func checkForActionsPresentation() {
            if shouldPresentPassphraseModal {
                shouldPresentPassphraseModal.toggle()
                isPresentingPassphraseModal = true
            }
            if shouldPresentDeleteBackupWarningModal {
                shouldPresentDeleteBackupWarningModal.toggle()
                isPresentingDeleteBackupWarningModal = true
            }
        }

        func onDeleteBackupTap() {
            switch bananaSplitMediator.removeBananaSplitBackup(seedName: seedName) {
            case .success:
                onCompletion(.backupDeleted)
            case let .failure(error):
                presentableError = .alertError(message: error.localizedDescription)
                isPresentingError = true
            }
        }
    }
}

#if DEBUG
    struct BananaSplitQRCodeModalView_Previews: PreviewProvider {
        static var previews: some View {
            Group {
                BananaSplitQRCodeModalView(
                    viewModel: .init(
                        seedName: "seed name",
                        bananaSplitBackup: .init(qrCodes: [[]]),
                        onCompletion: { _ in
                        }
                    )
                )
            }
            .previewLayout(.sizeThatFits)
            .preferredColorScheme(.dark)
            .environmentObject(ConnectivityMediator())
        }
    }
#endif
