//
//  TCNewSpecs.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 21.10.2021.
//

import SwiftUI

struct TCNewSpecs: View {
    var value: NetworkSpecsToSend
    var body: some View {
        VStack {
            Text("NEW NETWORK").foregroundColor(Asset.text600.swiftUIColor)
            VStack(alignment: .leading) {
                HStack {
                    Text("Network name:")
                        .foregroundColor(Asset.text400.swiftUIColor)
                    Text(value.title)
                        .foregroundColor(Asset.text600.swiftUIColor)
                }
                HStack {
                    Text("base58 prefix:")
                        .foregroundColor(Asset.text400.swiftUIColor)
                    Text(String(value.base58prefix))
                        .foregroundColor(Asset.text600.swiftUIColor)
                }
                HStack {
                    Text("decimals:")
                        .foregroundColor(Asset.text400.swiftUIColor)
                    Text(String(value.decimals))
                        .foregroundColor(Asset.text600.swiftUIColor)
                }
                HStack {
                    Text("unit:")
                        .foregroundColor(Asset.text400.swiftUIColor)
                    Text(value.unit)
                        .foregroundColor(Asset.text600.swiftUIColor)
                }
                HStack {
                    Text("genesis hash:")
                        .foregroundColor(Asset.text400.swiftUIColor)
                    Text(value.genesisHash.formattedAsString)
                        .foregroundColor(Asset.text600.swiftUIColor)
                }
                HStack {
                    Text("crypto:")
                        .foregroundColor(Asset.text400.swiftUIColor)
                    Text(
                        value.encryption == .ed25519 ? "ed25519" :
                            value.encryption == .sr25519 ? "sr25519" :
                            value.encryption == .ecdsa ? "ecdsa" : "error"
                    )
                    .foregroundColor(Asset.text600.swiftUIColor)
                }
                HStack {
                    Text("spec name:")
                        .foregroundColor(Asset.text400.swiftUIColor)
                    Text(value.name)
                        .foregroundColor(Asset.text600.swiftUIColor)
                }
                HStack {
                    Text("logo:")
                        .foregroundColor(Asset.text400.swiftUIColor)
                    NetworkLogo(logo: value.logo)
                }
                HStack {
                    Text("default path:")
                        .foregroundColor(Asset.text400.swiftUIColor)
                    Text(value.pathId)
                        .foregroundColor(Asset.text600.swiftUIColor)
                }
            }
        }
    }
}

// struct TCNewSpecs_Previews: PreviewProvider {
// static var previews: some View {
// TCNewSpecs()
// }
// }
