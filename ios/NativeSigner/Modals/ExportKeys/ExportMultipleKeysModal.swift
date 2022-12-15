//
//  ExportMultipleKeysModal.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 18/10/2022.
//

import SwiftUI

extension QRCodeRootFooterViewModel {
    init(_ keySummary: KeySummaryViewModel) {
        keyName = keySummary.keyName
        base58 = keySummary.base58
    }
}

extension QRCodeAddressFooterViewModel {
    init(_ derivedKey: DerivedKeyExportModel) {
        identicon = derivedKey.viewModel.identicon
        rootKeyName = derivedKey.viewModel.rootKeyName
        path = derivedKey.viewModel.path
        network = derivedKey.keyData.network.networkTitle
        base58 = derivedKey.viewModel.base58
    }
}

struct ExportMultipleKeysModalViewModel: Equatable {
    enum SelectedItems: Equatable {
        case keySets([KeySetViewModel])
        case keys(key: KeySummaryViewModel, derivedKeys: [DerivedKeyExportModel])
    }

    let selectedItems: SelectedItems
    let seedNames: [String]
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
            content: { content }
        )
    }

    var content: some View {
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
                        InfoBoxView(text: Localizable.KeysExport.KeySets.Label.info.string)
                            .padding(.top, Spacing.extraSmall)
                        // Keys list
                        keyList
                    }
                    .padding(Spacing.extraSmall)
                    .strokeContainerBackground()
                    .padding([.leading, .trailing], Spacing.medium)
                }
                .padding(.bottom, Spacing.medium)
            }
        }
        .onAppear {
            viewModel.prepareKeysExport()
        }
    }

    var keyList: some View {
        LazyVStack(alignment: .leading, spacing: 0) {
            switch viewModel.viewModel.selectedItems {
            case let .keySets(keySets):
                ForEach(
                    keySets.sorted(by: { $0.keyName < $1.keyName }),
                    id: \.id
                ) { keyItem($0, isLast: $0 == keySets.last) }
            case let .keys(key, derivedKeys):
                QRCodeRootFooterView(viewModel: .init(key))
                Divider()
                ForEach(
                    derivedKeys.sorted(by: { $0.viewModel.path < $1.viewModel.path }),
                    id: \.id
                ) {
                    QRCodeAddressFooterView(viewModel: .init($0))
                    if $0 != derivedKeys.last {
                        Divider()
                    }
                }
            }
        }
        .padding(.top, Spacing.extraExtraSmall)
    }
}

private extension ExportMultipleKeysModal {
    var header: some View {
        HStack {
            Text(headerName)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.titleS.font)
            Spacer()
            CloseModalButton(action: animateDismissal)
        }
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
            .font(PrimaryFont.bodyM.font)
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
                .font(PrimaryFont.bodyL.font)
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

private extension ExportMultipleKeysModal {
    var headerName: String {
        let localizableKey = Localizable.KeysExport.KeySets.Label.self
        let count = viewModel.viewModel.seedNames.count
        let suffix = count == 1 ? localizableKey.Header.Suffix.single.string : localizableKey.Header.Suffix.plural
            .string
        return localizableKey.header(String(count), suffix)
    }

    func animateDismissal() {
        Animations.chainAnimation(
            viewModel.animateBackground.toggle(),
            delayedAnimationClosure: viewModel.isPresented.toggle()
        )
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
