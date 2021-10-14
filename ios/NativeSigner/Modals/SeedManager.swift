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
                HeaderBar(line1: "SEEDS", line2: "select seed")
                ScrollView {
                    LazyVStack {
                        ForEach(data.seedNames, id: \.self) {seed in
                            HStack {
                                Button(action: {
                                    data.selectSeed(seedName: seed)
                                    data.keyManagerModal = .none
                                }) {
                                    SeedCardForManager(seedName: seed)
                                }
                                Spacer()
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
                                        message: Text(seedPhrase),
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
                            }
                            .background(Color("backgroundCard"))
                        }
                    }
                }
                Spacer()
                Button(
                    action: {data.keyManagerModal = .newSeed}) {
                        HStack {
                            Spacer()
                            Text("New seed").font(.subheadline)
                            Spacer()
                        }
                    }
                    .buttonStyle(.bordered)
            }
        }
    }
}

/*
 struct SeedManager_Previews: PreviewProvider {
 static var previews: some View {
 SeedManager()
 }
 }
 
 HStack {
 Button(action:{
 deleteConfirm = true
 }) {
 Text("Delete")
 .font(.largeTitle)
 .foregroundColor(Color("AccentColor"))
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
 Spacer()
 Button(action:{
 seedPhrase = data.getSeed(seedName: data.selectedSeed, backup: true)
 showBackup = !seedPhrase.isEmpty
 }) {
 Text("Backup")
 .font(.largeTitle)
 .foregroundColor(Color("AccentColor"))
 }
 .alert(isPresented: $showBackup, content: {
 Alert(
 title: Text("Backup your seed phrase"),
 message: Text(seedPhrase),
 dismissButton: .default(
 Text("Done"),
 action: {
 seedPhrase = ""
 showBackup = false
 }
 )
 )
 })
 }.padding()
 */
