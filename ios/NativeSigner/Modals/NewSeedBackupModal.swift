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
    let pushButton: (Action, String, String) -> Void
    @State var confirmBackup = false
    @State var createRoots = true
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 8).foregroundColor(Color("Bg200"))
            VStack {
                HeaderBar(line1: "Backup Seed Phrase", line2: content.seed)
                ZStack {
                    Text(content.seedPhrase)
                        .font(.system(size: 16, weight: .semibold, design: .monospaced))
                        .foregroundColor(Color("Crypto400"))
                        .padding(8)
                }
                .background(RoundedRectangle(cornerRadius: 8).foregroundColor(Color("Crypto100")))
                VStack(spacing: 16) {
                    Button(
                        action: {
                            confirmBackup.toggle()
                        },
                        label: {
                            HStack {
                                Image(systemName: confirmBackup ? "checkmark.square" : "square").imageScale(.large)
                                Text("I have written down my seed phrase")
                                    .multilineTextAlignment(.leading).foregroundColor(Color("Text500"))
                                Spacer()
                            }
                        })
                    Button(
                        action: {
                            createRoots.toggle()
                        },
                        label: {
                            HStack {
                                Image(systemName: createRoots ? "checkmark.square" : "square").imageScale(.large)
                                Text("Create root keys")
                                    .multilineTextAlignment(.leading).foregroundColor(Color("Text500"))
                                Spacer()
                            }
                        })
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

/*
 struct NewSeedBackupModal_Previews: PreviewProvider {
 static var previews: some View {
 NewSeedBackupModal()
 }
 }
 */
