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
                    if total <= 1 {
                        viewModel.isPresentingProgressSnackbar = false
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
                                    viewModel.onScanMultipleTap(model: model)
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
                .compositingGroup()
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
            onDismiss: {
                viewModel.isInTransactionProgress = false
                model.multipleTransactions = []
                model.start()
            }
        ) {
            TransactionPreview(
                viewModel: .init(
                    isPresented: $viewModel.isPresentingTransactionPreview,
                    content: viewModel.transactions,
                    signature: viewModel.signature
                )
            )
        }
        .fullScreenCover(
            isPresented: $viewModel.isPresentingEnterPassword,
            onDismiss: {
                if viewModel.shouldPresentError {
                    viewModel.isPresentingError = true
                    return
                }
                viewModel.isInTransactionProgress = false
                viewModel.enterPassword = nil
                model.start()
            }
        ) {
            EnterPasswordModal(
                viewModel: .init(
                    isPresented: $viewModel.isPresentingEnterPassword,
                    isErrorPresented: $viewModel.shouldPresentError,
                    dataModel: $viewModel.enterPassword,
                    signature: $viewModel.signature
                )
            )
            .clearModalBackground()
        }
        .fullScreenCover(
            isPresented: $viewModel.isPresentingError,
            onDismiss: {
                viewModel.isInTransactionProgress = false
                viewModel.enterPassword = nil
                model.start()
            }
        ) {
            ErrorBottomModal(
                viewModel: .signingForgotPassword(),
                isShowingBottomAlert: $viewModel.isPresentingError
            )
            .clearModalBackground()
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
                    viewModel.onMultipleTransactionSign(model.multipleTransactions, model: model)
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
        let suffix = (
            model.multipleTransactions.count > 1 ? key.SignMultiple.Suffix.plural : key.SignMultiple.Suffix
                .single
        ).string
        return key.signMultiple(model.multipleTransactions.count, suffix)
    }
}

extension CameraView {
    final class ViewModel: ObservableObject {
        // Overlay presentation
        @Published var isPresentingProgressSnackbar: Bool = false
        @Published var isScanningMultiple: Bool = false
        @Published var header: String = Localizable.Scanner.Label.Scan.Main.header.string
        @Published var message: String = Localizable.Scanner.Label.Scan.Main.message.string
        // Modal presentation
        @Published var isPresentingTransactionPreview: Bool = false
        @Published var isPresentingEnterPassword: Bool = false
        @Published var shouldPresentError: Bool = false
        @Published var isPresentingError: Bool = false
        @Published var isInTransactionProgress: Bool = false

        // Data models for modals
        @Published var transactions: [MTransaction] = []
        @Published var signature: MSignatureReady?
        @Published var enterPassword: MEnterPassword!

        @Binding var isPresented: Bool
        private weak var navigation: NavigationCoordinator!
        private let seedsMediator: SeedsMediating

        init(
            isPresented: Binding<Bool>,
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
        ) {
            _isPresented = isPresented
            self.seedsMediator = seedsMediator
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func checkForTransactionNavigation(_ payload: String?, model _: CameraService) {
            guard payload != nil, !isInTransactionProgress else { return }
            isInTransactionProgress = true
            let actionResult = navigation.performFake(
                navigation: .init(
                    action: .transactionFetched,
                    details: payload
                )
            )
            if case let .transaction(transactions) = actionResult.screenData {
                let firstTransaction = transactions.first
                switch firstTransaction?.ttype {
                case .sign:
                    let seedName = firstTransaction?.authorInfo?.address.seedName ?? ""
                    let seedPhrase = seedsMediator.getSeed(seedName: seedName)
                    let actionResult = navigation.performFake(
                        navigation:
                        .init(
                            action: .goForward,
                            details: "",
                            seedPhrase: seedPhrase
                        )
                    )
                    self.transactions = transactions
                    if case let .signatureReady(value) = actionResult.modalData {
                        signature = value
                        isPresentingTransactionPreview = true
                    }
                    if case let .enterPassword(value) = actionResult.modalData {
                        enterPassword = value
                        isPresentingEnterPassword = true
                    }
                default:
                    self.transactions = transactions
                    isPresentingTransactionPreview = true
                }
            }
        }

        func onMultipleTransactionSign(_ payloads: [String], model _: CameraService) {
            var transactions: [MTransaction] = []
            for payload in payloads {
                let actionResult = navigation.performFake(
                    navigation: .init(
                        action: .transactionFetched,
                        details: payload
                    )
                )
                if case let .transaction(value) = actionResult.screenData {
                    transactions += value
                }
            }
            self.transactions = transactions
            isPresentingTransactionPreview = true
        }

        func onScanMultipleTap(model: CameraService) {
            model.multipleTransactions = []
            model.isMultipleTransactionMode.toggle()
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
