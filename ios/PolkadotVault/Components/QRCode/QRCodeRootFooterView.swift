//
//  QRCodeRootFooterView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 13/09/2022.
//

import SwiftUI

struct QRCodeRootFooterViewModel: Equatable {
    let keyName: String
    let base58: String
}

/// Footer component for root key to be used below QR code container
/// Provides tap action that presents full `base58` address. Does not provide identicon
struct QRCodeRootFooterView: View {
    @State private var showFullAddress: Bool = false
    private let viewModel: QRCodeRootFooterViewModel

    init(viewModel: QRCodeRootFooterViewModel) {
        self.viewModel = viewModel
    }

    var body: some View {
        VStack(alignment: .leading, spacing: Spacing.extraSmall) {
            Text(Localizable.PublicKeyDetails.Label.keys(viewModel.keyName))
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.bodyM.font)
            HStack(spacing: Spacing.extraExtraSmall) {
                Text(showFullAddress ? viewModel.base58 : viewModel.base58.truncateMiddle())
                    .foregroundColor(.textAndIconsTertiary)
                    .font(PrimaryFont.bodyM.font)
                    .frame(idealWidth: .infinity, alignment: .leading)
                if !showFullAddress {
                    Image(.chevronDown)
                        .foregroundColor(.textAndIconsSecondary)
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
        .padding(.horizontal, Spacing.medium)
        .padding([.top, .bottom], Spacing.medium)
        .fixedSize(horizontal: false, vertical: true)
    }
}

#if DEBUG
    struct QRCodeRootFooterView_Previews: PreviewProvider {
        static var previews: some View {
            Group {
                VStack {
                    Spacer()
                    QRCodeRootFooterView(
                        viewModel: .stub
                    )
                    .background(.fill6Solid)
                    Spacer()
                }
                .background(.white)
                .preferredColorScheme(.dark)
            }
        }
    }
#endif
