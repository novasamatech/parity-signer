//
//  ExportMultipleKeysModal.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 18/10/2022.
//

import SwiftUI

struct ExportMultipleKeysModalViewModel: Equatable {
    let selectedItems: [KeySetViewModel]
    let seeds: [SeedNameCard]
}

struct ExportMultipleKeysModal: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                animateDismissal()
            },
            animateBackground: $viewModel.animateBackground,
            ignoredEdges: .bottom,
            content: {
                VStack {
                    // Header with X button
                    header
                        .padding([.leading], Spacing.large)
                        .padding([.trailing], Spacing.medium)
                    ScrollView {
                        VStack(alignment: .center) {
                            // QR Code container
                            VStack(spacing: 0) {
                                AnimatedQRCodeView(viewModel: $viewModel.qrCode)
                                    .padding(0.5)
                                    .fixedSize(horizontal: false, vertical: true)
                                HStack {
                                    Localizable.KeysExport.KeySets.Label.info.text
                                        .frame(maxWidth: .infinity, alignment: .leading)
                                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                                    Spacer().frame(maxWidth: Spacing.medium)
                                    Asset.infoIconBold.swiftUIImage
                                        .foregroundColor(Asset.accentPink300.swiftUIColor)
                                }
                                .padding()
                                .font(Fontstyle.bodyM.base)
                                .background(
                                    RoundedRectangle(cornerRadius: CornerRadius.small)
                                        .stroke(Asset.fill12.swiftUIColor, lineWidth: 1)
                                        .background(Asset.fill6.swiftUIColor)
                                        .cornerRadius(CornerRadius.small)
                                )
                                .padding(.top, Spacing.extraSmall)
                                // Keys list
                                VStack(alignment: .leading, spacing: 0) {
                                    ForEach(
                                        viewModel.viewModel.selectedItems.sorted(by: { $0.keyName < $1.keyName }),
                                        id: \.keyName
                                    ) { keyItem($0, isLast: $0 == viewModel.viewModel.selectedItems.last) }
                                }
                                .padding(.top, Spacing.extraExtraSmall)
                            }
                            .padding(Spacing.extraSmall)
                            .background(
                                RoundedRectangle(cornerRadius: CornerRadius.medium)
                                    .stroke(Asset.fill12.swiftUIColor, lineWidth: 1)
                                    .background(Asset.fill6.swiftUIColor)
                                    .cornerRadius(CornerRadius.medium)
                            )
                            .padding([.leading, .trailing], Spacing.medium)
                        }
                    }
                }
                .onAppear {
                    viewModel.prepareKeysExport()
                }
            }
        )
    }

    private func animateDismissal() {
        Animations.chainAnimation(
            viewModel.animateBackground.toggle(),
            delayedAnimationClosure: viewModel.isPresented.toggle()
        )
    }
}

extension ExportMultipleKeysModal {
    final class ViewModel: ObservableObject {
        private let keysExportService: ExportMultipleKeysService
        let viewModel: ExportMultipleKeysModalViewModel

        @Published var qrCode: AnimatedQRCodeViewModel = .init(qrCodes: [])
        @Published var isShowingKeysExportModal = false
        @Published var animateBackground: Bool = false

        @Binding var isPresented: Bool

        init(
            viewModel: ExportMultipleKeysModalViewModel,
            keysExportService: ExportMultipleKeysService = ExportMultipleKeysService(),
            isPresented: Binding<Bool>
        ) {
            self.viewModel = viewModel
            self.keysExportService = keysExportService
            _isPresented = isPresented
        }

        func prepareKeysExport() {
            keysExportService.exportMultipleKeys(items: viewModel.seeds.map(\.seedName)) { result in
                self.qrCode = (try? result.get()) ?? .init(qrCodes: [])
            }
        }
    }
}

private extension ExportMultipleKeysModal {
    var header: some View {
        HStack {
            Text(headerName)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(Fontstyle.titleS.base)
            Spacer()
            CloseModalButton(action: animateDismissal)
        }
    }

    var headerName: String {
        let localizableKey = Localizable.KeysExport.KeySets.Label.self
        let count = viewModel.viewModel.seeds.count
        let suffix = count == 1 ? localizableKey.Header.Suffix.single.string : localizableKey.Header.Suffix.plural
            .string
        return localizableKey.header(String(count), suffix)
    }

    func keyItem(_ viewModel: KeySetViewModel, isLast: Bool) -> some View {
        VStack(alignment: .leading) {
            Spacer()
            HStack {
                Text(viewModel.keyName)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                Text(" · ")
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                Text(viewModel.derivedKeys ?? "")
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                Spacer()
            }
            .font(Fontstyle.bodyM.base)
            Spacer()
            if !isLast {
                Divider()
            }
        }
        .frame(height: Heights.actionSheetButton)
        .padding([.leading, .trailing], Spacing.extraSmall)
        .background(Color.clear)
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
                .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                .font(Fontstyle.bodyL.base)
            CircularProgressView(
                CircularCountdownModel(
                    counter: Constants.keyVisibilityTime,
                    viewModel: .privateKeyCountdown,
                    onCompletion: hideAction
                )
            )
        }
        .padding([.leading, .trailing], Spacing.large)
        .padding([.top, .bottom], Spacing.extraSmall)
    }
}

#if DEBUG
    struct ExportMultipleKeysModal_Previews: PreviewProvider {
        static var previews: some View {
            Group {
                VStack {
                    ExportMultipleKeysModal(
                        viewModel: .init(
                            viewModel: PreviewData.exampleExportMultipleKeysModal,
                            isPresented: Binding<Bool>.constant(true)
                        )
                    )
                }
                .previewDevice("iPhone 11 Pro")
                .background(.gray)
                .preferredColorScheme(.dark)
                VStack {
                    ExportMultipleKeysModal(
                        viewModel: .init(
                            viewModel: PreviewData.exampleExportMultipleKeysModal,
                            isPresented: Binding<Bool>.constant(true)
                        )
                    )
                }
                .previewDevice("iPod touch (7th generation)")
                .background(.gray)
                .preferredColorScheme(.dark)
                VStack {
                    ExportMultipleKeysModal(
                        viewModel: .init(
                            viewModel: PreviewData.exampleExportMultipleKeysModal,
                            isPresented: Binding<Bool>.constant(true)
                        )
                    )
                }
                .previewDevice("iPhone 8")
                .background(.gray)
                .preferredColorScheme(.dark)
            }
            .environmentObject(NavigationCoordinator())
        }
    }
#endif
