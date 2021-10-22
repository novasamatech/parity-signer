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
        ZStack {
            RoundedRectangle(cornerRadius: 4).foregroundColor(Color("backgroundCard")).frame(height: 44)
            HStack {
                Image(uiImage: data.getRootIdenticon(seedName: seedName))
                    .resizable(resizingMode: .stretch)
                    .frame(width: 28, height: 28)
                
                if seedName == "" { //should never happen but just in case
                    Text("Select seed")
                        .foregroundColor(Color("textMainColor"))
                        .font(.title2)
                } else {
                    VStack (alignment: .leading) {
                        Text(seedName)
                            .foregroundColor(Color("textMainColor"))
                            .font(.callout)
                        Text(data.getRootTruncated(seedName: seedName))
                            .font(.system(size: 12, design: .monospaced))
                            .foregroundColor(Color("textFadedColor"))
                    }
                }
                
                Spacer()
                if data.getMultiSelectionMode() {
                    if let rootAddress = data.getRootAddress(seedName: seedName) {
                        if data.multiSelected.contains(rootAddress) {
                            Image(systemName: "checkmark.circle.fill").foregroundColor(Color("AccentColor")).imageScale(.medium)
                        } else {
                            Image(systemName: "circle").foregroundColor(Color("textFadedColor")).imageScale(.medium)
                        }
                    }
                }
            }
            .padding(8)
        }
    }
}

/*
 struct SeedCard_Previews: PreviewProvider {
 static var previews: some View {
 SeedCard()
 }
 }
 */
