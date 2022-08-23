//
//  ActionButton.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 23/08/2022.
//

import SwiftUI

struct ActionButtonStyle: ButtonStyle {
    let backgroundColor: Color
    let foregroundColor: Color

    func makeBody(configuration: Self.Configuration) -> some View {
        configuration.label
            .padding(Padding.large)
            .background(backgroundColor)
            .foregroundColor(foregroundColor)
            .cornerRadius(CornerRadius.extraExtraLarge)
            .font(Fontstyle.labelL.base)
            .frame(height: 57, alignment: .center)
    }
}

struct ActionButton: View {
    var action: () -> Void
    var text: LocalizedStringKey
    var style: ActionButtonStyle
    @State var isDisabled: Bool = false

    var body: some View {
        Button(action: action) {
            HStack {
                Text(text)
            }
            .frame(maxWidth: .infinity)
        }
        .buttonStyle(style)
        .disabled(isDisabled)
    }
}
