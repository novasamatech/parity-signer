//
//  SeedManager.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 12.8.2021.
//

import SwiftUI

struct SeedManager: View {
    let content: MSeeds
    let navigationRequest: NavigationRequest
    var body: some View {
        VStack {
            ScrollView {
                LazyVStack {
                    ForEach(
                        content.seedNameCards.sorted(by: { $0.seedName < $1.seedName }),
                        id: \.seedName
                    ) { seedNameCard in
                        HStack {
                            Button(
                                action: {
                                    navigationRequest(.init(action: .selectSeed, details: seedNameCard.seedName))
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

// struct SeedManager_Previews: PreviewProvider {
// static var previews: some View {
// SeedManager()
// }
