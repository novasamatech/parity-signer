//
//  TransactionCardSelector.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TransactionCardSelector: View {
    var card: TransactionCard
    var body: some View {
        switch card.card {
        // Author cards with identicon and variable description
        case let .authorPlainCard(value): // Not present on new designs
            TCAuthorPlain(value: value)
        case let .authorPublicKeyCard(value): // Not present on new designs
            TCAuthorPublicKey(value: value)
        // Foldable Markdown values on tap
        case let .callCard(value): // This is used to present `Method` and provides details on tap
            TCCall(value: value)
        case let .enumVariantNameCard(value):
            TCEnumVariantName(value: value)
        case let .fieldNameCard(value): // Presents `dest` or `value` indentent values
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
        case let .derivationsCard(value): // Used for Import Derived Keys flow
            TCDerivations(value: .constant(value), viewModel: .init())
        case let .txSpecPlainCard(value): // Unknown network information for given transaction, not present on new
            // designs
            TCTXSpecPlain(content: value)
        // Error handling
        case let .errorCard(text):
            TCError(text: text)
        case let .warningCard(text):
            TCWarning(text: text)
        // Simple values with identicons / icons / markdown
        case let .networkInfoCard(value): // Not present in new designs
            TCNetworkInfo(content: value)
        case let .typesInfoCard(value): // Not present in new designs
            TCTypesInfo(content: value)
        case let .textCard(text): // Markdown text field, not present on new designs
            TCText(text: text)
        // Simple values - redesigned
        case let .authorCard(author):
            TCNamedValueCard(name: Localizable.TCName.from.string, value: author.base58, valueInSameLine: false)
        case let .balanceCard(value):
            TCNamedValueCard(value: [value.amount, value.units].joined(separator: " "))
        case let .bitVecCard(text):
            TCNamedValueCard(name: Localizable.TCName.bitVec.string, value: text)
        case let .blockHashCard(text):
            TCNamedValueCard(name: Localizable.TCName.blockHash.string, value: text, valueInSameLine: false)
        case let .defaultCard(text):
            TCNamedValueCard(value: text)
        case .eraImmortalCard:
            TCNamedValueCard(name: Localizable.immortalTransaction.string)
        case let .eraMortalCard(content):
            TCNamedValueCard(name: Localizable.TCName.phase.string, value: content.phase)
            TCNamedValueCard(name: Localizable.TCName.period.string, value: content.period)
        case let .idCard(value): // ID card, new designs present it without identicon
            TCID(value: value)
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
