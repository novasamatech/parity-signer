//
//  SufficientCryptoReady.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 21.12.2021.
//

import SwiftUI

struct SufficientCryptoReady: View {
    @GestureState private var dragOffset = CGSize.zero
    @State private var offset: CGFloat = 0
    @State private var oldOffset: CGFloat = 0
    var content: MSufficientCryptoReady
    var body: some View {
        ZStack {
            RoundedRectangle(cornerRadius: 8).foregroundColor(Asset.bg000.swiftUIColor)
            VStack {
                HeaderBar(
                    line1: Localizable.yourSignature.key,
                    line2: Localizable.scanItIntoYourApplication.key
                )
                Image(uiImage: UIImage(data: Data(content.sufficient)) ?? UIImage())
                    .resizable()
                    .aspectRatio(contentMode: .fit)
                    .padding(12)
                AddressCard(address: content.authorInfo)
                switch content.content {
                case let .addSpecs(network):
                    Localizable.signatureForNetworkSpecs.text
                    NetworkCard(title: network.networkTitle, logo: network.networkLogo)
                case .loadTypes(types: _, pic: _):
                    Localizable.signatureForTypes.text
                case let .loadMetadata(name: name, version: version):
                    Localizable.signatureForMetadataUpdate.text
                    Text(Localizable.Signature.metadata(name, String(version)))
                }
            }
        }
    }
}

// struct SufficientCryptoReady_Previews: PreviewProvider {
// static var previews: some View {
// SufficientCryptoReady()
// }
// }
