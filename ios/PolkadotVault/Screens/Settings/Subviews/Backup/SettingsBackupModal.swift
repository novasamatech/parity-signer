//
//  SettingsBackupModal.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 01/02/2023.
//

import SwiftUI

struct SettingsBackupViewModel: Equatable {
    let keyName: String
    let seedPhrase: SeedPhraseViewModel
}

struct SettingsBackupModal: View {
    @State private var animateBackground: Bool = false
    @Binding private var isShowingBackupModal: Bool
    @StateObject private var snackbar: BottomSnackbarPresentation = .init()
    private let viewModel: SettingsBackupViewModel

    init(
        isShowingBackupModal: Binding<Bool>,
        viewModel: SettingsBackupViewModel
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
                            Text(viewModel.keyName)
                                .foregroundColor(.textAndIconsPrimary)
                                .font(PrimaryFont.titleS.font)
                        }
                        Spacer()
                        CloseModalButton(action: animateDismissal)
                    }
                    .padding(.leading, Spacing.large)
                    .padding(.trailing, Spacing.medium)
                    Divider()
                        .padding(.top, Spacing.medium)
                    switch UIScreen.main.bounds.width {
                    case DeviceConstants.compactDeviceWidth:
                        ScrollView(showsIndicators: false) {
                            seedPhraseContent()
                        }
                    default:
                        seedPhraseContent()
                    }
                }
                .onAppear {
                    snackbar.viewModel = .init(
                        title: Localizable.Settings.BackupModal.Label.snackbar.string,
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

    @ViewBuilder
    func seedPhraseContent() -> some View {
        VStack(alignment: .center, spacing: Spacing.medium) {
            // Seed phrase
            HStack {
                Localizable.Settings.BackupModal.Label.header.text
                    .foregroundColor(.textAndIconsPrimary)
                    .font(PrimaryFont.bodyL.font)
                Spacer()
            }
            SeedPhraseView(viewModel: .init(dataModel: viewModel.seedPhrase))
            Spacer()
                .frame(height: Spacing.backupComponentSpacer)
        }
        .padding(.horizontal, Spacing.large)
        .padding(.vertical, Spacing.medium)
    }

    private func animateDismissal() {
        Animations.chainAnimation(
            animateBackground.toggle(),
            delayedAnimationClosure: { isShowingBackupModal = false }()
        )
    }
}

#if DEBUG
    struct SettingsBackupModal_Previews: PreviewProvider {
        static var previews: some View {
            VStack {
                SettingsBackupModal(
                    isShowingBackupModal: Binding<Bool>.constant(true),
                    viewModel: .stub
                )
            }
            .previewLayout(.sizeThatFits)
            .preferredColorScheme(.dark)
        }
    }
#endif
