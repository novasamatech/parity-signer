//
//  ExportPrivateKeyModal.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 01/09/2022.
//

import SwiftUI

struct ExportPrivateKeyViewModel: Equatable {
    let identicon: [UInt8]
    let qrCode: [UInt8]
    let path: String
    let network: String
    let base58: String
}

struct ExportPrivateKeyModal: View {
    private enum Constants {
        static let compactDeviceWidth: CGFloat = 320
        static let qrCodeWidthForSmallDevices: CGFloat = 216
        static let qrCodeWidthForStandardDevices: CGFloat = 232
    }

    @State private var animateBackground: Bool = false
    @State private var showFullAddress: Bool = false

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
                        // QR Code
                        ZStack {
                            Image(uiImage: UIImage(data: Data(viewModel.qrCode)) ?? UIImage())
                                .resizable()
                                .aspectRatio(contentMode: .fit)
                                .frame(
                                    minWidth: Constants.qrCodeWidthForSmallDevices,
                                    idealWidth: Constants.qrCodeWidthForStandardDevices,
                                    maxWidth: Constants.qrCodeWidthForStandardDevices,
                                    alignment: .center
                                )
                        }
                        .frame(maxWidth: .infinity, alignment: .center)
                        .padding(
                            UIScreen.main.bounds.width == Constants.compactDeviceWidth ? Spacing.large : Spacing
                                .extraExtraLarge
                        )
                        .background(.white)
                        .cornerRadius(CornerRadius.medium)

                        // QR Footer
                        HStack(spacing: Spacing.small) {
                            VStack {
                                Identicon(identicon: viewModel.identicon, rowHeight: Heights.identiconInCell)
                                    .padding(.top, Spacing.extraExtraSmall)
                            }
                            VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                                Text(viewModel.network)
                                    .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                                    .font(Fontstyle.captionM.base)
                                Text(viewModel.path)
                                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                                    .font(Fontstyle.bodyM.base)
                                HStack(spacing: Spacing.extraExtraSmall) {
                                    Asset.derivedKeyAddress.swiftUIImage
                                    Text(showFullAddress ? viewModel.base58 : viewModel.base58.truncateMiddle())
                                        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                                        .font(Fontstyle.bodyM.base)
                                        .frame(idealWidth: .infinity, alignment: .leading)
                                    Asset.chevronDown.swiftUIImage
                                        .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                                        .rotationEffect(Angle(degrees: showFullAddress ? 180 : 0))
                                        .padding(.leading, Spacing.extraExtraSmall)
                                    Spacer()
                                }
                                .onTapGesture {
                                    withAnimation {
                                        showFullAddress.toggle()
                                    }
                                }
                            }
                        }
                        .padding([.leading, .trailing], Spacing.medium)
                        .padding([.top, .bottom], Spacing.medium)
                        .fixedSize(horizontal: false, vertical: true)
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
