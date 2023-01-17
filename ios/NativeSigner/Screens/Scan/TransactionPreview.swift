//
//  TransactionPreview.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 9.8.2021.
//

import SwiftUI

struct TransactionWrapper: Identifiable {
    let id = UUID()
    var content: MTransaction
}

struct TransactionPreview: View {
    @State private var comment = ""
    @FocusState private var focus: Bool
    @State private var isLogNoteVisible: Bool = false
    @StateObject var viewModel: ViewModel
    @EnvironmentObject private var navigation: NavigationCoordinator
    @EnvironmentObject private var data: SignerDataModel

    var body: some View {
        VStack {
            NavigationBarView(
                viewModel: .init(
                    title: title(
                        viewModel.dataModel.count,
                        previewType: viewModel.dataModel.first?.content.previewType
                    ),
                    leftButton: .xmark
                ),
                actionModel: .init(
                    leftBarMenuAction: { viewModel.onBackButtonTap() },
                    rightBarMenuAction: {}
                )
            )
            ScrollView(showsIndicators: false) {
                ForEach(viewModel.dataModel, id: \.id) { singleTransaction(content: $0.content) }
                qrCodeComponent(viewModel.dataModel.count)
                logNote(viewModel.dataModel.first?.content.ttype)
                actions(viewModel.dataModel.first?.content.ttype)
            }
        }
        .onAppear {
            viewModel.use(navigation: navigation)
            viewModel.onAppear()
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
        .bottomEdgeOverlay(
            overlayView:
            TransactionDetailsView(
                viewModel: .init(
                    isPresented: $viewModel.isDetailsPresented,
                    transaction: viewModel.selectedDetails
                )
            ),
            isPresented: $viewModel.isDetailsPresented
        )
    }

    @ViewBuilder
    func singleTransaction(content: MTransaction) -> some View {
        VStack(alignment: .leading, spacing: Spacing.medium) {
            if !content.transactionIssuesCards().isEmpty {
                TransactionErrorsView(content: content)
                    .padding(.horizontal, Spacing.medium)
            }
            switch content.ttype {
            case .sign,
                 .read:
                // Rounded corner summary card
                TransactionSummaryView(
                    renderable: .init(content),
                    onTransactionDetailsTap: {
                        viewModel.presentDetails(for: content)
                    }
                )
                .padding(.horizontal, Spacing.medium)
            // Used when new network is being added
            // User when network metadata is being added
            // Cards are redesigned to present new design
            case .stub,
                 .done:
                VStack {
                    ForEach(content.sortedValueCards(), id: \.index) { card in
                        TransactionCardView(card: card)
                    }
                }
                .padding(Spacing.medium)
            case .importDerivations:
                VStack {
                    ForEach(content.sortedValueCards(), id: \.index) { card in
                        TransactionCardSelector(card: card)
                            .frame(minHeight: Heights.minTransactionCardHeight)
                            .onAppear {
                                print("Card info: \(card.index)  \(card.indent)  \(card.card)")
                            }
                    }
                }
            }
        }
    }

    @ViewBuilder
    func logNote(_ transactionType: TransactionType?) -> some View {
        switch transactionType {
        case .sign:
            VStack(alignment: .leading) {
                if isLogNoteVisible {
                    VStack(alignment: .leading) {
                        Localizable.TransactionSign.Action.note.text
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                            .font(PrimaryFont.bodyL.font)
                        TextField("", text: $comment)
                            .primaryTextFieldStyle(
                                Localizable.TransactionSign.Action.note.string,
                                text: $comment
                            )
                            .focused($focus)
                    }
                } else {
                    InlineButton(
                        action: {
                            withAnimation {
                                isLogNoteVisible = true
                            }
                        },
                        icon: Asset.add.swiftUIImage,
                        text: Localizable.TransactionSign.Action.note.string
                    )
                }
            }
            .animation(.easeInOut, value: isLogNoteVisible)
            .padding(.horizontal, Spacing.large)
            .padding(.vertical, Spacing.medium)
        case .stub,
             .importDerivations,
             .read,
             .done,
             .none:
            EmptyView()
        }
    }

    @ViewBuilder
    func actions(_ transactionType: TransactionType?) -> some View {
        VStack {
            switch transactionType {
            case .sign,
                 .read:
                PrimaryButton(
                    action: viewModel.onDoneTap,
                    text: Localizable.TransactionPreview.Action.done.key,
                    style: .secondary()
                )
            case .stub:
                PrimaryButton(
                    action: viewModel.onApproveTap,
                    text: Localizable.TransactionPreview.Action.approve.key,
                    style: .primary()
                )
            case .importDerivations:
                PrimaryButton(
                    action: viewModel.onImportKeysTap,
                    text: Localizable.ImportKeys.Action.import.key,
                    style: .primary()
                )
            case .done,
                 .none:
                EmptyView()
            }
            if ![.done, .sign, .read].contains(transactionType) {
                EmptyButton(
                    action: viewModel.onCancelTap,
                    text: Localizable.TransactionPreview.Action.cancel.key
                )
            }
        }
        .padding(.horizontal, Spacing.large)
        .padding(.bottom, Spacing.medium)
        .padding(.top, Spacing.large)
    }

    @ViewBuilder
    func qrCodeComponent(_ transactionsCount: Int) -> some View {
        if let signature = viewModel.signature {
            VStack(alignment: .leading, spacing: Spacing.small) {
                // Header
                Localizable.TransactionSign.Label.signCode.text
                    .font(PrimaryFont.bodyL.font)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                    .padding(.leading, Spacing.extraSmall)
                // QR Code container
                VStack(alignment: .leading, spacing: Spacing.medium) {
                    AnimatedQRCodeView(
                        viewModel: Binding<AnimatedQRCodeViewModel>.constant(
                            .init(
                                qrCodes: signature.signatures.map(\.payload)
                            )
                        )
                    )
                    if transactionsCount > 1 {
                        InfoBoxView(text: Localizable.TransactionSign.Label.multipleTransactionsInfo.string)
                    }
                }
                .padding(transactionsCount > 1 ? Spacing.medium : 0)
                .containerBackground()
            }
            .padding(.horizontal, Spacing.medium)
            .padding(.top, Spacing.large)
        } else {
            EmptyView()
        }
    }

    func title(_ transactionsCount: Int, previewType: MTransaction.TransactionPreviewType?) -> String {
        switch previewType {
        case .addNetwork:
            return Localizable.TransactionSign.Label.Header.network.string
        case .metadata:
            return Localizable.TransactionSign.Label.Header.metadata.string
        case let .importKeys(keysCount):
            return keysCount == 1 ?
                Localizable.ImportKeys.Label.Title.single.string :
                Localizable.ImportKeys.Label.Title.multiple(keysCount)
        default:
            return transactionsCount == 1 ?
                Localizable.TransactionSign.Label.Header.single.string :
                Localizable.TransactionSign.Label.Header.multiple(transactionsCount)
        }
    }
}

extension TransactionPreview {
    final class ViewModel: ObservableObject {
        @Binding var isPresented: Bool
        @Published var isDetailsPresented: Bool = false
        @Published var selectedDetails: MTransaction!
        @Published var dataModel: [TransactionWrapper]
        private weak var navigation: NavigationCoordinator!
        private weak var data: SignerDataModel!
        private let seedsMediator: SeedsMediating
        private let snackbarPresentation: BottomSnackbarPresentation
        private let importKeysService: ImportDerivedKeysService

        let signature: MSignatureReady?

        init(
            isPresented: Binding<Bool>,
            content: [MTransaction],
            signature: MSignatureReady?,
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
            snackbarPresentation: BottomSnackbarPresentation = ServiceLocator.bottomSnackbarPresentation,
            importKeysService: ImportDerivedKeysService = ImportDerivedKeysService()
        ) {
            _isPresented = isPresented
            dataModel = content.map { TransactionWrapper(content: $0) }
            self.signature = signature
            self.seedsMediator = seedsMediator
            self.snackbarPresentation = snackbarPresentation
            self.importKeysService = importKeysService
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func use(data: SignerDataModel) {
            self.data = data
        }

        func onAppear() {
            updateImportDerivationsIfNeeeded()
        }

        /// For `ttype` transaction of `importDerivations`, we need to update data by passing all seed phrases to Rust
        private func updateImportDerivationsIfNeeeded() {
            let seedPreviews = dataModel.map(\.content).allImportDerivedKeys
            guard !seedPreviews.isEmpty else { return }

            importKeysService.updateWithSeeds(seedPreviews) { result in
                switch result {
                case let .success(updatedSeeds):
                    self.updateImportDerivationsData(updatedSeeds)
                case .failure:
                    self.snackbarPresentation.viewModel = .init(
                        title: Localizable.ImportKeys.Snackbar.Failure.unknown.string,
                        style: .warning
                    )
                    self.snackbarPresentation.isSnackbarPresented = true
                    self.isPresented.toggle()
                }
            }
        }

        /// We need to mutate existing data model in an ugly way as Rust data model
        /// features miriad of enums with associated values and nested array models...
        ///
        /// Logic here is to find `TransactionCard` with `importingDerivations` and just replace its
        /// `.derivationsCard(f: [SeedKeysPreview])` enum value with update `[SeedKeysPreview]`
        private func updateImportDerivationsData(_ updatedSeeds: [SeedKeysPreview]) {
            guard let indexToUpdate = dataModel.firstIndex(where: { $0.content.content.importingDerivations != nil })
            else { return }
            dataModel[indexToUpdate].content.content
                .importingDerivations = [.init(index: 0, indent: 0, card: .derivationsCard(f: updatedSeeds))]
        }

        func onBackButtonTap() {
            navigation.performFake(navigation: .init(action: .goBack))
            isPresented.toggle()
        }

        func onDoneTap() {
            navigation.performFake(navigation: .init(action: .goBack))
            isPresented.toggle()
        }

        func onCancelTap() {
            navigation.performFake(navigation: .init(action: .goBack))
            isPresented.toggle()
        }

        func onApproveTap() {
            navigation.perform(navigation: .init(action: .goForward))
            isPresented.toggle()
            switch dataModel.first?.content.previewType {
            case let .addNetwork(network):
                snackbarPresentation.viewModel = .init(
                    title: Localizable.TransactionSign.Snackbar.networkAdded(network),
                    style: .info
                )
                snackbarPresentation.isSnackbarPresented = true
            case let .metadata(network, version):
                snackbarPresentation.viewModel = .init(
                    title: Localizable.TransactionSign.Snackbar.metadata(network, version),
                    style: .info
                )
                snackbarPresentation.isSnackbarPresented = true
            default:
                ()
            }
        }

        func onImportKeysTap() {
            let importableKeys = dataModel.map(\.content).importableSeedKeysPreviews

            importKeysService.importDerivedKeys(importableKeys) { result in
                let derivedKeysCount = self.dataModel.map(\.content).importableKeysCount
                switch result {
                case .success:
                    if derivedKeysCount == 1 {
                        self.snackbarPresentation
                            .viewModel = .init(title: Localizable.ImportKeys.Snackbar.Success.single.string)
                    } else {
                        self.snackbarPresentation
                            .viewModel = .init(
                                title: Localizable.ImportKeys.Snackbar.Success
                                    .multiple(derivedKeysCount)
                            )
                    }
                case .failure:
                    self.snackbarPresentation.viewModel = .init(
                        title: Localizable.ImportKeys.Snackbar.Failure.unknown.string,
                        style: .warning
                    )
                }

                self.snackbarPresentation.isSnackbarPresented = true
                self.isPresented.toggle()
            }
        }

        func presentDetails(for content: MTransaction) {
            selectedDetails = content
            isDetailsPresented = true
        }
    }
}

#if DEBUG
    struct TransactionPreview_Previews: PreviewProvider {
        static var previews: some View {
            // Single transaction
            TransactionPreview(viewModel: .init(
                isPresented: Binding<Bool>.constant(true),
                content: [PreviewData.signTransaction],
                signature: MSignatureReady(signatures: [.regular(data: PreviewData.exampleQRCode)])
            ))
            .environmentObject(NavigationCoordinator())
            .environmentObject(SignerDataModel())
            .preferredColorScheme(.dark)
            // Multi transaction (i.e. different QR code design)
            TransactionPreview(viewModel: .init(
                isPresented: Binding<Bool>.constant(true),
                content: [PreviewData.signTransaction, PreviewData.signTransaction],
                signature: MSignatureReady(signatures: [.regular(data: PreviewData.exampleQRCode)])
            ))
            .environmentObject(NavigationCoordinator())
            .environmentObject(SignerDataModel())
//        .preferredColorScheme(.dark)
        }
    }
#endif
