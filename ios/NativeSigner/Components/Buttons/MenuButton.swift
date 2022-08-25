//
//  MenuButton.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 24/08/2022.
//

import SwiftUI

struct MenuButtonStyle: ButtonStyle {
    let foregroundColor: Color

    func makeBody(configuration: Self.Configuration) -> some View {
        configuration.label
            .padding(Padding.none)
            .foregroundColor(foregroundColor)
            .font(Fontstyle.button.base)
            .frame(height: Heights.menuButton, alignment: .leading)
    }
}

struct MenuButton: View {
    private let action: () -> Void
    private let icon: Image
    private let text: LocalizedStringKey
    private let foregroundColor: Color

    @State var isDisabled: Bool

    init(
        action: @escaping () -> Void,
        icon: Image,
        text: LocalizedStringKey,
        isDisabled: Bool = false,
        foregroundColor: Color = Asset.textAndIconsSecondary.swiftUIColor
    ) {
        self.action = action
        self.icon = icon
        self.text = text
        self.isDisabled = isDisabled
        self.foregroundColor = foregroundColor
    }

    var body: some View {
        Button(action: action) {
            HStack(alignment: .center, spacing: Padding.medium) {
                icon
                    .padding(10)
                Text(text)
            }
        }
        .buttonStyle(MenuButtonStyle(foregroundColor: foregroundColor))
        .disabled(isDisabled)
    }
}

//struct MenuButton_Previews: PreviewProvider {
//    static var previews: some View {
//        VStack(alignment: .leading, spacing: 10) {
//            MenuButton(
//                action: {},
//                icon: Asset.add.swiftUIImage,
//                text: "Short Title"
//            )
//            .padding(10)
//            MenuButton(
//                action: {},
//                icon: Asset.recover.swiftUIImage,
//                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
//            )
//            .padding(10)
//            MenuButton(
//                action: {},
//                icon: Asset.recover.swiftUIImage,
//                text: "Delete",
//                foregroundColor: Asset.accentRed400.swiftUIColor
//            )
//            .padding(10)
//        }
//        .preferredColorScheme(.dark)
//        .previewLayout(.sizeThatFits)
//        VStack(alignment: .leading, spacing: 10) {
//            MenuButton(
//                action: {},
//                icon: Asset.add.swiftUIImage,
//                text: "Short Title"
//            )
//            .padding(10)
//            MenuButton(
//                action: {},
//                icon: Asset.recover.swiftUIImage,
//                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
//            )
//            .padding(10)
//            MenuButton(
//                action: {},
//                icon: Asset.recover.swiftUIImage,
//                text: "Delete",
//                foregroundColor: Asset.accentRed400.swiftUIColor
//            )
//            .padding(10)
//        }
//        .preferredColorScheme(.light)
//        .previewLayout(.sizeThatFits)
//    }
//}
