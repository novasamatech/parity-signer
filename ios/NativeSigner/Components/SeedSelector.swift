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
            } label: {
                if data.selectedSeed == "" {
                    Text("Select seed")
                        .font(.title)
                } else {
                    Text(data.selectedSeed)
                        .font(.title)
                }
            }
            Spacer()
            Button(
                action: {data.newSeed = true}) {
                Text("New seed")
                    .font(.title)
                    .foregroundColor(Color("AccentColor"))
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
