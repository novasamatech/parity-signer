//
//  BackupModal.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 19/09/2022.
//

import SwiftUI

struct KeySummaryViewModel: Equatable {
    let keyName: String
    let base58: String
}

struct BackupModalViewModel: Equatable {
    let header: KeySummaryViewModel
    let derivedKeys: [DerivedKeyOverviewViewModel]
    let seedPhrase: SeedPhraseViewModel
}

struct BackupModal: View {
    @State private var animateBackground: Bool = false
    @Binding private var isShowingBackupModal: Bool
    @StateObject private var snackbar: BottomSnackbarPresentation = .init()
    private let viewModel: BackupModalViewModel

    init(
        isShowingBackupModal: Binding<Bool>,
        viewModel: BackupModalViewModel
    ) {
        _isShowingBackupModal = isShowingBackupModal
        self.viewModel = viewModel
    }

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                animateDismissal()
            },
            animateBackground: $animateBackground,
            ignoredEdges: .bottom,
            safeAreaInsetsMode: .full,
            content: {
                VStack(alignment: .center, spacing: 0) {
                    // Header with X button
                    HStack {
                        VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                            Text(viewModel.header.keyName)
                                .foregroundColor(.textAndIconsPrimary)
                                .font(PrimaryFont.titleS.font)
                            Text(viewModel.header.base58.truncateMiddle())
                                .foregroundColor(.textAndIconsSecondary)
                                .font(PrimaryFont.bodyM.font)
                        }
                        Spacer()
                        CloseModalButton(action: animateDismissal)
                    }
                    .padding(.leading, Spacing.large)
                    .padding(.trailing, Spacing.medium)
                    Divider()
                        .padding(.top, Spacing.medium)
                    ScrollView(showsIndicators: false) {
                        VStack(alignment: .center, spacing: Spacing.medium) {
                            // Seed phrase
                            HStack {
                                Localizable.BackupModal.Label.secret.text
                                    .foregroundColor(.textAndIconsPrimary)
                                    .font(PrimaryFont.bodyL.font)
                                Spacer()
                            }
                            SeedPhraseView(viewModel: .init(dataModel: viewModel.seedPhrase))
                            // Derived Keys
                            HStack {
                                Localizable.BackupModal.Label.derived.text
                                    .foregroundColor(.textAndIconsPrimary)
                                    .font(PrimaryFont.bodyL.font)
                                Spacer()
                            }
                            VStack(alignment: .leading, spacing: 0) {
                                ForEach(
                                    viewModel.derivedKeys,
                                    id: \.id
                                ) { derivedKey in
                                    DerivedKeyOverviewRow(derivedKey)
                                    if viewModel.derivedKeys.last != derivedKey {
                                        Divider()
                                    }
                                }
                            }
                            Spacer()
                                .frame(height: Spacing.componentSpacer)
                        }
                        .padding(.horizontal, Spacing.large)
                        .padding(.vertical, Spacing.medium)
                    }
                }
                .onAppear {
                    snackbar.viewModel = .init(
                        title: Localizable.BackupModal.Label.snackbar.string,
                        style: .info,
                        tapToDismiss: false,
                        countdown: .init(counter: 60, viewModel: .snackbarCountdown, onCompletion: animateDismissal)
                    )
                    snackbar.isSnackbarPresented = true
                }
                .bottomSnackbar(snackbar.viewModel, isPresented: $snackbar.isSnackbarPresented, autodismissCounter: 60)
            }
        )
    }

    private func animateDismissal() {
        Animations.chainAnimation(
            animateBackground.toggle(),
            delayedAnimationClosure: { isShowingBackupModal = false }()
        )
    }
}

#if DEBUG
    struct BackupModal_Previews: PreviewProvider {
        static var previews: some View {
            VStack {
                BackupModal(
                    isShowingBackupModal: Binding<Bool>.constant(true),
                    viewModel: .stub
                )
            }
            .previewLayout(.sizeThatFits)
            .preferredColorScheme(.dark)
        }
    }
#endif
