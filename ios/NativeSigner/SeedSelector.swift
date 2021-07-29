//
//  SeedSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 27.7.2021.
//

import SwiftUI

struct SeedSelector: View {
    @Binding var selectedSeed: String
    var seeds: [String]
    var body: some View {
        HStack {
            Menu {
                ForEach(seeds, id: \.self) {
                    seed in Button(action: {selectedSeed = seed}) {
                        Text(seed)
                    }
                }
                Button(action: {selectedSeed = ""}) {
                    Text("Show all")
                }
            } label: {
                if selectedSeed == "" {
                    Text("Select seed")
                        .font(.title)
                } else {
                    Text(selectedSeed)
                        .font(.title)
                }
            }
            Spacer()
            NavigationLink(
                destination: NewSeedScreen()
                ) {
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
        SeedSelector(selectedSeed: .constant("Alice"), seeds: ["Alice", "Bob", "Randy", "Elvis", "Zuk"]).previewLayout(.sizeThatFits)
    }
}
