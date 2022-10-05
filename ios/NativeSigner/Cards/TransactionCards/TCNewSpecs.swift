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
            Localizable.newNetwork.text
                .foregroundColor(Asset.text600.swiftUIColor)
            VStack(alignment: .leading) {
                HStack {
                    Localizable.networkName.text
                        .foregroundColor(Asset.text400.swiftUIColor)
                    Text(value.title)
                        .foregroundColor(Asset.text600.swiftUIColor)
                }
                HStack {
                    Localizable.base58Prefix.text
                        .foregroundColor(Asset.text400.swiftUIColor)
                    Text(String(value.base58prefix))
                        .foregroundColor(Asset.text600.swiftUIColor)
                }
                HStack {
                    Localizable.decimals.text
                        .foregroundColor(Asset.text400.swiftUIColor)
                    Text(String(value.decimals))
                        .foregroundColor(Asset.text600.swiftUIColor)
                }
                HStack {
                    Localizable.unit.text
                        .foregroundColor(Asset.text400.swiftUIColor)
                    Text(value.unit)
                        .foregroundColor(Asset.text600.swiftUIColor)
                }
                HStack {
                    Localizable.genesisHash.text
                        .foregroundColor(Asset.text400.swiftUIColor)
                    Text(value.genesisHash.formattedAsString)
                        .foregroundColor(Asset.text600.swiftUIColor)
                }
                HStack {
                    Localizable.crypto.text
                        .foregroundColor(Asset.text400.swiftUIColor)
                    Text(value.encryption.rawValue)
                        .foregroundColor(Asset.text600.swiftUIColor)
                }
                HStack {
                    Localizable.specName.text
                        .foregroundColor(Asset.text400.swiftUIColor)
                    Text(value.name)
                        .foregroundColor(Asset.text600.swiftUIColor)
                }
                HStack {
                    Localizable.logo.text
                        .foregroundColor(Asset.text400.swiftUIColor)
                    NetworkLogo(logo: value.logo)
                }
                HStack {
                    Localizable.defaultPath.text
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
