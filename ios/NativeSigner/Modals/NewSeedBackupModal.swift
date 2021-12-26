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
            RoundedRectangle(cornerRadius: 8).foregroundColor(Color("Bg000"))
            VStack {
                ZStack {
                    RoundedRectangle(cornerRadius: 8).stroke(Color("Crypto400")).foregroundColor(Color("Bg000")).frame(height: 200)
                    Text(content.seed_phrase)
                        .font(.system(size: 16, weight: .semibold, design: .monospaced))
                        .foregroundColor(Color("Crypto400"))
                        .padding(8)
                }
                VStack(spacing: 16) {
                    Button(action: {
                        confirmBackup.toggle()
                    }) {
                        HStack {
                            Image(systemName: confirmBackup ? "checkmark.square" : "square").imageScale(.large)
                            Text("I have written down my seed phrase")
                                .multilineTextAlignment(.leading)
                            Spacer()
                        }
                    }
                    Button(action: {
                        createRoots.toggle()
                    }) {
                        HStack {
                            Image(systemName: createRoots ? "checkmark.square" : "square").imageScale(.large)
                            Text("Create root keys")
                                .multilineTextAlignment(.leading)
                            Spacer()
                        }
                    }
                    BigButton(
                        text: "Next",
                        action: {
                            data.restoreSeed(seedName: content.seed, seedPhrase: content.seed_phrase, createRoots: createRoots)
                        },
                        isDisabled: !confirmBackup
                    )
                        .padding(.top, 16.0)
                }
            }
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
