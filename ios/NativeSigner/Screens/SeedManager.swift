//
//  SeedManager.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 12.8.2021.
//

import SwiftUI

struct SeedManager: View {
    @EnvironmentObject var data: SignerDataModel
    var content: MSeeds
    @State var showBackup = false
    @State var deleteConfirm = false
    @State var seedPhrase = ""
    @State var removeSeed = ""
    var body: some View {
        ZStack {
            VStack {
                ScrollView {
                    LazyVStack {
                        ForEach(content.seedNameCards, id: \.seedName) {seedNameCard in
                            HStack {
                                Button(action: {
                                    data.pushButton(buttonID: .SelectSeed, details: seedNameCard.seedName)
                                }) {
                                    SeedCardForManager(seedNameCard: seedNameCard)
                                    Spacer()
                                }
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
