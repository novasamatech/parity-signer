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
    var multiselectMode: Bool = false
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 4).foregroundColor(Color("Bg200")).frame(height: 47)
            HStack {
                ZStack {
                    Identicon(identicon: seedCard.identicon)
                    if multiselectMode && seedCard.base58 != "" {
                        VStack {
                            Spacer()
                            HStack {
                                Spacer()
                                Image(systemName: seedCard.multiselect ? "checkmark.circle.fill" : "circle")
                                    .imageScale(.large)
                            }
                        }
                    }
                }.frame(width: 30, height: 30)
                VStack(alignment: .leading) {
                    Text(seedCard.seedName)
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
