//
//  SettingsCardTemplate.swift
//  NativeSigner
//
//  Created by Sveta Goldstein on 15.12.2021.
//

import SwiftUI

struct SettingsCardTemplate: View {
    var text: String
    var danger: Bool = false
    var withIcon: Bool = true
    var withBackground: Bool = true

    var body: some View {
        HStack {
            Text(text)
                .font(Fontstyle.body1.base)
                .foregroundColor(danger ? Asset.signalDanger.swiftUIColor : Asset.text400.swiftUIColor)
            Spacer()
            if withIcon {
                Image(.chevron, variant: .forward)
                    .imageScale(.medium)
                    .foregroundColor(Asset.border400.swiftUIColor)
            }
        }
        .padding()
        .background(withBackground ? Asset.bg200.swiftUIColor : Color(""))
    }
}
