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
        ZStack {
            RoundedRectangle(cornerRadius: 8).foregroundColor(Asset.backgroundSecondary.swiftUIColor)
            VStack {
                HeaderBar(line1: Localizable.backupSeedPhrase.key, line2: LocalizedStringKey(content.seed))
                SeedPhraseView(viewModel: .init(seedPhrase: content.seedPhrase))
                VStack(spacing: 16) {
                    Button(
                        action: {
                            confirmBackup.toggle()
                        },
                        label: {
                            HStack {
                                (confirmBackup ? Image(.checkmark, variant: .square) : Image(.square))
                                    .imageScale(.large)
                                Localizable.iHaveWrittenDownMySeedPhrase.text
                                    .multilineTextAlignment(.leading)
                                    .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                                Spacer()
                            }
                        }
                    )
                    Spacer()
                    PrimaryButton(
                        action: {
                            ServiceLocator.seedsMediator.restoreSeed(
                                seedName: content.seed,
                                seedPhrase: content.seedPhrase,
                                navigate: true
                            )
                        },
                        text: Localizable.next.key,
                        style: .primary(isDisabled: .constant(!confirmBackup))
                    )
                    .padding(.vertical, 16.0)
                }
            }.padding(16)
        }
    }
}

// struct NewSeedBackupModal_Previews: PreviewProvider {
// static var previews: some View {
// NewSeedBackupModal()
// }
// }
