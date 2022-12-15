//
//  QRCodeAddressFooterView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 13/09/2022.
//

import SwiftUI

struct QRCodeAddressFooterViewModel: Equatable {
    let identicon: [UInt8]
    let rootKeyName: String
    let path: String
    let network: String
    let base58: String
}

/// Footer component to be used below QR code container
/// Provides tap action that presents full `base58` address
struct QRCodeAddressFooterView: View {
    @State private var showFullAddress: Bool = false
    private let viewModel: QRCodeAddressFooterViewModel

    init(viewModel: QRCodeAddressFooterViewModel) {
        self.viewModel = viewModel
    }

    var body: some View {
        VStack(spacing: Spacing.small) {
            HStack(alignment: .center, spacing: Spacing.small) {
                VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                    Text(viewModel.rootKeyName)
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(PrimaryFont.bodyM.font)
                    Text(viewModel.path)
                        .foregroundColor(Asset.textAndIconsSecondary.swiftUIColor)
                        .font(PrimaryFont.bodyM.font)
                }
                Spacer()
                Identicon(identicon: viewModel.identicon, rowHeight: Heights.identiconInCell)
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
                Text(viewModel.network)
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                    .font(PrimaryFont.captionM.font)
                    .padding([.top, .bottom], Spacing.extraExtraSmall)
                    .padding([.leading, .trailing], Spacing.extraSmall)
                    .background(Asset.fill12.swiftUIColor)
                    .clipShape(Capsule())
            }
        }
        .padding(Spacing.medium)
        .fixedSize(horizontal: false, vertical: true)
    }
}

struct QRCodeAddressFooterView_Previews: PreviewProvider {
    static var previews: some View {
        Group {
            VStack {
                Spacer()
                QRCodeAddressFooterView(
                    viewModel: PreviewData.qrCodeAddressFooterViewModel
                )
                .background(Asset.fill6Solid.swiftUIColor)
                Spacer()
            }
            .background(.white)
            .preferredColorScheme(.dark)
        }
    }
}
