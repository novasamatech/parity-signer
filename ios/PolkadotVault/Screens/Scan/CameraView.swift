//
//  CameraView.swift
//  Polkadot Vault
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
                    DispatchQueue.main.async {
                        viewModel.checkForTransactionNavigation(payload)
                    }
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
                }
                .onChange(of: model.requestPassword) { newValue in
                    guard newValue else { return }
                    viewModel.presentBananaSplitPassword()
                }
            VStack {
                ZStack(alignment: .bottom) {
                    // Blur overlay
                    Rectangle()
                        .background(.regularMaterial)
                    VStack {
                        HStack(spacing: Spacing.small) {
                            CameraButton(
                                action: viewModel.dismissView,
                                icon: Asset.xmarkButton.swiftUIImage
                            )
                            Spacer()
                            CameraButton(
                                action: { model.toggleTorch() },
                                icon: Asset.torchOff.swiftUIImage,
                                isPressed: $model.isTorchOn
                            )
                        }
                        .padding([.leading, .trailing], Spacing.medium)
                        .padding(.top, Spacing.medium + safeAreaInsets.top)
                        Spacer()
                        // Camera cutout
                        ZStack {
                            RoundedRectangle(cornerRadius: CornerRadius.qrCodeScanner)
                                .aspectRatio(1.0, contentMode: .fit)
                                .blendMode(.destinationOut)
                                .overlay(
                                    Asset.cameraOverlay.swiftUIImage
                                        .resizable(resizingMode: .stretch)
                                        .padding(-Spacing.extraExtraSmall)
                                )
                        }
                        .padding([.leading, .trailing], Spacing.medium)
                        Spacer()
                        // Text description
                        VStack(alignment: .center, spacing: Spacing.small) {
                            Text(viewModel.header)
                                .font(PrimaryFont.titleL.font)
                            Text(viewModel.message)
                                .font(PrimaryFont.bodyL.font)
                                .multilineTextAlignment(.center)
                        }
                        .foregroundColor(Asset.accentForegroundText.swiftUIColor)
                        .frame(width: UIScreen.main.bounds.width * 0.86, alignment: .center)
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
            viewModel.onAppear()
        }
        .onDisappear {
            model.shutdown()
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
        .fullScreenModal(
            isPresented: $viewModel.isPresentingTransactionPreview,
            onDismiss: {
                model.multipleTransactions = []
                model.start()
                viewModel.clearTransactionState()
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
        .fullScreenModal(
            isPresented: $viewModel.isPresentingEnterBananaSplitPassword,
            onDismiss: {
                model.start()

                // User entered invalid password too many times, present error
                if viewModel.shouldPresentError {
                    // iOS 15 handling of following .fullscreen presentation after dismissal, we need to dispatch this
                    // async
                    DispatchQueue.main.async { viewModel.isPresentingError = true }
                    return
                }
                viewModel.clearTransactionState()

                // User proceeded successfully with key recovery, dismiss camera
                if viewModel.wasBananaSplitKeyRecovered {
                    viewModel.dismissView()
                    viewModel.onBananaSplitComplete()
                }
            }
        ) {
            EnterBananaSplitPasswordModal(
                viewModel: .init(
                    isPresented: $viewModel.isPresentingEnterBananaSplitPassword,
                    isKeyRecovered: $viewModel.wasBananaSplitKeyRecovered,
                    isErrorPresented: $viewModel.shouldPresentError,
                    presentableError: $viewModel.presentableError,
                    qrCodeData: $model.bucket,
                    onComplete: $viewModel.onBananaSplitComplete
                )
            )
        }
        .fullScreenModal(
            isPresented: $viewModel.isPresentingEnterPassword,
            onDismiss: {
                // Clear password modal state no matter what
                defer { viewModel.enterPassword = nil }

                // User forgot password
                if viewModel.shouldPresentError {
                    viewModel.presentableError = .signingForgotPassword()
                    // iOS 15 handling of following .fullscreen presentation after dismissal, we need to dispatch this
                    // async
                    DispatchQueue.main.async { viewModel.isPresentingError = true }
                    return
                }
                // User entered valid password, signature is ready
                if viewModel.signature != nil {
                    // iOS 15 handling of following .fullscreen presentation after dismissal, we need to dispatch this
                    // async
                    DispatchQueue.main.async { viewModel.continueWithSignature() }
                    return
                }
                // Dismissed by user
                model.payload = nil
                model.start()
                viewModel.clearTransactionState()
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
        .fullScreenModal(
            isPresented: $viewModel.isPresentingError,
            onDismiss: {
                model.payload = nil
                model.start()
                viewModel.clearTransactionState()
            }
        ) {
            ErrorBottomModal(
                viewModel: viewModel.presentableError,
                isShowingBottomAlert: $viewModel.isPresentingError
            )
            .clearModalBackground()
        }
    }

    var multipleTransactionOverlay: some View {
        HStack(alignment: .center) {
            Text(signText())
                .font(PrimaryFont.titleS.font)
                .foregroundColor(Asset.accentForegroundText.swiftUIColor)
                .padding(.top, Spacing.medium)
            Spacer()
            CapsuleButton(
                action: {
                    viewModel.onMultipleTransactionSign(model.multipleTransactions)
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

        // Banana split flow
        @Published var isPresentingEnterBananaSplitPassword: Bool = false
        @Published var wasBananaSplitKeyRecovered: Bool = false
        @Published var onBananaSplitComplete: () -> Void = {}

        // Data models for modals
        @Published var transactions: [MTransaction] = []
        @Published var signature: MSignatureReady?
        @Published var enterPassword: MEnterPassword!
        @Published var presentableError: ErrorBottomModalViewModel = .signingForgotPassword()

        @Binding var isPresented: Bool
        @Binding var onComplete: () -> Void
        private weak var navigation: NavigationCoordinator!
        private let seedsMediator: SeedsMediating

        init(
            isPresented: Binding<Bool>,
            onComplete: Binding<() -> Void> = .constant {},
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
        ) {
            _isPresented = isPresented
            _onComplete = onComplete
            self.seedsMediator = seedsMediator
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func onAppear() {
            navigation.performFake(navigation: .init(action: .start))
            navigation.performFake(navigation: .init(action: .navbarScan))
        }

        func checkForTransactionNavigation(_ payload: String?) {
            guard let payload = payload, !isInTransactionProgress else { return }
            isInTransactionProgress = true
            switch navigation.performTransaction(with: payload) {
            case let .success(actionResult):
                // Handle transactions with just error payload
                guard case let .transaction(transactions) = actionResult.screenData else { return }
                if transactions.allSatisfy(\.isDisplayingErrorOnly) {
                    presentableError = .transactionSigningError(
                        message: transactions
                            .reduce("") { $0 + $1.transactionIssues() + ($1 == transactions.last ? "\n" : "") }
                    )
                    navigation.performFake(navigation: .init(action: .goBack))
                    isPresentingError = true
                    return
                }
                // Handle rest of transactions with optional error payload
                // Type is assumed based on first error
                let firstTransaction = transactions.first
                switch firstTransaction?.ttype {
                case .sign:
                    continueTransactionSignature(transactions)
                case .importDerivations:
                    continueImportDerivedKeys(transactions)
                default:
                    // Transaction with error
                    // Transaction that does not require signing (i.e. adding network or metadata)
                    self.transactions = transactions
                    isPresentingTransactionPreview = true
                }
            case let .failure(error):
                presentableError = ErrorBottomModalViewModel.transactionError(for: error)
                isPresentingError = true
            }
        }

        func continueWithSignature() {
            isPresentingTransactionPreview = true
        }

        func clearTransactionState() {
            transactions = []
            signature = nil
            enterPassword = nil
            isPresentingError = false
            shouldPresentError = false
            isInTransactionProgress = false
        }

        func presentBananaSplitPassword() {
            isPresentingEnterBananaSplitPassword = true
        }

        func dismissView() {
            isPresented = false
        }
    }
}

// MARK: - Transaction Signature

extension CameraView.ViewModel {
    func continueTransactionSignature(_ transactions: [MTransaction]) {
        let actionResult = sign(transactions: transactions)
        self.transactions = transactions
        // Password protected key, continue to modal
        if case let .enterPassword(value) = actionResult.modalData {
            enterPassword = value
            isPresentingEnterPassword = true
        }
        // Transaction ready to sign
        if case let .signatureReady(value) = actionResult.modalData {
            signature = value
            continueWithSignature()
        }
    }
}

// MARK: - Import Derived Keys

extension CameraView.ViewModel {
    func continueImportDerivedKeys(_ transactions: [MTransaction]) {
        // We always need to `.goBack` as even if camera is dismissed without import,
        // navigation "forward" already happened

        navigation.performFake(navigation: .init(action: .goBack))
        if let importError = transactions.dominantImportError {
            switch importError {
            case .networkMissing:
                presentableError = .importDerivedKeysMissingNetwork()
            case .keySetMissing:
                presentableError = .importDerivedKeysMissingKeySet()
            case .badFormat:
                presentableError = .importDerivedKeysBadFormat()
            }
            isPresentingError = true
        } else if !transactions.hasImportableKeys {
            presentableError = .allKeysAlreadyExist()
            isPresentingError = true
        } else {
            self.transactions = transactions
            isPresentingTransactionPreview = true
        }
    }
}

// MARK: - Mutliple Transactions mode

extension CameraView.ViewModel {
    func onMultipleTransactionSign(_ payloads: [String]) {
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
}

private extension CameraView.ViewModel {
    func updateTexts() {
        let key = Localizable.Scanner.Label.Scan.self
        withAnimation {
            header = (isScanningMultiple ? key.Multiple.header : key.Main.header).string
            message = (isScanningMultiple ? key.Multiple.message : key.Main.message).string
        }
    }

    func sign(transactions: [MTransaction]) -> ActionResult {
        let seedNames = transactions.compactMap { $0.authorInfo?.address.seedName }
        let seedPhrasesDictionary = seedsMediator.getSeeds(seedNames: Set(seedNames))
        return navigation.performFake(
            navigation:
            .init(
                action: .goForward,
                details: "",
                seedPhrase: formattedPhrase(seedNames: seedNames, with: seedPhrasesDictionary)
            )
        )
    }

    func formattedPhrase(seedNames: [String], with dictionary: [String: String]) -> String {
        seedNames.reduce(into: "") { $0 += "\(dictionary[$1] ?? "")\n" }
    }
}
