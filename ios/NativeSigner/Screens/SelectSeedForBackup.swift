//
//  SelectSeedForBackup.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 23.12.2021.
//

import SwiftUI

struct SelectSeedForBackup: View {
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
                                    navigationRequest(.init(action: .backupSeed, details: seedNameCard.seedName))
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

// struct SelectSeedForBackup_Previews: PreviewProvider {
// static var previews: some View {
// SelectSeedForBackup()
// }
// }
