//
//  NewSeedBackupModal.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 25.12.2021.
//

import SwiftUI

struct NewSeedBackupModal: View {
    let content: MNewSeedBackup
    @State private var confirmBackup = false

    @EnvironmentObject var navigation: NavigationCoordinator

    var body: some View {
        VStack {
            NavigationBarView(
                viewModel: .init(
                    title: nil,
                    leftButton: .arrow
                )
            )
            VStack(alignment: .center, spacing: 0) {
                Localizable.NewSeed.Backup.Label.header.text
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .font(PrimaryFont.bodyL.font)
                    .multilineTextAlignment(.center)
                    .lineSpacing(Spacing.extraSmall)
            }
            .padding(.top, Spacing.extraSmall)
            .padding(.bottom, Spacing.medium)
            .padding(.horizontal, Spacing.medium)
            VStack(alignment: .leading, spacing: 0) {
                SeedPhraseView(viewModel: .init(seedPhrase: content.seedPhrase))
                    .padding(.bottom, Spacing.medium)
                Button(
                    action: {
                        confirmBackup.toggle()
                    },
                    label: {
                        HStack {
                            (confirmBackup ? Asset.checkboxChecked.swiftUIImage : Asset.checkboxEmpty.swiftUIImage)
                                .foregroundColor(Asset.accentPink300.swiftUIColor)
                            Localizable.iHaveWrittenDownMySeedPhrase.text
                                .multilineTextAlignment(.leading)
                                .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                            Spacer()
                        }
                    }
                )
                .padding(.vertical, Spacing.small)
                Spacer()
                PrimaryButton(
                    action: {
                        ServiceLocator.seedsMediator.restoreSeed(
                            seedName: content.seed,
                            seedPhrase: content.seedPhrase,
                            navigate: true
                        )
                    },
                    text: Localizable.NewSeed.Backup.Action.create.key,
                    style: .primary(isDisabled: .constant(!confirmBackup))
                )
                .padding(.vertical, Spacing.medium)
            }
            .padding(.horizontal, Spacing.medium)
        }
        .background(Asset.backgroundSecondary.swiftUIColor)
    }
}

// struct NewSeedBackupModal_Previews: PreviewProvider {
// static var previews: some View {
// NewSeedBackupModal()
// }
// }
