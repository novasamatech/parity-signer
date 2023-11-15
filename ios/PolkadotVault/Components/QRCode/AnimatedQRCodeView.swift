//
//  AnimatedQRCodeView.swift
//  Polkadot Vault
//
//  Created by Krzysztof Rodak on 18/10/2022.
//

import QRCode
import SwiftUI

struct AnimatedQRCodeViewModel: Equatable {
    var qrCodes: [[UInt8]]
    var style: QRCodeImageGenerator.Style = .public
}

/// Component that displays given QR code with unifid insets
/// Insets are adjusted for smaller devices
struct AnimatedQRCodeView: View {
    private enum Constants {
        static let animationFps = 0.125
        static let qrCodeWidthForSmallDevices: CGFloat = 216
        static let qrCodeWidthForStandardDevices: CGFloat = 232
        static let qrCodeWidthForLargerDevices: CGFloat = 320
    }

    @Binding var viewModel: AnimatedQRCodeViewModel
    private let qrCodesGenerator: QRCodeImageGenerator
    @State private var images: [UIImage] = []
    @State private var currentImage: UIImage?
    @State private var timer = Timer
        .publish(every: Constants.animationFps, on: .main, in: .common)
        .autoconnect()
    @State private var imagesIterator: IndexingIterator<[UIImage]>

    init(
        viewModel: Binding<AnimatedQRCodeViewModel>,
        qrCodesGenerator: QRCodeImageGenerator = QRCodeImageGenerator(),
        shouldDecode _: Bool = true
    ) {
        _viewModel = viewModel
        self.qrCodesGenerator = qrCodesGenerator
        let images = viewModel.qrCodes.wrappedValue.compactMap {
            qrCodesGenerator.generateQRCode(from: $0, style: viewModel.style.wrappedValue)
        }
        _imagesIterator = State(wrappedValue: images.makeIterator())
        _images = State(wrappedValue: images)
    }

    var body: some View {
        ZStack {
            Image(uiImage: currentImage ?? UIImage())
                .interpolation(.none)
                .resizable()
                .aspectRatio(contentMode: .fit)
                .frame(
                    minWidth: Constants.qrCodeWidthForSmallDevices,
                    idealWidth: Constants.qrCodeWidthForStandardDevices,
                    maxWidth: Constants.qrCodeWidthForLargerDevices,
                    alignment: .center
                )
                .onAppear {
                    start()
                }
                .onDisappear {
                    stop()
                }
                .onReceive(timer) { _ in
                    if let next = imagesIterator.next() {
                        currentImage = next
                    } else {
                        imagesIterator = images.makeIterator()
                        currentImage = imagesIterator.next()
                    }
                }
                .onChange(of: viewModel) { newValue in
                    stop()
                    images = newValue.qrCodes.map { qrCodesGenerator.generateQRCode(from: $0) }
                    imagesIterator = images.makeIterator()
                    start()
                }
        }
        .padding(
            UIScreen.main.bounds.width == DeviceConstants.compactDeviceWidth ? Spacing.large : Spacing.x3Large
        )
        .background(
            RoundedRectangle(cornerRadius: CornerRadius.medium)
                .stroke(.fill12, lineWidth: 1)
                .background(.backgroundSystemLightOnly)
                .cornerRadius(CornerRadius.medium)
        )
    }

    private func stop() {
        timer.upstream.connect().cancel()
    }

    private func start() {
        timer = Timer.publish(every: Constants.animationFps, on: .main, in: .common).autoconnect()
    }
}

#if DEBUG
    struct AnimatedQRCodeView_Previews: PreviewProvider {
        static var previews: some View {
            Group {
                VStack {
                    AnimatedQRCodeView(
                        viewModel: Binding<AnimatedQRCodeViewModel>.constant(.stub)
                    )
                }
                .previewDevice("iPhone 11 Pro")
                .background(.gray)
                .preferredColorScheme(.dark)
                VStack {
                    AnimatedQRCodeView(
                        viewModel: Binding<AnimatedQRCodeViewModel>.constant(.stub)
                    )
                }
                .previewDevice("iPod touch (7th generation)")
                .background(.gray)
                .preferredColorScheme(.dark)
                VStack {
                    AnimatedQRCodeView(
                        viewModel: Binding<AnimatedQRCodeViewModel>.constant(.stub)
                    )
                }
                .previewDevice("iPhone 8")
                .background(.gray)
                .preferredColorScheme(.dark)
            }
        }
    }
#endif
