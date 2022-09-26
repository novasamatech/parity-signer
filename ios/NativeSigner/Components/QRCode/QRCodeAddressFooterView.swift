//
//  QRCodeAddressFooterView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 13/09/2022.
//

import SwiftUI

struct QRCodeAddressFooterViewModel: Equatable {
    let identicon: [UInt8]
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
}

struct QRCodeAddressFooterView_Previews: PreviewProvider {
    static var previews: some View {
        Group {
            VStack {
                QRCodeAddressFooterView(
                    viewModel: PreviewData.qrCodeAddressFooterViewModel
                )
            }
            .background(.gray)
            .preferredColorScheme(.dark)
        }
    }
}
