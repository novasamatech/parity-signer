//
//  SeedCard.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 11.10.2021.
//

import SwiftUI

struct SeedCard: View {
    @EnvironmentObject var data: SignerDataModel
    var seedName: String
    var body: some View {
        HStack {
            Image(uiImage: data.getRootIdenticon(seedName: seedName))
                .resizable(resizingMode: .stretch)
                .frame(width: 42, height: 42)
            
            if seedName == "" {
                Text("Select seed")
                    .foregroundColor(Color("textMainColor"))
                    .font(.largeTitle)
            } else {
                VStack (alignment: .leading) {
                    Text(seedName)
                        .foregroundColor(Color("textMainColor"))
                        .font(.headline)
                    Text(data.getRootTruncated(seedName: seedName))
                        .font(.headline)
                        .foregroundColor(Color("textFadedColor"))
                }
            }
            
            Spacer()
            if data.getMultiSelectionMode() {
                if let rootAddress = data.getRootAddress(seedName: seedName) {
                    if data.multiSelected.contains(rootAddress) {
                        Image(systemName: "checkmark.circle.fill").foregroundColor(Color("AccentColor")).imageScale(.large)
                    } else {
                        Image(systemName: "circle").foregroundColor(Color("textFadedColor")).imageScale(.large)
                    }
                }
            }
        }
        .padding(8)
        .background(Color("backgroundCard"))
    }
}

/*
 struct SeedCard_Previews: PreviewProvider {
 static var previews: some View {
 SeedCard()
 }
 }
 */
