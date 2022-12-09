//
//  Buttons.swift
//  NativeSigner
//
//  Created by Sveta Goldstein on 13.12.2021.
//

import SwiftUI

struct BigButtonStyle: ButtonStyle {
    var bgColor: Color
    var fgColor: Color

    func makeBody(configuration: Self.Configuration) -> some View {
        configuration.label
            .padding(12)
            .background(bgColor)
            .cornerRadius(8)
            .foregroundColor(fgColor)
    }
}

struct BigButton: View {
    var text: LocalizedStringKey
    var isShaded: Bool = false
    var isCrypto: Bool = false
    var isDangerous: Bool = false
    var action: () -> Void
    var isDisabled: Bool = false

    var body: some View {
        let accentColor = isCrypto ? Asset.textAndIconsPrimary.swiftUIColor : Asset.textAndIconsPrimary.swiftUIColor
        let bgColor = isDisabled
            ? Asset.backgroundSecondary.swiftUIColor
            : isShaded ? Asset.backgroundTertiary.swiftUIColor : accentColor
        let fgColor = isDisabled
            ? Asset.textAndIconsTertiary.swiftUIColor
            : isShaded
            ? isDangerous ? Asset.accentRed400.swiftUIColor : Asset.textAndIconsPrimary.swiftUIColor
            : Asset.accentForegroundText.swiftUIColor

        Button(action: action) {
            HStack {
                Spacer()
                Text(text).font(PrimaryFont.labelL.font)
                Spacer()
            }
        }
        .buttonStyle(BigButtonStyle(bgColor: bgColor, fgColor: fgColor))
        .disabled(isDisabled)
    }
}
