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

