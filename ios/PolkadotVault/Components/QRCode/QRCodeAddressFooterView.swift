//
//  QRCodeAddressFooterView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 13/09/2022.
//

import SwiftUI

struct QRCodeAddressFooterViewModel: Equatable {
    let identicon: [UInt8]
    let rootKeyName: String
    let path: String
    let hasPassword: Bool
    let network: String
    let networkLogo: String
    let base58: String
}

/// Footer component to be used below QR code container
/// Provides tap action that presents full `base58` address
struct QRCodeAddressFooterView: View {
    @State private var showFullAddress: Bool = false
    private let viewModel: QRCodeAddressFooterViewModel
    private let backgroundColor: Color

    init(
        viewModel: QRCodeAddressFooterViewModel,
        backgroundColor: Color
    ) {
        self.viewModel = viewModel
        self.backgroundColor = backgroundColor
    }

    var body: some View {
        VStack(spacing: Spacing.small) {
            HStack(alignment: .center, spacing: Spacing.small) {
                VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                    pathView()
                    Text(viewModel.rootKeyName)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(PrimaryFont.bodyM.font)
                }
                Spacer()
                NetworkIdenticon(
                    identicon: viewModel.identicon,
                    network: viewModel.networkLogo,
                    background: backgroundColor,
                    size: Heights.identiconInCell
                )
            }
            HStack(alignment: .top, spacing: Spacing.small) {
                VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                    HStack(spacing: Spacing.extraExtraSmall) {
                        Text(showFullAddress ? viewModel.base58 : viewModel.base58.truncateMiddle())
                            .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                            .font(PrimaryFont.bodyM.font)
                            .frame(idealWidth: .infinity, alignment: .leading)
                        if !showFullAddress {
                            Asset.chevronDown.swiftUIImage
                                .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                                .padding(.leading, Spacing.extraExtraSmall)
                        }
                        Spacer()
                    }
                    .onTapGesture {
                        withAnimation {
                            showFullAddress = true
                        }
                    }
                }
                NetworkCapsuleView(network: viewModel.network)
            }
        }
        .padding(Spacing.medium)
        .fixedSize(horizontal: false, vertical: true)
    }

    /// String interpolation for SFSymbols is a bit unstable if creating `String` inline by using conditional logic or
    /// `appending` from `StringProtocol`. Hence less DRY approach and dedicated function to wrap that
    private var fullPath: Text {
        viewModel.hasPassword ?
            Text(
                "\(viewModel.path)\(Localizable.Shared.Label.passwordedPathDelimeter.string)\(Image(.lock))"
            ) :
            Text(viewModel.path)
    }

    @ViewBuilder
    private func pathView() -> some View {
        if viewModel.path.isEmpty {
            EmptyView()
        } else {
            fullPath
                .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                .font(PrimaryFont.bodyM.font)
        }
    }
}

#if DEBUG
    struct QRCodeAddressFooterView_Previews: PreviewProvider {
        static var previews: some View {
            Group {
                VStack {
                    Spacer()
                    QRCodeAddressFooterView(
                        viewModel: PreviewData.qrCodeAddressFooterViewModel,
                        backgroundColor: Asset.fill6Solid.swiftUIColor
                    )
                    Spacer()
                    QRCodeAddressFooterView(
                        viewModel: PreviewData.qrCodeAddressFooterViewModelNoPath,
                        backgroundColor: Asset.fill12.swiftUIColor
                    )
                    Spacer()
                    QRCodeAddressFooterView(
                        viewModel: PreviewData.qrCodeAddressFooterViewModelVeryLongPath,
                        backgroundColor: Asset.backgroundSecondary.swiftUIColor
                    )
                    Spacer()
                }
                .background(.white)
                .preferredColorScheme(.dark)
            }
        }
    }
#endif
