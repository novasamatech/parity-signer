//
//  SignatureReady.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 15.12.2021.
//

import SwiftUI

struct SignatureReady: View {
    var content: MSignatureReady
    let navigationRequest: NavigationRequest
    @State var animateBackground: Bool = false
    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                dismiss()
            },
            animateBackground: $animateBackground
        ) {
            VStack {
                HeaderBar(
                    line1: Localizable.yourSignature.key,
                    line2: Localizable.scanItIntoYourApplication.key
                )
                AnimatedQRCodeView(
                    viewModel: Binding<AnimatedQRCodeViewModel>
                        .constant(.init(qrCodes: content.signatures))
                )
                .aspectRatio(contentMode: .fit)
                .padding(Spacing.small)
                BigButton(text: Localizable.done.key, action: {
                    dismiss()
                })
                .padding(.bottom, Spacing.medium)
            }
            .padding(Spacing.medium)
            .cornerRadius(CornerRadius.extraSmall)
            .background(Asset.bg000.swiftUIColor)
        }
    }

    func dismiss() {
        navigationRequest(.init(action: .goBack))
    }
}

// struct SignatureReady_Previews: PreviewProvider {
// static var previews: some View {
// SignatureReady()
// }
// }
