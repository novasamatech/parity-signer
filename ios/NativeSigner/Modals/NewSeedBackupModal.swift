//
//  NewSeedBackupModal.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 25.12.2021.
//

import SwiftUI

struct NewSeedBackupModal: View {
    @EnvironmentObject var data: SignerDataModel
    let content: MNewSeedBackup
    @State var confirmBackup = false
    @State var createRoots = true
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 8).foregroundColor(Color("Bg200"))
            VStack {
                HeaderBar(line1: "Backup Seed Phrase", line2: content.seed.decode64())
                ZStack {
                    //RoundedRectangle(cornerRadius: 8).foregroundColor(Color("Crypto100")).frame(height: 200)
                    Text(content.seed_phrase)
                        .font(.system(size: 16, weight: .semibold, design: .monospaced))
                        .foregroundColor(Color("Crypto400"))
                        .padding(8)
                }
                .background(RoundedRectangle(cornerRadius: 8).foregroundColor(Color("Crypto100")))
                VStack(spacing: 16) {
                    Button(action: {
                        confirmBackup.toggle()
                    }) {
                        HStack {
                            Image(systemName: confirmBackup ? "checkmark.square" : "square").imageScale(.large)
                            Text("I have written down my seed phrase")
                                .multilineTextAlignment(.leading).foregroundColor(Color("Text500"))
                            Spacer()
                        }
                    }
                    Button(action: {
                        createRoots.toggle()
                    }) {
                        HStack {
                            Image(systemName: createRoots ? "checkmark.square" : "square").imageScale(.large)
                            Text("Create seed keys")
                                .multilineTextAlignment(.leading).foregroundColor(Color("Text500"))
                            Spacer()
                        }
                    }
                    Spacer()
                    BigButton(
                        text: "Next",
                        action: {
                            data.restoreSeed(seedName: content.seed, seedPhrase: content.seed_phrase, createRoots: createRoots)
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
