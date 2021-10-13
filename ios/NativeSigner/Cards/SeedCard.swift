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
                        Image(systemName: "checkmark.circle.fill")
                    } else {
                        Image(systemName: "circle")
                    }
                }
            }
        }
        .padding(8)
        .background(Color(seedName == "" ? "backgroundColor" : "backgroundCard"))
    }
}

/*
 struct SeedCard_Previews: PreviewProvider {
 static var previews: some View {
 SeedCard()
 }
 }
 */
