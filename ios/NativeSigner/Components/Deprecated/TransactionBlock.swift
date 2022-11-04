//
//  TransactionBlock.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 28.12.2021.
//

import SwiftUI

struct TransactionBlock: View {
    var cards: [TransactionCard]
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 8).stroke(Asset.crypto400.swiftUIColor)
            VStack {
                ForEach(cards, id: \.index) { card in
                    TransactionCardView(card: card)
                }
            }
            .padding(16)
        }
        .padding(12)
        .frame(width: UIScreen.main.bounds.size.width)
    }
}

// struct TransactionBlock_Previews: PreviewProvider {
// static var previews: some View {
// TransactionBlock()
// }
// }
