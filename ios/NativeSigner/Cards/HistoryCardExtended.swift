//
//  HistoryCardExtended.swift
//  NativeSigner
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
            HistoryCard(event: event.event)
        case let .identityAdded(value):
            HistoryCardTemplate(
                image: .init(.aqi, variant: .medium),
                danger: false,
                line1: Localizable.keyCreated.string,
                line2: value.seedName + value.path + " in network with hash " +
                    value.networkGenesisHash.formattedAsString
            )
        case let .identityRemoved(value):
            HistoryCardTemplate(
                image: .init(.xmark, variants: [.rectangle, .portrait]),
                danger: false,
                line1: Localizable.keyRemoved.string,
                line2: value.seedName + value.path + " in network with hash " +
                    value.networkGenesisHash.formattedAsString
            )
        case let .secretWasExported(value):
            HistoryCardTemplate(
                image: .init(.eye, variants: [.trianglebadge, .exclamationmark, .fill]),
                danger: true,
                line1: Localizable.secretWasExported.string,
                line2: value.seedName + value.path + " in network with hash " +
                    value.networkGenesisHash.formattedAsString
            )
        case let .networkVerifierSet(value):
            switch value.validCurrentVerifier {
            case .general:
                HistoryCardTemplate(
                    image: .init(.checkmark, variant: .shield),
                    danger: false,
                    line1: Localizable.networkVerifierSet.string,
                    line2: value.generalVerifier.show() + " for network with genesis hash " +
                        value.genesisHash.formattedAsString
                )
            case let .custom(verifier):
                HistoryCardTemplate(
                    image: .init(.checkmark, variant: .shield),
                    danger: false,
                    line1: Localizable.networkVerifierSet.string,
                    line2: verifier.show() + " for network with genesis hash " +
                        value.genesisHash.formattedAsString
                )
            }
        case let .metadataSigned(value):
            HistoryCardTemplate(
                image: .init(.signature),
                danger: false,
                line1: Localizable.metadataSigned.string,
                line2: value.name + String(value.version)
            )
        case let .typesSigned(value):
            HistoryCardTemplate(
                image: .init(.signature),
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
                    Identicon(identicon: event.signedBy?.address.identicon.svgPayload ?? [])
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
                    Identicon(identicon: event.signedBy?.address.identicon.svgPayload ?? [])
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
                .stroke(Asset.crypto400.swiftUIColor)
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

// struct HistoryCardExtended_Previews: PreviewProvider {
// static var previews: some View {
// HistoryCardExtended()
// }
// }
