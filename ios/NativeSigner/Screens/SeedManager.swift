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
        VStack {
            ScrollView {
                LazyVStack {
                    ForEach(content.seedNameCards.sorted(by: {$0.seed_name < $1.seed_name}), id: \.seed_name) {seedNameCard in
                        HStack {
                            Button(action: {
                                data.pushButton(buttonID: .SelectSeed, details: seedNameCard.seed_name)
                            }) {
                                SeedCardForManager(seedNameCard: seedNameCard)
                                Spacer()
                            }
                        }
                    }
                }
            }
            Spacer()
        }
    }
}

/*
 struct SeedManager_Previews: PreviewProvider {
 static var previews: some View {
 SeedManager()
 }
 */
