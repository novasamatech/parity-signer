//
//  ExportPrivateKeyModal.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 01/09/2022.
//

import SwiftUI

struct ExportPrivateKeyViewModel: Equatable {
    let qrCode: QRCodeContainerViewModel
    let addressFooter: QRCodeAddressFooterViewModel
}

struct ExportPrivateKeyModal: View {
    @State private var animateBackground: Bool = false

    @Binding var isPresentingExportKeysModal: Bool
    @ObservedObject var navigation: NavigationCoordinator
    let viewModel: ExportPrivateKeyViewModel

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                animateDismissal()
            },
            animateBackground: $animateBackground,
            content: {
                VStack(alignment: .center) {
                    // Header with X button
                    HStack {
                        Localizable.KeyExport.Label.header.text
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            .font(Fontstyle.titleS.base)
                        Spacer()
                        CloseModalButton(action: animateDismissal)
                    }
                    .padding([.leading], Spacing.large)
                    .padding([.trailing], Spacing.medium)
                    // QR Code container
                    VStack(spacing: 0) {
                        QRCodeContainerView(viewModel: viewModel.qrCode)
                        QRCodeAddressFooterView(viewModel: viewModel.addressFooter)
                    }
                    .fixedSize(horizontal: false, vertical: true)
                    .background(
                        RoundedRectangle(cornerRadius: CornerRadius.medium)
                            .stroke(Asset.fill12.swiftUIColor, lineWidth: 1)
                            .background(Asset.fill6.swiftUIColor)
                            .cornerRadius(CornerRadius.medium)
                    )
                    .padding([.leading, .trailing], Spacing.large)
                    // Bottom "Hide" container
                    ExportPrivateKeyAddressFooter(hideAction: animateDismissal)
                        .padding([.leading, .trailing], Spacing.extraSmall)
                }
            }
        )
    }

    private func animateDismissal() {
        Animations.chainAnimation(
            animateBackground.toggle(),
            delayedAnimationClosure: isPresentingExportKeysModal.toggle()
        )
    }
}

/// `Hide Secret Key` footer for private key export
private struct ExportPrivateKeyAddressFooter: View {
    private enum Constants {
        static let keyVisibilityTime: CGFloat = 60
    }

    private var hideAction: () -> Void

    init(hideAction: @escaping () -> Void) {
        self.hideAction = hideAction
    }

    var body: some View {
        HStack {
            Localizable.KeyExport.Label.hide.text
                .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                .font(Fontstyle.bodyL.base)
            CircularProgressView(
                CircularCountdownModel(
                    counter: Constants.keyVisibilityTime,
                    onCompletion: hideAction
                ),
                viewModel: .privateKeyCountdown
            )
        }
        .padding([.leading, .trailing], Spacing.large)
        .padding([.top, .bottom], Spacing.extraSmall)
    }
}

// struct ExportPrivateKeyModal_Previews: PreviewProvider {
//    static var previews: some View {
//        Group {
//            VStack {
//                ExportPrivateKeyModal(
//                    isPresentingExportKeysModal: Binding<Bool>.constant(true),
//                    navigation: NavigationCoordinator(),
//                    viewModel: PreviewData.exampleExportPrivateKey
//                )
//            }
//            .previewDevice("iPhone 11 Pro")
//            .background(.gray)
//            .preferredColorScheme(.dark)
//            VStack {
//                ExportPrivateKeyModal(
//                    isPresentingExportKeysModal: Binding<Bool>.constant(true),
//                    navigation: NavigationCoordinator(),
//                    viewModel: PreviewData.exampleExportPrivateKey
//                )
//            }
//            .previewDevice("iPod touch (7th generation)")
//            .background(.gray)
//            .preferredColorScheme(.dark)
//            VStack {
//                ExportPrivateKeyModal(
//                    isPresentingExportKeysModal: Binding<Bool>.constant(true),
//                    navigation: NavigationCoordinator(),
//                    viewModel: PreviewData.exampleExportPrivateKey
//                )
//            }
//            .previewDevice("iPhone 8")
//            .background(.gray)
//            .preferredColorScheme(.dark)
//        }
//    }
// }
