//
//  QRCodeAddressFooterView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 13/09/2022.
//

import SwiftUI

struct QRCodeAddressFooterViewModel: Equatable {
    let identicon: Identicon
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
            HStack(alignment: .center, spacing: Spacing.extraExtraSmall) {
                Group {
                    Text(showFullAddress ? viewModel.base58 : viewModel.base58.truncateMiddle())
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
                NetworkIdenticon(
                    identicon: viewModel.identicon,
                    network: viewModel.networkLogo,
                    background: backgroundColor,
                    size: Heights.identiconInCell
                )
            }
        }
        .padding(Spacing.medium)
        .fixedSize(horizontal: false, vertical: true)
    }
}

#if DEBUG
    struct QRCodeAddressFooterView_Previews: PreviewProvider {
        static var previews: some View {
            VStack {
                Spacer()
                QRCodeAddressFooterView(
                    viewModel: .stub,
                    backgroundColor: .fill6Solid
                )
                Spacer()
                QRCodeAddressFooterView(
                    viewModel: .stub,
                    backgroundColor: .fill12
                )
                Spacer()
                QRCodeAddressFooterView(
                    viewModel: .stub,
                    backgroundColor: .backgroundSecondary
                )
                Spacer()
            }
            .background(.white)
            .preferredColorScheme(.dark)
        }
    }
#endif
