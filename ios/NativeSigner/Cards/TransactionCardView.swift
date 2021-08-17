//
//  TransactionCardView.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 10.8.2021.
//

import SwiftUI

struct TransactionCardView: View {
    var card: TransactionCard
    var body: some View {
        VStack(alignment: .leading) {
            TransactionCardSelector(card: card)
        }
        .border(Color("borderSignalColor"))
        .padding(.leading, CGFloat(card.indent)*10.0)
    }
}

 /*
struct TransactionCardView_Previews: PreviewProvider {
    static var previews: some View {
        TransactionCardView(card: TransactionCard(index: 0, indent: 0, card: .error("this is a preview")))
    }
}*/
