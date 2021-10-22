//
//  AddressCardSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.10.2021.
//

import SwiftUI

struct SeedCardForManager: View {
    @EnvironmentObject var data: SignerDataModel
    var seedName: String
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 4).foregroundColor(Color("backgroundCard")).frame(height: 44)
            HStack {
                Image(uiImage: data.getRootIdenticon(seedName: seedName))
                    .resizable(resizingMode: .stretch)
                    .frame(width: 28, height: 28)
                
                if seedName == "" {
                    Text("Select seed")
                        .foregroundColor(Color("textMainColor"))
                        .font(.title2)
                } else {
                    VStack (alignment: .leading) {
                        Text(seedName)
                            .foregroundColor(Color("textMainColor"))
                            .font(.callout)
                    }
                }
                Spacer()
            }
            .padding(8)
            .background(Color("backgroundCard"))
        }
    }
}

/*
 struct AddressCardSelector_Previews: PreviewProvider {
 static var previews: some View {
 SeedCardForManager()
 }
 }
 */
