//
//  SeedManager.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 12.8.2021.
//

import SwiftUI

struct SeedManager: View {
    @EnvironmentObject var data: SignerDataModel
    @State var showBackup = false
    @State var deleteConfirm = false
    @State var seedPhrase = ""
    var body: some View {
        ZStack {
            ModalBackdrop()
            VStack {
                ScrollView {
                    LazyVStack {
                        ForEach(data.seedNames, id: \.self) {seed in
                            HStack {
                                Button(action: {
                                    data.selectSeed(seedName: seed)
                                    data.keyManagerModal = .none
                                }) {
                                    SeedCardForManager(seedName: seed)
                                    Spacer()
                                }
                                Button(action: {
                                    seedPhrase = data.getSeed(seedName: seed, backup: true)
                                    showBackup = !seedPhrase.isEmpty
                                }) {
                                    VStack {
                                        Image(systemName: "eye").imageScale(.large)
                                    }
                                    .background(Color("backgroundCard"))
                                }
                                .alert(isPresented: $showBackup, content: {
                                    Alert(
                                        title: Text("Backup your seed phrase"),
                                        message: Text(seedPhrase).font(.title2),
                                        dismissButton: .default(
                                            Text("Done"),
                                            action: {
                                                seedPhrase = ""
                                                showBackup = false
                                            }
                                        )
                                    )
                                })
                                Button(action: {
                                    deleteConfirm = true
                                }) {
                                    VStack {
                                        Image(systemName: "trash").imageScale(.large)
                                    }
                                    .background(Color("backgroundCard"))
                                }
                                .alert(isPresented: $deleteConfirm, content: {
                                    Alert(
                                        title: Text("Delete seed?"),
                                        message: Text("You are about to delete seed " + seed),
                                        primaryButton: .cancel(),
                                        secondaryButton: .destructive(
                                            Text("Delete"),
                                            action: { data.removeSeed(seedName: seed)
                                            }
                                        )
                                    )
                                })
                                .padding(.horizontal)
                            }
                            .background(Color("backgroundCard"))
                        }
                    }
                }
                Spacer()
            }
        }.onAppear {
            data.selectSeed(seedName: "")
        }
    }
}

/*
 struct SeedManager_Previews: PreviewProvider {
 static var previews: some View {
 SeedManager()
 }
 }
 
 Button(
     action: {data.keyManagerModal = .newSeed}) {
         HStack {
             Spacer()
             Text("New seed").font(.subheadline)
             Spacer()
         }
     }
     .buttonStyle(.bordered)
 */
