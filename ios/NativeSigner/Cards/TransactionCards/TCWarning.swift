//
//  TCWarning.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 17.8.2021.
//

import SwiftUI

struct TCWarning: View {
    let text: String
    var body: some View {
        HStack {
            Localizable.warning.text
            Text(text)
            Spacer()
        }
        .foregroundColor(Asset.accentRed400.swiftUIColor)
        .font(PrimaryFont.bodyM.font)
        .padding(Spacing.small)
        .containerBackground(isError: true)
    }
}

struct TCWarning_Previews: PreviewProvider {
    static var previews: some View {
        TCWarning(text: "Content of the warning")
    }
}
