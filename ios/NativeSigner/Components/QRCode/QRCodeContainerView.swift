//
//  QRCodeContainerView.swift
//  NativeSigner
//
//  Created by Krzysztof Rodak on 13/09/2022.
//

import SwiftUI

struct QRCodeContainerViewModel: Equatable {
    let qrCode: [UInt8]
}

/// Component that displays given QR code with unifid insets
/// Insets are adjusted for smaller devices
struct QRCodeContainerView: View {
    private enum Constants {
        static let compactDeviceWidth: CGFloat = 320
        static let qrCodeWidthForSmallDevices: CGFloat = 216
        static let qrCodeWidthForStandardDevices: CGFloat = 232
    }

    private let viewModel: QRCodeContainerViewModel

    init(viewModel: QRCodeContainerViewModel) {
        self.viewModel = viewModel
    }

    var body: some View {
        ZStack {
            Image(uiImage: UIImage(data: Data(viewModel.qrCode)) ?? UIImage())
                .resizable()
                .aspectRatio(contentMode: .fit)
                .frame(
                    minWidth: Constants.qrCodeWidthForSmallDevices,
                    idealWidth: Constants.qrCodeWidthForStandardDevices,
                    maxWidth: Constants.qrCodeWidthForStandardDevices,
                    alignment: .center
                )
        }
        .frame(maxWidth: .infinity, alignment: .center)
        .padding(
            UIScreen.main.bounds.width == Constants.compactDeviceWidth ? Spacing.large : Spacing
                .extraExtraLarge
        )
        .background(.white)
        .cornerRadius(CornerRadius.medium)
    }
}

struct QRCodeContainerView_Previews: PreviewProvider {
    static var previews: some View {
        Group {
            VStack {
                QRCodeContainerView(
                    viewModel: PreviewData.qrCodeContainerViewModel
                )
            }
            .previewDevice("iPhone 11 Pro")
            .background(.gray)
            .preferredColorScheme(.dark)
            VStack {
                QRCodeContainerView(
                    viewModel: PreviewData.qrCodeContainerViewModel
                )
            }
            .previewDevice("iPod touch (7th generation)")
            .background(.gray)
            .preferredColorScheme(.dark)
            VStack {
                QRCodeContainerView(
                    viewModel: PreviewData.qrCodeContainerViewModel
                )
            }
            .previewDevice("iPhone 8")
            .background(.gray)
            .preferredColorScheme(.dark)
        }
    }
}
