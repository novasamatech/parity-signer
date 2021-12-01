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
    @State var removeSeed = ""
    var body: some View {
        ZStack {
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
                                    data.selectSeed(seedName: seed)
                                    data.keyManagerModal = .seedBackup
                                }) {
                                    VStack {
                                        Image(systemName: "rectangle.and.pencil.and.ellipsis").imageScale(.large)
                                    }
                                    .background(Color("backgroundCard"))
                                }
                                Button(action: {
                                    removeSeed = seed
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
                                        message: Text("You are about to delete seed " + removeSeed),
                                        primaryButton: .cancel(),
                                        secondaryButton: .destructive(
                                            Text("Delete"),
                                            action: {
                                                data.removeSeed(seedName: removeSeed)
                                                removeSeed = ""
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
