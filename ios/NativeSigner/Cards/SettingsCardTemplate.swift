//
//  SettingsCardTemplate.swift
//  NativeSigner
//
//  Created by Sveta Goldstein on 15.12.2021.
//

import SwiftUI

struct SettingsCardTemplate: View {
    var text: LocalizedStringKey
    var danger: Bool = false
    var withIcon: Bool = true
    var withBackground: Bool = true

    var body: some View {
        HStack {
            Text(text)
                .font(PrimaryFont.bodyL.font)
                .foregroundColor(danger ? Asset.signalDanger.swiftUIColor : Asset.text400.swiftUIColor)
            Spacer()
            if withIcon {
                Image(.chevron, variant: .forward)
                    .imageScale(.medium)
                    .foregroundColor(Asset.border400.swiftUIColor)
            }
        }
        .padding()
        .background(withBackground ? Asset.bg200.swiftUIColor : Color.clear)
    }
}
