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
    @StateObject var progressViewModel: ProgressSnackbarViewModel = ProgressSnackbarViewModel()
    @EnvironmentObject private var navigation: NavigationCoordinator
    @Environment(\.safeAreaInsets) private var safeAreaInsets

    var body: some View {
        ZStack {
            // Full screen camera preview
            CameraPreview(session: model.session)
                .onReceive(model.$payload) { payload in
                    viewModel.checkForTransactionNavigation(payload, model: model)
                }
                .onChange(of: model.total) { total in
                    progressViewModel.total = total
                    if total > 1, viewModel.isPresentingProgressSnackbar == false, !viewModel.isScanningMultiple {
                        viewModel.isPresentingProgressSnackbar = true
                    }
                }
                .onChange(of: model.captured) { newValue in
                    progressViewModel.current = newValue
                    UIApplication.shared.isIdleTimerDisabled = newValue > 0
                }
            VStack {
                ZStack(alignment: .bottom) {
                    // Blur overlay
                    Rectangle()
                        .background(.regularMaterial)
                    VStack {
                        HStack(spacing: Spacing.small) {
                            CameraButton(
                                action: { viewModel.isPresented.toggle() },
                                icon: Asset.xmarkButton.swiftUIImage
                            )
                            Spacer()
                            CameraButton(
                                action: {
                                    model.isMultipleTransactionMode.toggle()
                                    viewModel.onScanMultipleTap()
                                },
                                icon: Asset.scanMultiple.swiftUIImage,
                                isPressed: $viewModel.isScanningMultiple
                            )
                            CameraButton(
                                action: { model.toggleTorch() },
                                icon: Asset.torchOff.swiftUIImage,
                                isPressed: $model.isTorchOn
                            )
                        }
                        .padding([.leading, .trailing], Spacing.medium)
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
                            Text(viewModel.header)
                                .font(Fontstyle.titleL.base)
                            Text(viewModel.message)
                                .font(Fontstyle.bodyL.base)
                                .multilineTextAlignment(.center)
                        }
                        .foregroundColor(Asset.accentForegroundText.swiftUIColor)
                        .padding([.leading, .trailing], Spacing.componentSpacer)
                        Spacer()
                    }
                    if viewModel.isScanningMultiple, !model.multipleTransactions.isEmpty {
                        VStack(spacing: 0) {
                            multipleTransactionOverlay
                            Asset.backgroundSecondaryDarkOnly.swiftUIColor
                                .frame(height: safeAreaInsets.bottom)
                        }
                        .transition(.move(edge: .bottom))
                    }
                }
            }
            .onAppear {
                progressViewModel.title = Localizable.Scanner.Label.multipart.string
                progressViewModel.cancelActionTitle = Localizable.Scanner.Action.cancel.string
                progressViewModel.cancelAction = {
                    model.reset()
                    viewModel.isPresentingProgressSnackbar = false
                }
            }
            .bottomProgressSnackbar(progressViewModel, isPresented: $viewModel.isPresentingProgressSnackbar)
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

    var multipleTransactionOverlay: some View {
        HStack(alignment: .center) {
            Text(signText())
                .font(Fontstyle.titleS.base)
                .foregroundColor(Asset.accentForegroundText.swiftUIColor)
                .padding(.top, Spacing.medium)
            Spacer()
            CapsuleButton(
                action: {
                    // TODO: Agree with backend what to do
                },
                icon: Asset.arrowForward.swiftUIImage,
                title: Localizable.Scanner.Action.sign.string
            )
            .padding(.top, Spacing.extraSmall)
        }
        .padding(.leading, Spacing.medium)
        .padding(.trailing, Spacing.extraSmall)
        .frame(height: Heights.bottomBarHeight)
        .background(Asset.backgroundSecondaryDarkOnly.swiftUIColor)
    }

    func signText() -> String {
        let key = Localizable.Scanner.Label.self
        let suffix = (model.multipleTransactions.count > 1 ? key.SignMultiple.Suffix.plural : key.SignMultiple.Suffix.single).string
        return key.signMultiple(model.multipleTransactions.count, suffix)
    }
}

extension CameraView {
    final class ViewModel: ObservableObject {
        @Published var isPresentingTransactionPreview: Bool = false
        @Published var isPresentingProgressSnackbar: Bool = false
        @Published var isScanningMultiple: Bool = false
        @Published var mTransaction: MTransaction?
        @Published var header: String = Localizable.Scanner.Label.Scan.Main.header.string
        @Published var message: String = Localizable.Scanner.Label.Scan.Main.message.string

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

        func onScanMultipleTap() {
            isScanningMultiple.toggle()
            updateTexts()
        }

        private func updateTexts() {
            let key = Localizable.Scanner.Label.Scan.self
            withAnimation {
                header = (isScanningMultiple ? key.Multiple.header : key.Main.header).string
                message = (isScanningMultiple ? key.Multiple.message : key.Main.message).string
            }
        }
    }
}

// struct CameraView_Previews: PreviewProvider {
// static var previews: some View {
// CameraView()
// }
// }
