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
    var text: String
    var isShaded: Bool = false
    var isCrypto: Bool = false
    var isDangerous: Bool = false
    var action: () -> Void
    var isDisabled: Bool = false

    var body: some View {
        let accentColor = isCrypto ? Color("Crypto400") : Color("Action400")
        let bgColor = isDisabled
            ? Color("Bg200")
            : isShaded ? Color("Bg300") : accentColor
        let fgColor = isDisabled
            ? Color("Text300")
            : isShaded
                ? isDangerous ? Color("SignalDanger") : accentColor
                : Color("Action600")

        Button(action: action) {
            HStack {
                Spacer()
                Text(text).font(FBase(style: .button))
                Spacer()
            }
        }
        .buttonStyle(BigButtonStyle(bgColor: bgColor, fgColor: fgColor))
        .disabled(isDisabled)
    }
}
