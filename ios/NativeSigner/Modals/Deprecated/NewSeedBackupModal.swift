//
//  NewSeedBackupModal.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 25.12.2021.
//

import SwiftUI

struct NewSeedBackupModal: View {
    let content: MNewSeedBackup
    let restoreSeed: (String, String, Bool) -> Void
    let navigationRequest: NavigationRequest
    @State private var confirmBackup = false
    @State private var createRoots = true
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 8).foregroundColor(Asset.backgroundSecondary.swiftUIColor)
            VStack {
                HeaderBar(line1: Localizable.backupSeedPhrase.key, line2: LocalizedStringKey(content.seed))
                ZStack {
                    Text(content.seedPhrase)
                        .font(.system(size: 16, weight: .semibold, design: .monospaced))
                        .foregroundColor(Asset.accentPink300.swiftUIColor)
                        .padding(8)
                }
                .background(RoundedRectangle(cornerRadius: 8).foregroundColor(Asset.backgroundSecondary.swiftUIColor))
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
                    Button(
                        action: {
                            createRoots.toggle()
                        },
                        label: {
                            HStack {
                                (createRoots ? Image(.checkmark, variant: .square) : Image(.square)).imageScale(.large)
                                Localizable.createRootKeys.text
                                    .multilineTextAlignment(.leading)
                                    .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                                Spacer()
                            }
                        }
                    )
                    Spacer()
                    BigButton(
                        text: Localizable.next.key,
                        action: {
                            restoreSeed(content.seed, content.seedPhrase, createRoots)
                        },
                        isDisabled: !confirmBackup
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
