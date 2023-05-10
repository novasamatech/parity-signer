//
//  BackupSelectKeyView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 02/02/2023.
//

import SwiftUI

struct BackupSelectKeyView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var connectivityMediator: ConnectivityMediator
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        VStack(spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: Localizable.Settings.SelectKey.title.string,
                    leftButtons: [.init(
                        type: .arrow,
                        action: { presentationMode.wrappedValue.dismiss() }
                    )],
                    rightButtons: [.init(type: .empty)],
                    backgroundColor: Asset.backgroundPrimary.swiftUIColor
                )
            )
            ScrollView {
                Localizable.Settings.SelectKey.header.text
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(PrimaryFont.captionM.font)
                    .padding(.horizontal, Spacing.large)
                    .padding(.vertical, Spacing.medium)
                LazyVStack(alignment: .leading, spacing: 0) {
                    ForEach(viewModel.seedsMediator.seedNames, id: \.self) {
                        seedNameView($0)
                            .padding(.bottom, Spacing.extraExtraSmall)
                            .padding(.horizontal, Spacing.extraSmall)
                    }
                }
            }
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
        .fullScreenModal(
            isPresented: $viewModel.isPresentingBackupModal,
            onDismiss: { viewModel.seedPhraseToPresent = .init(keyName: "", seedPhrase: .init(seedPhrase: "")) }
        ) {
            SettingsBackupModal(
                isShowingBackupModal: $viewModel.isPresentingBackupModal,
                viewModel: viewModel.seedPhraseToPresent
            )
            .clearModalBackground()
        }
        .fullScreenModal(
            isPresented: $viewModel.isPresentingConnectivityAlert
        ) {
            ErrorBottomModal(
                viewModel: connectivityMediator.isConnectivityOn ? .connectivityOn() : .connectivityWasOn(
                    continueAction: viewModel.onConnectivityWarningTap()
                ),
                isShowingBottomAlert: $viewModel.isPresentingConnectivityAlert
            )
            .clearModalBackground()
        }
        .onAppear {
            viewModel.use(connectivityMediator: connectivityMediator)
        }
    }

    @ViewBuilder
    func seedNameView(_ seedName: String) -> some View {
        HStack(alignment: .center) {
            Text(seedName)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.titleS.font)
            Spacer()
            Asset.chevronRight.swiftUIImage
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
        }
        .padding(.horizontal, Spacing.medium)
        .frame(height: Heights.settingsSelectKeyEntryHeight)
        .background(Asset.fill6.swiftUIColor)
        .cornerRadius(CornerRadius.small)
        .contentShape(Rectangle())
        .onTapGesture { viewModel.onSeedNameTap(seedName) }
    }
}

extension BackupSelectKeyView {
    final class ViewModel: ObservableObject {
        @Published var isPresentingBackupModal = false
        @Published var isPresentingConnectivityAlert = false
        @Published var seedPhraseToPresent: SettingsBackupViewModel = .init(
            keyName: "",
            seedPhrase: .init(seedPhrase: "")
        )
        private var awaitingSeedName: String?
        private weak var connectivityMediator: ConnectivityMediator!
        let seedsMediator: SeedsMediating
        private let warningStateMediator: WarningStateMediator

        init(
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
            warningStateMediator: WarningStateMediator = ServiceLocator.warningStateMediator
        ) {
            self.seedsMediator = seedsMediator
            self.warningStateMediator = warningStateMediator
        }

        func use(connectivityMediator: ConnectivityMediator) {
            self.connectivityMediator = connectivityMediator
        }

        func onSeedNameTap(_ seedName: String) {
            if connectivityMediator.isConnectivityOn || warningStateMediator.alert {
                isPresentingConnectivityAlert = true
                awaitingSeedName = seedName
            } else {
                presentBackupModal(seedName)
            }
        }

        func onConnectivityWarningTap() {
            warningStateMediator.resetConnectivityWarnings()
            isPresentingConnectivityAlert = false
            guard let awaitingSeedName else { return }
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.3) {
                self.presentBackupModal(awaitingSeedName)
                self.isPresentingBackupModal = true
            }
        }

        private func presentBackupModal(_ seedName: String) {
            seedPhraseToPresent = .init(
                keyName: seedName,
                seedPhrase: .init(
                    seedPhrase: seedsMediator.getSeedBackup(seedName: seedName)
                )
            )
            isPresentingBackupModal = true
            awaitingSeedName = nil
        }
    }
}

#if DEBUG
    struct BackupSelectKeyView_Previews: PreviewProvider {
        static var previews: some View {
            BackupSelectKeyView(
                viewModel: .init()
            )
        }
    }
#endif
