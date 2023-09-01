//
//  ExportMultipleKeysModal.swift
//  Polkadot Vault
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
        networkLogo = derivedKey.keyData.network.networkLogo
        base58 = derivedKey.viewModel.base58
    }
}

struct ExportMultipleKeysModalViewModel: Equatable {
    let key: KeySummaryViewModel
    let derivedKeys: [DerivedKeyExportModel]
    let count: Int
}

struct ExportMultipleKeysModal: View {
    @StateObject var viewModel: ViewModel

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
            ScrollView(showsIndicators: false) {
                VStack(alignment: .center) {
                    // QR Code container
                    VStack(spacing: 0) {
                        AnimatedQRCodeView(viewModel: $viewModel.qrCode)
                            .padding(Spacing.stroke)
                            .fixedSize(horizontal: false, vertical: true)
                        InfoBoxView(text: Localizable.KeysExport.KeySets.Label.info.string)
                            .padding(.top, Spacing.extraSmall)
                        // Keys list
                        keyList
                    }
                    .padding(Spacing.extraSmall)
                    .strokeContainerBackground()
                    .padding(.horizontal, Spacing.medium)
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
            QRCodeRootFooterView(viewModel: .init(viewModel.viewModel.key))
            if !viewModel.viewModel.derivedKeys.isEmpty {
                Divider()
            }
            ForEach(
                viewModel.viewModel.derivedKeys.sorted(by: { $0.viewModel.path < $1.viewModel.path }),
                id: \.id
            ) {
                ExportDerivedKeyView(dataModel: $0, backgroundColor: Asset.fill6Solid.swiftUIColor)
                if $0 != viewModel.viewModel.derivedKeys.last {
                    Divider()
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
}

private struct ExportDerivedKeyView: View {
    @State private var showFullAddress: Bool = false
    private let dataModel: DerivedKeyExportModel
    private let backgroundColor: Color

    init(
        dataModel: DerivedKeyExportModel,
        backgroundColor: Color
    ) {
        self.dataModel = dataModel
        self.backgroundColor = backgroundColor
    }

    var body: some View {
        VStack(alignment: .leading, spacing: Spacing.small) {
            HStack(alignment: .center, spacing: Spacing.extraExtraSmall) {
                VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                    fullPath
                        .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                        .font(PrimaryFont.captionM.font)
                    Text(dataModel.viewModel.rootKeyName)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(PrimaryFont.bodyM.font)
                }
                Spacer()
                NetworkIdenticon(
                    identicon: dataModel.viewModel.identicon,
                    network: dataModel.keyData.network.networkLogo,
                    background: backgroundColor,
                    size: Heights.identiconInCell
                )
            }
            HStack(alignment: .center, spacing: Spacing.extraExtraSmall) {
                Group {
                    Text(showFullAddress ? dataModel.viewModel.base58 : dataModel.viewModel.base58.truncateMiddle())
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(PrimaryFont.bodyL.font)
                        .frame(idealWidth: .infinity, alignment: .leading)
                    if !showFullAddress {
                        Asset.chevronDown.swiftUIImage
                            .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                            .padding(.leading, Spacing.extraExtraSmall)
                    }
                }
                .onTapGesture {
                    withAnimation {
                        showFullAddress = true
                    }
                }
                Spacer()
                NetworkCapsuleView(network: dataModel.viewModel.network)
            }
        }
        .padding(Spacing.medium)
        .fixedSize(horizontal: false, vertical: true)
    }

    /// String interpolation for SFSymbols is a bit unstable if creating `String` inline by using conditional logic or
    /// `appending` from `StringProtocol`. Hence less DRY approach and dedicated function to wrap that
    private var fullPath: Text {
        dataModel.viewModel.hasPassword ?
            Text(
                "\(dataModel.viewModel.path)\(Localizable.Shared.Label.passwordedPathDelimeter.string)\(Image(.lock))"
            ) :
            Text(dataModel.viewModel.path)
    }
}

private extension ExportMultipleKeysModal {
    var headerName: String {
        let localizableKey = Localizable.KeysExport.KeySets.Label.self
        let count = viewModel.viewModel.count
        let suffix = count == 1 ? localizableKey.Header.Suffix.single.string : localizableKey.Header.Suffix.plural
            .string
        return localizableKey.header(String(count), suffix)
    }

    func animateDismissal() {
        Animations.chainAnimation(
            viewModel.animateBackground.toggle(),
            delayedAnimationClosure: { self.viewModel.isPresented = false }()
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
                            viewModel: .stub,
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
                            viewModel: .stub,
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
                            viewModel: .stub,
                            isPresented: Binding<Bool>.constant(true)
                        )
                    )
                }
                .previewDevice("iPhone 8")
                .background(.gray)
                .preferredColorScheme(.dark)
            }
        }
    }
#endif
