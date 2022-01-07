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
        HStack {
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
            TCBitVec(content: text)
        case .blockHash(let text):
            TCBlockHash(text: text)
        case .call(let value):
            TCCall(value: value)
        case .defaultCard(let text):
            TCDefault(content: text)
        case .derivations(let value):
            TCDerivations(value: value)
        case .enumVariantName(let value):
            TCEnumVariantName(value: value)
        case .eraImmortal:
            TCEraImmortal()
        case .eraMortal(let eraMortal):
            TCEraMortal(eraMortal: eraMortal)
        case .error(let text):
            TCError(text: text)
        case .fieldName(let value):
            TCFieldName(value: value)
        case .fieldNumber(let value):
            TCFieldNumber(value: value)
        case .id(let value):
            TCID(value: value)
        case .identityField(let text):
            TCIdentityField(content: text)
        case .meta(let value):
            TCMeta(content: value)
        case .nameVersion(let value):
            TCNameVersion(value: value)
        case .newSpecs(let value):
            TCNewSpecs(value: value)
        case .nonce(let text):
            TCNonce(content: text)
        case .none:
            EmptyView()
        case .pallet(let text):
            TCPallet(text: text)
        case .text(let text):
            TCText(text: text)
        case .tip(let value):
            TCTip(value: value)
        case .tipPlain(let text):
            TCTipPlain(content: text)
        case .txSpec(let value):
            TCTXSpec(value: value)
        case .txSpecPlain(let value):
            TCTXSpecPlain(content: value)
        case .typesInfo(let text):
            TCTypesInfo(text: text)
        case .varName(let text):
            TCVarName(text: text)
        case .verifier(let value):
            TCVerifier(value: value)
        case .warning(let text):
            TCWarning(text: text)
        case .networkGenesisHash(let text):
            TCGenesisHash(content: text)
        case .networkName(let text):
            TCNetworkName(content: text)
        }
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
