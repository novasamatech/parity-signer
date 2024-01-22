//
//  BackupSelectKeyView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 02/02/2023.
//

import SwiftUI

struct BackupSelectKeyView: View {
    @StateObject var viewModel: ViewModel
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        VStack(spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: .title(Localizable.Settings.SelectKey.title.string),
                    leftButtons: [.init(
                        type: .arrow,
                        action: { presentationMode.wrappedValue.dismiss() }
                    )],
                    rightButtons: [.init(type: .empty)],
                    backgroundColor: .backgroundPrimary
                )
            )
            ScrollView {
                Localizable.Settings.SelectKey.header.text
                    .foregroundColor(.textAndIconsPrimary)
                    .font(PrimaryFont.titleL.font)
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
        .background(.backgroundPrimary)
        .fullScreenModal(
            isPresented: $viewModel.isPresentingBackupModal,
            onDismiss: { viewModel.seedPhraseToPresent = .init(keyName: "", seedPhrase: .init(seedPhrase: "")) }
        ) {
            SettingsBackupModal(
                viewModel: .init(
                    isPresented: $viewModel.isPresentingBackupModal,
                    viewModel: viewModel.seedPhraseToPresent
                )
            )
            .clearModalBackground()
        }
    }

    @ViewBuilder
    func seedNameView(_ seedName: String) -> some View {
        HStack(alignment: .center) {
            Text(seedName)
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.titleS.font)
            Spacer()
            Image(.chevronRight)
                .foregroundColor(.textAndIconsTertiary)
        }
        .padding(.horizontal, Spacing.medium)
        .frame(height: Heights.settingsSelectKeyEntryHeight)
        .background(.fill6)
        .cornerRadius(CornerRadius.small)
        .contentShape(Rectangle())
        .onTapGesture { viewModel.onSeedNameTap(seedName) }
    }
}

extension BackupSelectKeyView {
    final class ViewModel: ObservableObject {
        @Published var isPresentingBackupModal = false
        @Published var seedPhraseToPresent: SettingsBackupViewModel = .init(
            keyName: "",
            seedPhrase: .init(seedPhrase: "")
        )
        let seedsMediator: SeedsMediating

        init(
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
        ) {
            self.seedsMediator = seedsMediator
        }

        func onSeedNameTap(_ seedName: String) {
            seedPhraseToPresent = .init(
                keyName: seedName,
                seedPhrase: .init(
                    seedPhrase: seedsMediator.getSeedBackup(seedName: seedName)
                )
            )
            isPresentingBackupModal = true
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
