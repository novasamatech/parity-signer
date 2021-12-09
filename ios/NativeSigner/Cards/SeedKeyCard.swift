//
//  SeedKeyCard.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 9.12.2021.
//

import SwiftUI

struct SeedKeyCard: View {
    @EnvironmentObject var data: SignerDataModel
    var seedCard: MSeedKeyCard
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 4).foregroundColor(Color("Bg200")).frame(height: 47)
            HStack {
                Image(uiImage: UIImage(data: Data(fromHexEncodedString: seedCard.identicon) ?? Data()) ?? UIImage())
                    .resizable(resizingMode: .stretch)
                    .frame(width: 30, height: 30)
                VStack (alignment: .leading) {
                    Text(seedCard.seed_name)
                        .foregroundColor(Color("Text600"))
                        .font(FBase(style: .subtitle1))
                    Text(seedCard.base58.truncateMiddle(length: 8))
                        .foregroundColor(Color("Text400"))
                        .font(FCrypto(style: .body1))
                }
                Spacer()
            }
            .padding(8)
        }
    }
}

/*
struct SeedKeyCard_Previews: PreviewProvider {
    static var previews: some View {
        SeedKeyCard()
    }
}
*/
