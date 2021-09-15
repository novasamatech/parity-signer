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
    @State var seedPhrase = ""
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 50).foregroundColor(/*@START_MENU_TOKEN@*/Color("backgroundCard")/*@END_MENU_TOKEN@*/)
            VStack {
                Text("Seeds")
                    .font(.largeTitle)
                    .foregroundColor(Color("AccentColor"))
            ScrollView {
                LazyVStack {
                    ForEach(data.seedNames, id: \.self) {seed in
                        VStack {
                            if(seed == data.selectedSeed) {
                                Button(action: {
                                        data.selectedSeed = ""
                                }) {
                                    Text(seed)
                                        .font(.largeTitle)
                                        .foregroundColor(Color("AccentColor"))
                                }
                                HStack {
                                    Button(action:{}) {
                                        Text("Delete")
                                            .font(.largeTitle)
                                            .foregroundColor(Color("AccentColor"))
                                    }
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
                            } else {
                                Button(action: {
                                        data.selectSeed(seedName: seed)
                                }) {
                                    Text(seed)
                                        .font(.largeTitle)
                                        .foregroundColor(Color("AccentColor"))
                                }
                            }
                        }
                    }
                }
            }
                Spacer()
                Button(action: {data.settingsModal = .none})
                    {
                    Text("Back")
                        .font(.largeTitle)
                        .foregroundColor(Color("AccentColor"))
                }
            }
        }
        .onDisappear {
            seedPhrase = ""
        }
    }
}

/*
struct SeedManager_Previews: PreviewProvider {
    static var previews: some View {
        SeedManager()
    }
}
*/
