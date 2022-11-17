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
            // Author cards with identicon and variable description
            case let .authorCard(author): // Not present on new designs
                TCAuthor(author: author)
            case let .authorPlainCard(value): // Not present on new designs
                TCAuthorPlain(value: value)
            case let .authorPublicKeyCard(value): // Not present on new designs
                TCAuthorPublicKey(value: value)

            // Foldable Markdown values on tap
            case let .callCard(value):
                TCCall(value: value)
            case let .enumVariantNameCard(value):
                TCEnumVariantName(value: value)
            case let .fieldNameCard(value):
                TCFieldName(value: value)
            case let .fieldNumberCard(value):
                TCFieldNumber(value: value)

            // Sections
            case let .newSpecsCard(value): // User when adding new network, redesigned
                TCAddNewNetwork(value: value)
            case let .metaCard(value): // Used when scanning metadata update, redesigned
                TCMeta(value: value)
            case let .verifierCard(value): // Used in metadata update, adding new network, redesigned
                TCVerifier(value: value)
            case let .derivationsCard(value): // Not present on new designs
                TCDerivations(value: value)
            case let .txSpecPlainCard(value): // Unknown network information for given transaction, not present on new
                // designs
                TCTXSpecPlain(content: value)

            // Error handling
            case let .errorCard(text):
                TCError(text: text)
            case let .warningCard(text):
                TCWarning(text: text)

            // List values
            case let .eraMortalCard(content):
                TCEraMortal(content: content)

            // Simple values with identicons / icons / markdown
            case let .idCard(value): // Not present in new designs
                TCID(value: value)
            case let .networkInfoCard(value): // Not present in new designs
                TCNetworkInfo(content: value)
            case let .typesInfoCard(value): // Not present in new designs
                TCTypesInfo(content: value)
            case let .textCard(text): // Markdown text field, not present on new designs
                TCText(text: text)

            // Simple values - redesigned
            case let .balanceCard(value):
                TCNamedValueCard(value: [value.amount, value.units].joined(separator: " "))
            case let .bitVecCard(text):
                TCNamedValueCard(name: Localizable.TCName.bitVec.string, value: text)
            case let .blockHashCard(text):
                TCNamedValueCard(name: Localizable.TCName.blockHash.string, value: text)
            case let .defaultCard(text):
                TCNamedValueCard(value: text)
            case .eraImmortalCard:
                TCNamedValueCard(name: Localizable.immortalTransaction.string)
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
                TCNamedValueCard(value: text)
            case .noneCard:
                TCNamedValueCard(name: Localizable.noneCapitalised.string)
            }
        }
    }
}

// struct TransactionCardSelector_Previews: PreviewProvider {
//    static var previews: some View {
//        TransactionCardSelector()
//    }
// }
