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
        Button(action: action) {
            HStack {
                Spacer()
                Text(text).font(PrimaryFont.labelL.font)
                Spacer()
            }
        }
        .buttonStyle(style())
        .disabled(isDisabled)
    }

    func style() -> BigButtonStyle {
        let bgColor: Color
        let fgColor: Color

        switch (isCrypto, isDisabled, isShaded) {
        case (true, true, _):
            bgColor = Asset.backgroundTertiary.swiftUIColor
            fgColor = Asset.textAndIconsTertiary.swiftUIColor
        case (true, false, true):
            bgColor = Asset.backgroundTertiary.swiftUIColor
            fgColor = isDangerous ? Asset.accentRed300.swiftUIColor : Asset.accentPink300.swiftUIColor
        case (true, false, false):
            bgColor = Asset.backgroundPrimary.swiftUIColor
            fgColor = Asset.textAndIconsPrimary.swiftUIColor
        case (false, true, _):
            bgColor = Asset.backgroundTertiary.swiftUIColor
            fgColor = Asset.textAndIconsTertiary.swiftUIColor
        case (false, false, true):
            bgColor = Asset.backgroundTertiary.swiftUIColor
            fgColor = isDangerous ? Asset.accentRed300.swiftUIColor : Asset.textAndIconsPrimary.swiftUIColor
        case (false, false, false):
            bgColor = Asset.accentPink500.swiftUIColor
            fgColor = Asset.accentForegroundText.swiftUIColor
        }
        return BigButtonStyle(bgColor: bgColor, fgColor: fgColor)
    }
}
