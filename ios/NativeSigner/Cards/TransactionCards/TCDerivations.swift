//
//  TCDerivations.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 6.1.2022.
//

import SwiftUI

struct TCDerivations: View {
    let value: [SeedKeysPreview]
    var body: some View {
        HStack {
            VStack {
                Localizable.importingDerivations.text
                    .font(Fontstyle.header1.base)
                    .foregroundColor(Asset.text600.swiftUIColor)
                ForEach(value, id: \.self) { seed in
                    ForEach(seed.derivedKeys, id: \.self) { key in
                        HStack {
                            Text(key.derivationPath ?? "/")
                                .font(Fontstyle.body2.crypto)
                                .foregroundColor(Asset.crypto400.swiftUIColor)
                            Spacer()
                        }
                    }
                }
            }
        }
    }
}

// struct TCDerivations_Previews: PreviewProvider {
// static var previews: some View {
// TCDerivations()
// }
// }
