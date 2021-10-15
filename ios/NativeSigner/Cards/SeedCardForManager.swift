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
                }
            }
        }
        .padding(8)
        .background(Color("backgroundCard"))
    }
}

/*
struct AddressCardSelector_Previews: PreviewProvider {
    static var previews: some View {
        SeedCardForManager()
    }
}
*/
