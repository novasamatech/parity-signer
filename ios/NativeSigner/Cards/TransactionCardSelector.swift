//
//  TransactionCardSelector.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TransactionCardSelector: View {
    var card: TransactionCard
    var body: some View {
        HStack {
        switch card.card {
        case .authorCard(let author):
            TCAuthor(author: author)
        case .authorPlainCard(let value):
            TCAuthorPlain(value: value)
        case .authorPublicKeyCard(let value):
            TCAuthorPublicKey(value: value)
        case .balanceCard(let value):
            TCBalance(value: value)
        case .bitVecCard(let text):
            TCBitVec(content: text)
        case .blockHashCard(let text):
            TCBlockHash(text: text)
        case .callCard(let value):
            TCCall(value: value)
        case .defaultCard(let text):
            TCDefault(content: text)
        case .derivationsCard(let value):
            TCDerivations(value: value)
        case .enumVariantNameCard(let value):
            TCEnumVariantName(value: value)
        case .eraImmortalCard:
            TCEraImmortal()
        case .eraMortalCard(let eraMortal):
            TCEraMortal(eraMortal: eraMortal)
        case .errorCard(let text):
            TCError(text: text)
        case .fieldNameCard(let value):
            TCFieldName(value: value)
        case .fieldNumberCard(let value):
            TCFieldNumber(value: value)
        case .idCard(let value):
            TCID(value: value)
        case .identityFieldCard(let text):
            TCIdentityField(content: text)
        case .metaCard(let value):
            TCMeta(content: value)
        case .nameVersionCard(let value):
            TCNameVersion(value: value)
        case .networkInfoCard(let value):
            TCNetworkInfo(content: value)
        case .newSpecsCard(let value):
            TCNewSpecs(value: value)
        case .nonceCard(let text):
            TCNonce(content: text)
        case .noneCard:
            Text("None")
        case .palletCard(let text):
            TCPallet(text: text)
        case .textCard(let text):
            TCText(text: text)
        case .tipCard(let value):
            TCTip(value: value)
        case .tipPlainCard(let text):
            TCTipPlain(content: text)
        case .txSpecCard(let value):
            TCTXSpec(value: value)
        case .txSpecPlainCard(let value):
            TCTXSpecPlain(content: value)
        case .typesInfoCard(let value):
            TCTypesInfo(content: value)
        case .varNameCard(let text):
            TCVarName(text: text)
        case .verifierCard(let value):
            TCVerifier(value: value)
        case .warningCard(let text):
            TCWarning(text: text)
        case .networkGenesisHashCard(let text):
            TCGenesisHash(content: text)
        case .networkNameCard(let text):
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
