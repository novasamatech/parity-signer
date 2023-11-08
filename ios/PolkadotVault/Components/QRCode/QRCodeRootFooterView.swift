//
//  QRCodeRootFooterView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 13/09/2022.
//

import SwiftUI

struct QRCodeRootFooterViewModel: Equatable {
    let identicon: Identicon
    let base58: String
}

/// Footer component for root key to be used below QR code container
/// Provides tap action that presents full `base58` address. Does not provide identicon
struct QRCodeRootFooterView: View {
    private let viewModel: QRCodeRootFooterViewModel

    init(viewModel: QRCodeRootFooterViewModel) {
        self.viewModel = viewModel
    }

    var body: some View {
        HStack(spacing: Spacing.small) {
            IdenticonView(
                identicon: viewModel.identicon,
                rowHeight: Heights.identiconInCell
            )
            Text(viewModel.base58.truncateMiddle())
                .foregroundColor(.textAndIconsTertiary)
                .font(PrimaryFont.bodyL.font)
                .frame(idealWidth: .infinity, alignment: .leading)
            Spacer()
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
