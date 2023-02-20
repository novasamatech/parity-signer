//
//  SelectSeedForBackup.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 23.12.2021.
//

import SwiftUI

/// Probably deprecated
struct SelectSeedForBackup: View {
    let content: MSeeds
    @EnvironmentObject var navigation: NavigationCoordinator
    var body: some View {
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
                                    navigation
                                        .perform(navigation: .init(action: .backupSeed, details: seedNameCard.seedName))
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
