//
//  CameraButton.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 14/10/2022.
//

import SwiftUI

struct CameraButtonStyle: ButtonStyle {
    func makeBody(configuration: Self.Configuration) -> some View {
        configuration.label
            .foregroundColor(Asset.navbarIcon.swiftUIColor)
            .frame(
                width: Heights.navigationButton,
                height: Heights.navigationButton,
                alignment: .center
            )
    }
}

struct CameraButton: View {
    private let action: () -> Void
    private let icon: Image
    @Binding var isPressed: Bool

    init(
        action: @escaping () -> Void,
        icon: Image,
        isPressed: Binding<Bool> = Binding<Bool>.constant(false)
    ) {
        self.action = action
        self.icon = icon
        _isPressed = isPressed
    }

    var body: some View {
        Button(action: action) {
            icon
                .foregroundColor(isPressed ? Asset.accentPink500.swiftUIColor : .white)
        }
        .buttonStyle(CameraButtonStyle())
        .background(isPressed ? Asset.accentForegroundText.swiftUIColor : Asset.fill30LightOnly.swiftUIColor)
        .clipShape(Circle())
    }
}

struct CameraButton_Previews: PreviewProvider {
    static var previews: some View {
        VStack(alignment: .leading, spacing: 10) {
            CameraButton(
                action: {},
                icon: Asset.xmarkButton.swiftUIImage
            )
            CameraButton(
                action: {},
                icon: Asset.torchOff.swiftUIImage
            )
            CameraButton(
                action: {},
                icon: Asset.torchOff.swiftUIImage,
                isPressed: Binding<Bool>.constant(true)
            )
        }
        .preferredColorScheme(.dark)
        .previewLayout(.sizeThatFits)
    }
}
