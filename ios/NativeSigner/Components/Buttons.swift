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
            .padding(10)
            .background(
                RoundedRectangle(
                    cornerRadius: 8,
                    style: .continuous
                )
                .fill(bgColor)
            )
            .foregroundColor(fgColor)
    }
}

struct BigButton: View {
    var text: String
    var isShaded: Bool = false
    var action: ()->Void
    var isDisabled: Bool = false
    
    var body: some View {
        let bgColor = isDisabled
            ? Color("Bg200")
            : isShaded ? Color("Bg300") : Color("Action400")
        let fgColor = isDisabled
            ? Color("Text300")
            : isShaded ? Color("Action400") : Color("Action600")
        
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
