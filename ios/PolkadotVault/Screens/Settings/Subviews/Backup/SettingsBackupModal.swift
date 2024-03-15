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
    @StateObject var viewModel: ViewModel

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: viewModel.dismissModal,
            animateBackground: $viewModel.animateBackground,
            ignoredEdges: .bottom,
            safeAreaInsetsMode: .full,
            content: {
                VStack(alignment: .center, spacing: 0) {
                    // Header with X button
                    HStack {
                        VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                            Text(viewModel.backupViewModel.keyName)
                                .foregroundColor(.textAndIconsPrimary)
                                .font(PrimaryFont.titleS.font)
                        }
                        Spacer()
                        CircleButton(action: viewModel.dismissModal)
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
                .onAppear { viewModel.onAppear() }
                .bottomSnackbar(
                    viewModel.snackbar.viewModel,
                    isPresented: $viewModel.snackbar.isSnackbarPresented,
                    autodismissCounter: 60
                )
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
            SeedPhraseView(viewModel: .init(dataModel: viewModel.backupViewModel.seedPhrase))
            Spacer()
                .frame(height: Spacing.backupComponentSpacer)
        }
        .padding(.horizontal, Spacing.large)
        .padding(.vertical, Spacing.medium)
    }
}

extension SettingsBackupModal {
    final class ViewModel: ObservableObject {
        @Published var animateBackground: Bool = false
        @Published var snackbar: BottomSnackbarPresentation = .init()
        @Binding var isPresented: Bool
        let backupViewModel: SettingsBackupViewModel

        init(
            isPresented: Binding<Bool>,
            viewModel: SettingsBackupViewModel
        ) {
            _isPresented = isPresented
            backupViewModel = viewModel
        }

        func onAppear() {
            snackbar.viewModel = .init(
                title: Localizable.Settings.BackupModal.Label.snackbar.string,
                style: .info,
                tapToDismiss: false,
                countdown: .init(counter: 60, viewModel: .snackbarCountdown, onCompletion: dismissModal)
            )
            snackbar.isSnackbarPresented = true
        }

        func dismissModal() {
            Animations.chainAnimation(
                animateBackground.toggle(),
                // swiftformat:disable all
                delayedAnimationClosure: self.hide()
            )
        }

        private func hide() {
            isPresented = false
        }
    }
}

#if DEBUG
struct SettingsBackupModal_Previews: PreviewProvider {
    static var previews: some View {
        VStack {
            SettingsBackupModal(
                viewModel: .init(
                    isPresented: Binding<Bool>.constant(true),
                    viewModel: .stub
                )
            )
        }
        .previewLayout(.sizeThatFits)
        .preferredColorScheme(.dark)
    }
}
#endif
