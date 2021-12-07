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
                                    data.pushButton(buttonID: .SelectSeed, details: seed)
                                }) {
                                    SeedCardForManager(seedName: seed)
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
