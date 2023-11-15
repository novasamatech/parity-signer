//
//  RootKeyDetailsModal.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 28/12/2022.
//

import SwiftUI

struct RootKeyDetailsModal: View {
    @StateObject var viewModel: ViewModel
    @State private var showFullAddress: Bool = false

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                animateDismissal()
            },
            animateBackground: $viewModel.animateBackground,
            safeAreaInsetsMode: .partial,
            content: {
                VStack(alignment: .leading, spacing: 0) {
                    // Content
                    VStack(alignment: .leading, spacing: 0) {
                        VStack(spacing: 0) {
                            // QR Code container
                            AnimatedQRCodeView(
                                viewModel: $viewModel.qrCode
                            )
                            .padding(Spacing.stroke)
                            // QR Code footer
                            VStack(spacing: Spacing.small) {
                                HStack(alignment: .center, spacing: Spacing.extraExtraSmall) {
                                    Group {
                                        Text(
                                            showFullAddress ? viewModel.renderable.base58 : viewModel.renderable.base58
                                                .truncateMiddle()
                                        )
                                        .foregroundColor(.textAndIconsPrimary)
                                        .font(PrimaryFont.bodyL.font)
                                        .frame(idealWidth: .infinity, alignment: .leading)
                                        if !showFullAddress {
                                            Image(.chevronDown)
                                                .foregroundColor(.textAndIconsSecondary)
                                                .padding(.leading, Spacing.extraExtraSmall)
                                        }
                                    }
                                    .onTapGesture {
                                        withAnimation {
                                            showFullAddress = true
                                        }
                                    }
                                    Spacer()
                                    if let identicon = viewModel.renderable.identicon {
                                        IdenticonView(
                                            identicon: identicon,
                                            rowHeight: Heights.identiconInCell
                                        )
                                    }
                                }
                            }
                            .padding(.horizontal, Spacing.medium)
                            .padding(.top, Spacing.medium)
                            .padding(.bottom, Spacing.extraSmall)
                            .fixedSize(horizontal: false, vertical: true)
                        }
                        .padding(Spacing.extraSmall)
                        .strokeContainerBackground()
                    }
                    .padding(.horizontal, Spacing.medium)
                    .padding(.top, Spacing.medium)
                    // Close
                    SecondaryButton(
                        action: animateDismissal(),
                        text: Localizable.KeyDetails.Root.Action.close.key
                    )
                    .padding(Spacing.large)
                }
                .onAppear {
                    viewModel.onAppear()
                }
            }
        )
    }

    private func animateDismissal() {
        Animations.chainAnimation(
            viewModel.animateBackground.toggle(),
            delayedAnimationClosure: { viewModel.isPresented = false }()
        )
    }
}

extension RootKeyDetailsModal {
    struct Renderable {
        let seedName: String
        let identicon: Identicon?
        let base58: String
    }

    final class ViewModel: ObservableObject {
        private let keyExportService: ExportKeySetService
        let renderable: Renderable

        @Published var qrCode: AnimatedQRCodeViewModel = .init(qrCodes: [])
        @Published var isShowingKeysExportModal = false
        @Published var animateBackground: Bool = false

        @Binding var isPresented: Bool

        init(
            renderable: Renderable,
            keyExportService: ExportKeySetService = ExportKeySetService(),
            isPresented: Binding<Bool>
        ) {
            self.renderable = renderable
            self.keyExportService = keyExportService
            _isPresented = isPresented
        }

        func onAppear() {
            keyExportService.exportRoot(
                seedName: renderable.seedName
            ) { result in
                self.qrCode = (try? result.get()) ?? .init(qrCodes: [])
            }
        }
    }
}

#if DEBUG
    struct RootKeyDetailsModal_Previews: PreviewProvider {
        static var previews: some View {
            RootKeyDetailsModal(
                viewModel: .init(
                    renderable: .init(
                        seedName: "Key name",
                        identicon: .stubJdenticon,
                        base58: "5CfLC887VYVLN6gG5rmp6wyUoXQYVQxEwNekdCbUUphnnQgW"
                    ),
                    isPresented: .constant(true)
                )
            )
            .previewLayout(.sizeThatFits)
            .preferredColorScheme(.dark)
        }
    }
#endif
