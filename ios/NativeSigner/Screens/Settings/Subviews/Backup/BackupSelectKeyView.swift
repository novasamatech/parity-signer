//
//  BackupSelectKeyView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 02/02/2023.
//

import SwiftUI

struct BackupSelectKeyView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var data: SignerDataModel

    var body: some View {
        VStack(spacing: 0) {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: Localizable.Settings.SelectKey.title.string,
                    leftButton: .arrow,
                    rightButton: .empty,
                    backgroundColor: Asset.backgroundSystem.swiftUIColor
                ),
                actionModel: .init(
                    leftBarMenuAction: viewModel.onBackTap
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
        @Published var seedPhraseToPresent: SettingsBackupViewModel = .init(
            keyName: "",
            seedPhrase: .init(seedPhrase: "")
        )

        let seedsMediator: SeedsMediating
        private weak var navigation: NavigationCoordinator!

        init(
            isPresented: Binding<Bool>,
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
        ) {
            _isPresented = isPresented
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

        func onBackTap() {
            isPresented = false
        }
    }
}

#if DEBUG
    struct BackupSelectKeyView_Previews: PreviewProvider {
        static var previews: some View {
            BackupSelectKeyView(
                viewModel: .init(isPresented: .constant(true))
            )
        }
    }
#endif
