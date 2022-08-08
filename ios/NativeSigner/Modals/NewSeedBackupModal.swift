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
            RoundedRectangle(cornerRadius: 8).foregroundColor(Asset.bg200.swiftUIColor)
            VStack {
                HeaderBar(line1: "Backup Seed Phrase", line2: content.seed)
                ZStack {
                    Text(content.seedPhrase)
                        .font(.system(size: 16, weight: .semibold, design: .monospaced))
                        .foregroundColor(Asset.crypto400.swiftUIColor)
                        .padding(8)
                }
                .background(RoundedRectangle(cornerRadius: 8).foregroundColor(Asset.crypto100.swiftUIColor))
                VStack(spacing: 16) {
                    Button(
                        action: {
                            confirmBackup.toggle()
                        },
                        label: {
                            HStack {
                                (confirmBackup ? Image(.checkmark, variant: .square) : Image(.square))
                                    .imageScale(.large)
                                Text("I have written down my seed phrase")
                                    .multilineTextAlignment(.leading).foregroundColor(Asset.text500.swiftUIColor)
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
                                Text("Create root keys")
                                    .multilineTextAlignment(.leading).foregroundColor(Asset.text500.swiftUIColor)
                                Spacer()
                            }
                        }
                    )
                    Spacer()
                    BigButton(
                        text: "Next",
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
