//
//  CameraView.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 19.7.2021.
//

import AVFoundation
import SwiftUI

struct CameraView: View {
    @StateObject var model = CameraService()
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator
    @Environment(\.safeAreaInsets) private var safeAreaInsets

    var body: some View {
        ZStack {
            // Full screen camera preview
            CameraPreview(session: model.session)
                .onReceive(model.$payload) { payload in
                    viewModel.checkForTransactionNavigation(payload, model: model)
                }
                .onChange(of: model.captured) { newValue in
                    UIApplication.shared.isIdleTimerDisabled = newValue > 0
                }
            VStack {
                ZStack {
                    // Blur overlay
                    Rectangle()
                        .background(.regularMaterial)
                    VStack {
                        HStack {
                            CameraButton(
                                action: {
                                    viewModel.isPresented.toggle()
                                },
                                icon: Asset.xmarkButton.swiftUIImage
                            )
                            Spacer()
                            CapsuleButton(
                                action: {},
                                icon: Asset.scanMultiple.swiftUIImage,
                                title: Localizable.Scanner.Action.multiple.string,
                                isPressed: false
                            )
                            Spacer()
                            CameraButton(
                                action: {
                                    model.toggleTorch()
                                },
                                icon: Asset.torchOff.swiftUIImage,
                                isPressed: $model.isTorchOn
                            )
                        }
                        .padding([.leading, .trailing], Spacing.extraExtraLarge)
                        .padding(.top, Spacing.medium + safeAreaInsets.top)
                        // Camera cutout
                        ZStack {
                            RoundedRectangle(cornerRadius: CornerRadius.qrCodeScanner)
                                .aspectRatio(1.0, contentMode: .fit)
                                .blendMode(.destinationOut)
                                .overlay(
                                    Asset.cameraOverlay.swiftUIImage
                                        .resizable(resizingMode: .stretch)
                                )
                        }
                        .padding([.leading, .trailing], Spacing.extraExtraLarge)
                        .padding([.top, .bottom], Spacing.componentSpacer)
                        // Text description
                        VStack(spacing: Spacing.small) {
                            Localizable.Scanner.Label.Scan.header.text
                                .font(Fontstyle.titleL.base)
                            Localizable.Scanner.Label.Scan.message.text
                                .font(Fontstyle.bodyL.base)
                                .multilineTextAlignment(.center)
                        }
                        .foregroundColor(Asset.accentForegroundText.swiftUIColor)
                        .padding([.leading, .trailing], Spacing.componentSpacer)
                        Spacer()
                    }
                }
//                .compositingGroup()
            }
            if model.total > 1 {
                MenuStack {
                    HeadingOverline(text: Localizable.CameraView.parsingMultidata.key)
                        .padding(.top, Spacing.small)
                    ProgressView(value: min(Float(model.captured) / Float(model.total), 1))
                        .border(Asset.crypto400.swiftUIColor)
                        .foregroundColor(Asset.crypto400.swiftUIColor)
                        .padding(.vertical, Spacing.extraSmall)
                    Text(Localizable.Scanner.Label.progress(model.captured, model.total))
                        .font(Fontstyle.subtitle1.base)
                        .foregroundColor(Asset.text600.swiftUIColor)
                    Localizable.pleaseHoldStill.text
                        .font(Fontstyle.subtitle2.base)
                        .foregroundColor(Asset.text400.swiftUIColor)
                    MenuButtonsStack {
                        BigButton(
                            text: Localizable.CameraView.startOver.key,
                            isShaded: true,
                            action: {
                                model.reset()
                            }
                        )
                    }
                }
            }
        }
        .ignoresSafeArea(edges: [.top, .bottom])
        .onAppear {
            model.configure()
            viewModel.use(navigation: navigation)
        }
        .onDisappear {
            UIApplication.shared.isIdleTimerDisabled = false
            model.shutdown()
        }

        .background(Asset.bg100.swiftUIColor)
        .fullScreenCover(
            isPresented: $viewModel.isPresentingTransactionPreview,
            onDismiss: { model.start() }
        ) {
            if let transaction = viewModel.mTransaction {
                TransactionPreview(
                    viewModel: .init(
                        isPresented: $viewModel.isPresentingTransactionPreview,
                        content: [transaction]
                    )
                )
            } else {
                EmptyView()
            }
        }
    }
}

extension CameraView {
    final class ViewModel: ObservableObject {
        @Published var isPresentingTransactionPreview: Bool = false
        @Published var mTransaction: MTransaction?
        @Binding var isPresented: Bool
        private weak var navigation: NavigationCoordinator!

        init(
            isPresented: Binding<Bool>
        ) {
            _isPresented = isPresented
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func checkForTransactionNavigation(_ payload: String?, model: CameraService) {
            guard payload != nil, !isPresentingTransactionPreview else { return }
            let actionResult = navigation.performFake(
                navigation: .init(
                    action: .transactionFetched,
                    details: payload
                )
            )
            if case let .transaction(value) = actionResult.screenData {
                mTransaction = value
                isPresentingTransactionPreview = true
                model.shutdown()
            }
        }
    }
}

// struct CameraView_Previews: PreviewProvider {
// static var previews: some View {
// CameraView()
// }
// }
