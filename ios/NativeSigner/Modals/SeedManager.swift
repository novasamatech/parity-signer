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
            RoundedRectangle(cornerRadius: 50).foregroundColor(/*@START_MENU_TOKEN@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
            ScrollView {
                Button(action: {
                    data.goBack()
                }) {
                    HStack {
                        Text("Show all").font(.largeTitle)
                        Spacer()
                    }.padding()
                }
                LazyVStack {
                    ForEach(data.seedNames, id: \.self) {seed in
                        HStack {
                            Button(action: {
                                data.selectSeed(seedName: seed)
                                data.goBack()
                            }) {
                                SeedCard(seedName: seed)
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
                Button(
                    action: {data.keyManagerModal = .newSeed}) {
                        Text(" + New seed").font(.title)
                    }
                Spacer()
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
