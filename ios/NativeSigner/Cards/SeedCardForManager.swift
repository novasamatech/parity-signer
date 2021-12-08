//
//  AddressCardSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.10.2021.
//

import SwiftUI

struct SeedCardForManager: View {
    @EnvironmentObject var data: SignerDataModel
    var seedNameCard: SeedNameCard
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 4).foregroundColor(Color("Bg200")).frame(height: 47)
            HStack {
                Image(uiImage: UIImage(data: Data(fromHexEncodedString: seedNameCard.identicon) ?? Data()) ?? UIImage())
                    .resizable(resizingMode: .stretch)
                    .frame(width: 30, height: 30)
                VStack (alignment: .leading) {
                    Text(seedNameCard.seed_name)
                        .foregroundColor(Color("Text600"))
                        .font(FBase(style: .subtitle1))
                }
                Spacer()
            }
            .padding(8)
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
