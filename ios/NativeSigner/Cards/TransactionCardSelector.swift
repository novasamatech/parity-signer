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
            // Author
            case let .authorCard(author): // Not present on new designs
                TCAuthor(author: author)
            case let .authorPlainCard(value): // Not present on new designs
                TCAuthorPlain(value: value)
            case let .authorPublicKeyCard(value): // Not present on new designs
                TCAuthorPublicKey(value: value)

            // Cards ?
            case let .balanceCard(value):
                TCNamedValueCard(name: "", value: [value.amount, value.units].joined(separator: " "))
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

            // Fields?
            case let .fieldNameCard(value):
                TCFieldName(value: value)
            case let .fieldNumberCard(value):
                TCFieldNumber(value: value)
            case let .idCard(value):
                TCID(value: value)
            case let .networkInfoCard(value):
                TCNetworkInfo(content: value)
            case let .newSpecsCard(value):
                TCAddNewNetwork(value: value)
            case let .textCard(text):
                TCText(text: text)
            case let .txSpecPlainCard(value):
                TCTXSpecPlain(content: value)
            case let .typesInfoCard(value):
                TCTypesInfo(content: value)

            // Sections
            case let .metaCard(value): // Used when scanning metadata update
                TCMeta(value: value)
            case let .verifierCard(value): // Used in metadata update, adding new network
                TCVerifier(value: value)

            // Error handling
            case let .errorCard(text):
                TCError(text: text)
            case let .warningCard(text):
                TCWarning(text: text)

            // Simple values
            case let .bitVecCard(text):
                TCNamedValueCard(name: Localizable.TCName.bitVec.string, value: text)
            case let .blockHashCard(text):
                TCNamedValueCard(name: Localizable.TCName.blockHash.string, value: text)
            case let .identityFieldCard(text):
                TCNamedValueCard(name: Localizable.TCName.identityField.string, value: text)
            case let .nameVersionCard(value):
                TCNamedValueCard(name: value.name, value: value.version)
            case let .nonceCard(text):
                TCNamedValueCard(name: Localizable.TCName.nonce.string, value: text)
            case let .palletCard(text):
                TCNamedValueCard(name: Localizable.TCName.pallet.string, value: text)
            case let .tipCard(value):
                TCNamedValueCard(
                    name: Localizable.TCName.tip.string,
                    value: [value.amount, value.units].joined(separator: " ")
                )
            case let .tipPlainCard(text):
                TCNamedValueCard(name: Localizable.TCName.tip.string, value: text)
            case let .txSpecCard(value):
                TCNamedValueCard(name: Localizable.TCName.TxVersion.uppercased.string, value: value)
            case let .networkGenesisHashCard(text):
                TCNamedValueCard(name: Localizable.TCName.genesisHash.string, value: text)
            case let .networkNameCard(text):
                TCNamedValueCard(name: Localizable.TCName.networkName.string, value: text)
            case let .varNameCard(text):
                TCNamedValueCard(name: "", value: text)
            case .noneCard:
                TCNamedValueCard(name: Localizable.noneCapitalised.string, value: "")
            }
        }
    }
}

// struct TransactionCardSelector_Previews: PreviewProvider {
//    static var previews: some View {
//        TransactionCardSelector()
//    }
// }
