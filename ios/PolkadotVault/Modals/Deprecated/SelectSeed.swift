//
//  SelectSeed.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 6.1.2022.
//

import SwiftUI

struct SelectSeed: View {
    let content: MSeeds
    @EnvironmentObject var data: SharedDataModel

    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 8).foregroundColor(Asset.backgroundPrimary.swiftUIColor)
            VStack {
                ScrollView(showsIndicators: false) {
                    LazyVStack {
                        ForEach(
                            content.seedNameCards.sorted(by: { $0.seedName < $1.seedName }),
                            id: \.seedName
                        ) { seedNameCard in
                            HStack {
                                Button(
                                    action: {
                                        data.sign(seedName: seedNameCard.seedName, comment: seedNameCard.seedName)
                                    },
                                    label: {
                                        SeedCardForManager(seedNameCard: seedNameCard)
                                        Spacer()
                                    }
                                )
                            }
                        }
                    }
                }
                Spacer()
            }
        }
    }
}

// struct SelectSeed_Previews: PreviewProvider {
// static var previews: some View {
// SelectSeed()
// }
// }
