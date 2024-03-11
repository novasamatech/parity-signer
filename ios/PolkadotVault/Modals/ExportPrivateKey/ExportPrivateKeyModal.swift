//
//  ExportPrivateKeyModal.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 01/09/2022.
//

import SwiftUI

struct ExportPrivateKeyViewModel: Equatable {
    let qrCode: QrData
    let addressFooter: QRCodeAddressFooterViewModel
}

struct ExportPrivateKeyModal: View {
    @State private var animateBackground: Bool = false

    @Binding var isPresentingExportKeysModal: Bool
    @EnvironmentObject var applicationStatePublisher: ApplicationStatePublisher
    let viewModel: ExportPrivateKeyViewModel

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                animateDismissal()
            },
            animateBackground: $animateBackground,
            safeAreaInsetsMode: .full,
            content: {
                VStack(alignment: .center, spacing: 0) {
                    // Header with X button
                    HStack {
                        Localizable.KeyExport.Label.header.text
                            .foregroundColor(.textAndIconsPrimary)
                            .font(PrimaryFont.titleS.font)
                        Spacer()
                        CircleButton(action: animateDismissal)
                    }
                    .padding([.leading], Spacing.large)
                    .padding([.trailing], Spacing.medium)
                    .padding(.bottom, Spacing.medium)
                    // QR Code container
                    VStack(spacing: 0) {
                        AnimatedQRCodeView(
                            viewModel: Binding<AnimatedQRCodeViewModel>.constant(
                                .init(
                                    qrCodes: [viewModel.qrCode.payload],
                                    style: .private
                                )
                            )
                        )
                        .padding(Spacing.stroke)
                        .privacySensitive()
                        QRCodeAddressFooterView(
                            viewModel: viewModel.addressFooter,
                            backgroundColor: .fill6Solid
                        )
                    }
                    .redacted(
                        reason: applicationStatePublisher.applicationState == .inactive ? .privacy : []
                    )
                    .fixedSize(horizontal: false, vertical: true)
                    .strokeContainerBackground()
                    .padding(.horizontal, Spacing.large)
                    // Bottom "Hide" container
                    ExportPrivateKeyAddressFooter(hideAction: animateDismissal)
                        .padding(.horizontal, Spacing.extraSmall)
                        .padding(.vertical, Spacing.medium)
                }
            }
        )
    }

    private func animateDismissal() {
        Animations.chainAnimation(
            animateBackground.toggle(),
            delayedAnimationClosure: { isPresentingExportKeysModal = false }()
        )
    }
}

/// `Hide Secret Key` footer for private key export
private struct ExportPrivateKeyAddressFooter: View {
    private enum Constants {
        static let keyVisibilityTime: CGFloat = 60
    }

    private let hideAction: () -> Void

    init(hideAction: @escaping () -> Void) {
        self.hideAction = hideAction
    }

    var body: some View {
        HStack {
            Localizable.KeyExport.Label.hide.text
                .foregroundColor(.textAndIconsSecondary)
                .font(PrimaryFont.bodyL.font)
            CircularProgressView(
                CircularCountdownModel(
                    counter: Constants.keyVisibilityTime,
                    viewModel: .privateKeyCountdown,
                    onCompletion: hideAction
                )
            )
        }
        .padding(.horizontal, Spacing.large)
        .padding([.top, .bottom], Spacing.extraSmall)
    }
}

#if DEBUG
    struct ExportPrivateKeyModal_Previews: PreviewProvider {
        static var previews: some View {
            Group {
                VStack {
                    ExportPrivateKeyModal(
                        isPresentingExportKeysModal: Binding<Bool>.constant(true),
                        viewModel: .stub
                    )
                }
                .previewDevice("iPhone 11 Pro")
                .background(.gray)
                .preferredColorScheme(.dark)
                VStack {
                    ExportPrivateKeyModal(
                        isPresentingExportKeysModal: Binding<Bool>.constant(true),
                        viewModel: .stub
                    )
                }
                .previewDevice("iPod touch (7th generation)")
                .background(.gray)
                .preferredColorScheme(.dark)
                VStack {
                    ExportPrivateKeyModal(
                        isPresentingExportKeysModal: Binding<Bool>.constant(true),
                        viewModel: .stub
                    )
                }
                .previewDevice("iPhone 8")
                .background(.gray)
                .preferredColorScheme(.dark)
            }
        }
    }
#endif
