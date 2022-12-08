//
//  SmallButton.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 14.10.2021.
//

import SwiftUI

struct SmallButton: View {
    var text: LocalizedStringKey
    var body: some View {
        Text(text)
            .padding(.horizontal, 8)
            .padding(.vertical, 4)
            .overlay(RoundedRectangle(cornerRadius: 8.0).strokeBorder().foregroundColor(Asset.action400.swiftUIColor))
    }
}

// struct SmallButton_Previews: PreviewProvider {
//    static var previews: some View {
//        SmallButton(text: "test")
//    }
// }
