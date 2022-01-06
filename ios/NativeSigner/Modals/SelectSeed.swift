//
//  SelectSeed.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 6.1.2022.
//

import SwiftUI

struct SelectSeed: View {
    @EnvironmentObject var data: SignerDataModel
    var content: MSeeds
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 8).foregroundColor(Color("Bg200"))
            VStack {
                ScrollView {
                    LazyVStack {
                        ForEach(content.seedNameCards.sorted(by: {$0.seed_name < $1.seed_name}), id: \.seed_name) {seedNameCard in
                            HStack {
                                Button(action: {
                                    let seedPhrase = data.getSeed(seedName: seedNameCard.seed_name)
                                    if seedPhrase != "" {
                                        data.pushButton(buttonID: .GoForward, details: seedNameCard.seed_name, seedPhrase: seedPhrase)
                                    }
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
}

/*
 struct SelectSeed_Previews: PreviewProvider {
 static var previews: some View {
 SelectSeed()
 }
 }
 */
