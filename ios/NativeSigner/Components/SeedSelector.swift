//
//  SeedSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 27.7.2021.
//

import SwiftUI

struct SeedSelector: View {
    @EnvironmentObject var data: SignerDataModel
    var body: some View {
        HStack {
            Menu {
                Button(action: {
                    data.selectSeed(seedName: "")
                }) {
                    Text("Show all")
                }
                ForEach(data.seedNames, id: \.self) {seed in
                    Button(action: {
                        data.selectSeed(seedName: seed)
                    }) {
                        Text(seed)
                    }
                }
                Button(
                    action: {data.keyManagerModal = .newSeed}) {
                    Text(" + New seed")
                }
            } label: {
                if data.selectedSeed == "" {
                    Text("Select seed")
                        .font(.headline)
                } else {
                    VStack {
                        Text("seed").font(.footnote)
                        Text(data.selectedSeed)
                            .font(.headline)
                    }
                }
            }
        }
        .padding()
    }
}

struct SeedSelector_Previews: PreviewProvider {
    static var previews: some View {
        SeedSelector().previewLayout(.sizeThatFits)
    }
}
