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
        let accentColor = isCrypto ? Asset.crypto400.swiftUIColor : Asset.action400.swiftUIColor
        let bgColor = isDisabled
            ? Asset.bg200.swiftUIColor
            : isShaded ? Asset.bg300.swiftUIColor : accentColor
        let fgColor = isDisabled
            ? Asset.text300.swiftUIColor
            : isShaded
            ? isDangerous ? Asset.signalDanger.swiftUIColor : accentColor
            : Asset.action600.swiftUIColor

        Button(action: action) {
            HStack {
                Spacer()
                Text(text).font(Fontstyle.button.base)
                Spacer()
            }
        }
        .buttonStyle(BigButtonStyle(bgColor: bgColor, fgColor: fgColor))
        .disabled(isDisabled)
    }
}
