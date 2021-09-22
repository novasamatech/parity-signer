//
//  TransactionCardSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

//TODO: all cards should be pretty
struct TransactionCardSelector: View {
    var card: TransactionCard
    var body: some View {
        switch card.card {
        case .author(let author):
            TCAuthor(author: author)
        case .authorPlain(let value):
            TCAuthorPlain(value: value)
        case .authorPublicKey(let value):
            TCAuthorPublicKey(value: value)
        case .balance(let value):
            TCBalance(value: value)
        case .bitVec(let text):
            Text(text).foregroundColor(Color("textMainColor"))
        case .blockHash(let text):
            TCBlockHash(text: text)
        case .call(let value):
            TCCall(value: value)
        case .defaultCard(let text):
            Text(text).foregroundColor(Color("textMainColor"))
        case .enumVariantName(let text):
            TCEnumVariantName(text: text)
        case .eraImmortalNonce(let eraImmortalNonce):
            TCEraImmortalNonce(eraImmortalNonce: eraImmortalNonce)
        case .eraMortalNonce(let eraMortalNonce):
            TCEraMortalNonce(eraMortalNonce: eraMortalNonce)
        case .error(let text):
            TCError(text: text)
        case .fieldName(let text):
            Text(text).foregroundColor(Color("textMainColor"))
        case .fieldNumber(let text):
            Text(text).foregroundColor(Color("textMainColor"))
        case .id(let text):
            TCID(text: text)
        case .identityField(let text):
            Text(text).foregroundColor(Color("textMainColor"))
        case .meta(let value):
            Text(String(describing: value))
                .foregroundColor(Color("textMainColor"))
        case .newNetwork(let value):
            Text(String(describing: value))
                .foregroundColor(Color("textMainColor"))
        case .none:
            EmptyView()
        case .tip(let value):
            TCTip(value: value)
        case .tipPlain(let text):
            Text(text).foregroundColor(Color("textMainColor"))
        case .txSpec(let value):
            TCTXSpec(value: value)
        case .txSpecPlain(let value):
            Text(String(describing: value))
                .foregroundColor(Color("textMainColor"))
        case .typesInfo(let text):
            TCTypesInfo(text: text)
        case .varName(let text):
            TCVarName(text: text)
        case .verifier(let value):
            Text(String(describing: value))
                .foregroundColor(Color("textMainColor"))
        case .warning(let text):
            TCWarning(text: text)
        }
    }
}

/*
struct TransactionCardSelector_Previews: PreviewProvider {
    static var previews: some View {
        TransactionCardSelector()
    }
}
*/
