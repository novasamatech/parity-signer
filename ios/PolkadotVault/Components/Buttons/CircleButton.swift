//
//  CloseModalButton.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 05/09/2022.
//

import SwiftUI

struct CloseModalButton: View {
    private let action: () -> Void

    init(
        action: @escaping () -> Void
    ) {
        self.action = action
    }

    var body: some View {
        Button(
            action: action,
            label: {
                ZStack {
                    Circle()
                        .frame(width: Sizes.xmarkButtonDiameter, height: Sizes.xmarkButtonDiameter, alignment: .center)
                        .foregroundColor(Asset.fill18.swiftUIColor)
                    Asset.xmarkButton.swiftUIImage
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                }
            }
        )
    }
}

//
// struct CloseModalButton_Previews: PreviewProvider {
//    static var previews: some View {
//        VStack(alignment: .center) {
//            CloseModalButton(action: {})
//        }
//        .preferredColorScheme(.dark)
//        .previewLayout(.sizeThatFits)
//        VStack(alignment: .center) {
//            CloseModalButton(action: {})
//        }
//        .preferredColorScheme(.light)
//        .previewLayout(.sizeThatFits)
//    }
// }
