//
//  NetworkDetails.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 13.10.2021.
//

import SwiftUI

struct NetworkDetails: View {
    let content: MNetworkDetails
    let navigationRequest: NavigationRequest
    var body: some View {
        ZStack {
            VStack {
                VStack(alignment: .leading) {
                    NetworkCard(title: content.title, logo: content.logo)
                    HStack {
                        Localizable.networkName.text
                        Text(content.name)
                    }
                    HStack {
                        Localizable.base58Prefix.text
                        Text(String(content.base58prefix))
                    }
                    HStack {
                        Localizable.decimals.text
                        Text(String(content.decimals))
                    }
                    HStack {
                        Localizable.unit.text
                        Text(content.unit)
                    }
                    HStack {
                        Localizable.genesisHash.text
                        Text(content.genesisHash.formattedAsString)
                            .fixedSize(horizontal: false, vertical: true)
                    }
                    HStack {
                        Localizable.verifierCertificateAlt.text.fixedSize(horizontal: false, vertical: true)
                        switch content.currentVerifier.ttype {
                        case "general":
                            Localizable.general.text
                        case "custom":
                            Identicon(identicon: content.currentVerifier.details.identicon)
                            VStack {
                                Localizable.custom.text
                                Text(content.currentVerifier.details.publicKey)
                                    .fixedSize(horizontal: false, vertical: true)
                                Text(Localizable.encryption(content.currentVerifier.details.encryption))
                            }
                        case "none":
                            Localizable.none.text
                        default:
                            Localizable.unknown.text
                        }
                    }
                }
                Localizable.metadataAvailable.text
                ScrollView {
                    LazyVStack {
                        ForEach(content.meta, id: \.metaHash) { metaEntry in
                            Button(
                                action: {
                                    navigationRequest(.init(action: .manageMetadata, details: metaEntry.specsVersion))
                                },
                                label: {
                                    MetadataCard(meta: metaEntry)
                                }
                            )
                        }
                    }
                }
                Spacer()
            }.padding()
        }
    }
}

// struct NetworkDetails_Previews: PreviewProvider {
// static var previews: some View {
// NetworkDetails()
// }
// }
