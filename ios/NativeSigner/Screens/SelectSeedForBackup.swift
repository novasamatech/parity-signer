//
//  SelectSeedForBackup.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 23.12.2021.
//

import SwiftUI

struct SelectSeedForBackup: View {
    @EnvironmentObject var data: SignerDataModel
    var content: MSeeds
    var body: some View {
        VStack {
            ScrollView {
                LazyVStack {
                    ForEach(content.seedNameCards.sorted(by: {$0.seed_name < $1.seed_name}), id: \.seed_name) {seedNameCard in
                        HStack {
                            Button(action: {
                                data.pushButton(buttonID: .BackupSeed, details: seedNameCard.seed_name)
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
struct SelectSeedForBackup_Previews: PreviewProvider {
    static var previews: some View {
        SelectSeedForBackup()
    }
}
*/
