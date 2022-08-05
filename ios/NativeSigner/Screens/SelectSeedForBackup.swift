//
//  SelectSeedForBackup.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 23.12.2021.
//

import SwiftUI

struct SelectSeedForBackup: View {
    let content: MSeeds
    let pushButton: (Action, String, String) -> Void
    var body: some View {
        VStack {
            ScrollView {
                LazyVStack {
                    ForEach(
                        content.seedNameCards.sorted(by: {$0.seedName < $1.seedName}),
                        id: \.seedName
                    ) {seedNameCard in
                        HStack {
                            Button(
                                action: {
                                    pushButton(.backupSeed, seedNameCard.seedName, "")
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

/*
 struct SelectSeedForBackup_Previews: PreviewProvider {
 static var previews: some View {
 SelectSeedForBackup()
 }
 }
 */
