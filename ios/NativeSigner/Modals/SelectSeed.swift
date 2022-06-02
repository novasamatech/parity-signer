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
            RoundedRectangle(cornerRadius: 8).foregroundColor(Color("Bg100"))
            VStack {
                ScrollView {
                    LazyVStack {
                        ForEach(content.seedNameCards.sorted(by: {$0.seedName < $1.seedName}), id: \.seedName) {seedNameCard in
                            HStack {
                                Button(action: {
                                    let seedPhrase = data.getSeed(seedName: seedNameCard.seedName)
                                    if seedPhrase != "" {
                                        data.pushButton(action: .goForward, details: seedNameCard.seedName, seedPhrase: seedPhrase)
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
