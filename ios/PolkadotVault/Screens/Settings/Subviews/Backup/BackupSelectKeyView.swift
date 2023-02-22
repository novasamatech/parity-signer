//
//  BackupSelectKeyView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 02/02/2023.
//

import SwiftUI

struct BackupSelectKeyView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var data: SharedDataModel
    @EnvironmentObject private var connectivityMediator: ConnectivityMediator

    var body: some View {
        VStack(spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: Localizable.Settings.SelectKey.title.string,
                    leftButtons: [.init(
                        type: .arrow,
                        action: viewModel.onBackTap
                    )],
                    rightButtons: [.init(type: .empty)],
                    backgroundColor: Asset.backgroundSystem.swiftUIColor
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
        .fullScreenCover(
            isPresented: $viewModel.isPresentingBackupModal,
            onDismiss: { viewModel.seedPhraseToPresent = .init(keyName: "", seedPhrase: .init(seedPhrase: "")) }
        ) {
            SettingsBackupModal(
                isShowingBackupModal: $viewModel.isPresentingBackupModal,
                viewModel: viewModel.seedPhraseToPresent
            )
            .clearModalBackground()
        }
        .fullScreenCover(
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
            viewModel.use(data: data)
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
        @Binding var isPresented: Bool
        @Published var isPresentingBackupModal = false
        @Published var isPresentingConnectivityAlert = false
        @Published var seedPhraseToPresent: SettingsBackupViewModel = .init(
            keyName: "",
            seedPhrase: .init(seedPhrase: "")
        )
        private var awaitingSeedName: String?
        private weak var connectivityMediator: ConnectivityMediator!
        private weak var navigation: NavigationCoordinator!
        private weak var data: SharedDataModel!
        private let resetWarningAction: ResetConnectivtyWarningsAction
        let seedsMediator: SeedsMediating

        init(
            isPresented: Binding<Bool>,
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
            resetWarningAction: ResetConnectivtyWarningsAction
        ) {
            _isPresented = isPresented
            self.seedsMediator = seedsMediator
            self.resetWarningAction = resetWarningAction
        }

        func use(connectivityMediator: ConnectivityMediator) {
            self.connectivityMediator = connectivityMediator
        }

        func use(data: SharedDataModel) {
            self.data = data
        }

        func onSeedNameTap(_ seedName: String) {
            if connectivityMediator.isConnectivityOn || data.alert {
                isPresentingConnectivityAlert = true
                awaitingSeedName = seedName
            } else {
                presentBackupModal(seedName)
            }
        }

        func onBackTap() {
            isPresented = false
        }

        func onConnectivityWarningTap() {
            resetWarningAction.resetConnectivityWarnings()
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
                viewModel: .init(isPresented: .constant(true), resetWarningAction: .init(alert: .constant(false)))
            )
        }
    }
#endif
