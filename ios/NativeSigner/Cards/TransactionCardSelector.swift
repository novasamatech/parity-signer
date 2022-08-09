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
            case let .authorCard(author):
                TCAuthor(author: author)
            case let .authorPlainCard(value):
                TCAuthorPlain(value: value)
            case let .authorPublicKeyCard(value):
                TCAuthorPublicKey(value: value)
            case let .balanceCard(value):
                TCBalance(value: value)
            case let .bitVecCard(text):
                TCBitVec(content: text)
            case let .blockHashCard(text):
                TCBlockHash(text: text)
            case let .callCard(value):
                TCCall(value: value)
            case let .defaultCard(text):
                TCDefault(content: text)
            case let .derivationsCard(value):
                TCDerivations(value: value)
            case let .enumVariantNameCard(value):
                TCEnumVariantName(value: value)
            case .eraImmortalCard:
                TCEraImmortal()
            case let .eraMortalCard(eraMortal):
                TCEraMortal(eraMortal: eraMortal)
            case let .errorCard(text):
                TCError(text: text)
            case let .fieldNameCard(value):
                TCFieldName(value: value)
            case let .fieldNumberCard(value):
                TCFieldNumber(value: value)
            case let .idCard(value):
                TCID(value: value)
            case let .identityFieldCard(text):
                TCIdentityField(content: text)
            case let .metaCard(value):
                TCMeta(content: value)
            case let .nameVersionCard(value):
                TCNameVersion(value: value)
            case let .networkInfoCard(value):
                TCNetworkInfo(content: value)
            case let .newSpecsCard(value):
                TCNewSpecs(value: value)
            case let .nonceCard(text):
                TCNonce(content: text)
            case .noneCard:
                Localizable.noneCapitalised.text
            case let .palletCard(text):
                TCPallet(text: text)
            case let .textCard(text):
                TCText(text: text)
            case let .tipCard(value):
                TCTip(value: value)
            case let .tipPlainCard(text):
                TCTipPlain(content: text)
            case let .txSpecCard(value):
                TCTXSpec(value: value)
            case let .txSpecPlainCard(value):
                TCTXSpecPlain(content: value)
            case let .typesInfoCard(value):
                TCTypesInfo(content: value)
            case let .varNameCard(text):
                TCVarName(text: text)
            case let .verifierCard(value):
                TCVerifier(value: value)
            case let .warningCard(text):
                TCWarning(text: text)
            case let .networkGenesisHashCard(text):
                TCGenesisHash(content: text)
            case let .networkNameCard(text):
                TCNetworkName(content: text)
            }
        }
    }
}

// struct TransactionCardSelector_Previews: PreviewProvider {
//    static var previews: some View {
//        TransactionCardSelector()
//    }
// }
