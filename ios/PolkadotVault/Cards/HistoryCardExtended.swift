//
//  HistoryCardExtended.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 28.10.2021.
//

import SwiftUI

struct HistoryCardExtended: View {
    var event: MEventMaybeDecoded
    var body: some View {
        switch event.event {
        case .databaseInitiated,
             .deviceWasOnline,
             .generalVerifierSet,
             .historyCleared,
             .identitiesWiped,
             .metadataAdded,
             .metadataRemoved,
             .networkSpecsAdded,
             .networkSpecsRemoved,
             .resetDangerRecord,
             .seedCreated,
             .seedRemoved,
             .seedNameWasShown,
             .networkSpecsSigned,
             .systemEntry,
             .typesAdded,
             .typesRemoved,
             .userEntry,
             .warning,
             .wrongPassword,
             .messageSignError,
             .messageSigned:
            HistoryCard(
                timestamp: nil,
                danger: event.event.isWarning,
                line1: event.event.eventTitle,
                line2: event.event.displayValue
            )
        case let .identityAdded(value):
            HistoryCard(
                danger: false,
                line1: Localizable.keyCreated.string,
                line2: value.seedName + value.path + " in network with hash " +
                    value.networkGenesisHash.formattedAsString
            )
        case let .identityRemoved(value):
            HistoryCard(
                danger: false,
                line1: Localizable.keyRemoved.string,
                line2: value.seedName + value.path + " in network with hash " +
                    value.networkGenesisHash.formattedAsString
            )
        case let .secretWasExported(value):
            HistoryCard(
                danger: true,
                line1: Localizable.secretWasExported.string,
                line2: value.seedName + value.path + " in network with hash " +
                    value.networkGenesisHash.formattedAsString
            )
        case let .networkVerifierSet(value):
            switch value.validCurrentVerifier {
            case .general:
                HistoryCard(
                    danger: false,
                    line1: Localizable.networkVerifierSet.string,
                    line2: value.generalVerifier.show() + " for network with genesis hash " +
                        value.genesisHash.formattedAsString
                )
            case let .custom(verifier):
                HistoryCard(
                    danger: false,
                    line1: Localizable.networkVerifierSet.string,
                    line2: verifier.show() + " for network with genesis hash " +
                        value.genesisHash.formattedAsString
                )
            }
        case let .metadataSigned(value):
            HistoryCard(
                danger: false,
                line1: Localizable.metadataSigned.string,
                line2: value.name + String(value.version)
            )
        case let .typesSigned(value):
            HistoryCard(
                danger: false,
                line1: Localizable.typesSigned.string,
                line2: value.typesHash.formattedAsString
            )
        case let .transactionSignError(value):
            VStack {
                Localizable.transactionFailed.text
                OldTransactionBlock(cards: event.decoded?.asSortedCards() ?? [])
                Localizable.signedBy.text
                HStack {
                    IdenticonView(identicon: event.signedBy?.address.identicon ?? .dots(identity: []))
                    VStack {
                        Text(value.signedBy.show())
                        Text((event.signedBy?.address.seedName ?? "") + (event.signedBy?.address.path ?? ""))
                    }
                }
                Localizable.inNetwork.text
                Text(value.networkName)
                Localizable.commentAlt.text
                Text(value.userComment)
            }
        case let .transactionSigned(value):
            VStack {
                OldTransactionBlock(cards: event.decoded?.asSortedCards() ?? [])
                Localizable.signedBy.text
                HStack {
                    IdenticonView(identicon: event.signedBy?.address.identicon ?? .dots(identity: []))
                    VStack {
                        Text(value.signedBy.show())
                        Text((event.signedBy?.address.seedName ?? "") + (event.signedBy?.address.path ?? ""))
                    }
                }
                Localizable.inNetwork.text
                Text(value.networkName)
                Localizable.commentAlt.text
                Text(value.userComment)
            }
        }
    }
}

struct OldTransactionBlock: View {
    var cards: [TransactionCard]
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: CornerRadius.extraSmall)
                .stroke(.fill6)
            VStack {
                ForEach(cards, id: \.index) { card in
                    TransactionCardView(card: card)
                }
            }
            .padding(Spacing.medium)
        }
        .padding(Spacing.small)
        .frame(width: UIScreen.main.bounds.size.width)
    }
}
