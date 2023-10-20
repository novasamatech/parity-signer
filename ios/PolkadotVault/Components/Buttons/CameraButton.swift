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
            .foregroundColor(.navbarIcon)
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
                .foregroundColor(isPressed ? .accentPink500 : .white)
        }
        .buttonStyle(CameraButtonStyle())
        .background(isPressed ? .accentForegroundText : .fill30LightOnly)
        .clipShape(Circle())
    }
}

#if DEBUG
    struct CameraButton_Previews: PreviewProvider {
        static var previews: some View {
            VStack(alignment: .leading, spacing: 10) {
                CameraButton(
                    action: {},
                    icon: Image(.xmarkButton)
                )
                CameraButton(
                    action: {},
                    icon: Image(.torchOff)
                )
                CameraButton(
                    action: {},
                    icon: Image(.torchOff),
                    isPressed: Binding<Bool>.constant(true)
                )
            }
            .preferredColorScheme(.dark)
            .previewLayout(.sizeThatFits)
        }
    }
#endif
