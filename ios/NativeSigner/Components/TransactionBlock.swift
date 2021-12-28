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
            RoundedRectangle(cornerRadius: 8).stroke(Color("Crypto400"))
        ScrollView {
            LazyVStack {
                ForEach(cards, id: \.index) { card in
                    TransactionCardView(card: card)
                }
            }
        }
            
        }
    }
}

/*
struct TransactionBlock_Previews: PreviewProvider {
    static var previews: some View {
        TransactionBlock()
    }
}
*/
