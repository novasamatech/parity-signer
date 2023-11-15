//
//  TransactionPreview.swift
//  Polkadot Vault
//
//  Created by Alexander Slesarev on 9.8.2021.
//

import Combine
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
    @Environment(\.presentationMode) var presentationMode

    var body: some View {
        VStack {
            NavigationBarView(
                viewModel: .init(
                    title: .title(title(
                        viewModel.dataModel.count,
                        previewType: viewModel.dataModel.first?.content.previewType
                    )),
                    leftButtons: [.init(type: .xmark, action: viewModel.onBackButtonTap)],
                    rightButtons: [.init(type: .empty)]
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
            viewModel.onAppear()
        }
        .background(.backgroundPrimary)
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
        .onReceive(viewModel.dismissViewRequest) { _ in
            presentationMode.wrappedValue.dismiss()
        }
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
                            .foregroundColor(.textAndIconsPrimary)
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
                        icon: Image(.addLarge),
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
                    action: viewModel.onCancelTap(),
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
                    .foregroundColor(.textAndIconsPrimary)
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
            Localizable.TransactionSign.Label.Header.network.string
        case .metadata:
            Localizable.TransactionSign.Label.Header.metadata.string
        case let .importKeys(keysCount):
            keysCount == 1 ?
                Localizable.ImportKeys.Label.Title.single.string :
                Localizable.ImportKeys.Label.Title.multiple(keysCount)
        default:
            transactionsCount == 1 ?
                Localizable.TransactionSign.Label.Header.single.string :
                Localizable.TransactionSign.Label.Header.multiple(transactionsCount)
        }
    }
}

extension TransactionPreview {
    enum OnCompletionAction: Equatable {
        case onDismissal
        case onDone
        case onImportKeysFailure
        case onNetworkAdded(String)
        case onNetworkMetadataAdded(network: String, metadataVersion: String)
        case onDerivedKeysImport(count: Int)
    }

    final class ViewModel: ObservableObject {
        @Published var isDetailsPresented: Bool = false
        @Published var selectedDetails: MTransaction!
        @Published var dataModel: [TransactionWrapper]
        private let scanService: ScanTabService
        private let seedsMediator: SeedsMediating
        private let importKeysService: ImportDerivedKeysService
        var dismissViewRequest: AnyPublisher<Void, Never> { dismissRequest.eraseToAnyPublisher() }
        private let dismissRequest = PassthroughSubject<Void, Never>()
        private let onCompletion: (OnCompletionAction) -> Void

        let signature: MSignatureReady?

        init(
            content: [MTransaction],
            signature: MSignatureReady?,
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
            importKeysService: ImportDerivedKeysService = ImportDerivedKeysService(),
            scanService: ScanTabService = ScanTabService(),
            onCompletion: @escaping (OnCompletionAction) -> Void
        ) {
            dataModel = content.map { TransactionWrapper(content: $0) }
            self.signature = signature
            self.seedsMediator = seedsMediator
            self.importKeysService = importKeysService
            self.scanService = scanService
            self.onCompletion = onCompletion
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
                    self.onCompletion(.onImportKeysFailure)
                    self.dismissRequest.send()
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
            scanService.resetNavigationState()
            dismissRequest.send()
            onCompletion(.onDismissal)
        }

        func onDoneTap() {
            scanService.resetNavigationState()
            dismissRequest.send()
            onCompletion(.onDone)
        }

        func onCancelTap() {
            scanService.resetNavigationState()
            dismissRequest.send()
            onCompletion(.onDismissal)
        }

        func onApproveTap() {
            scanService.onTransactionApprove()
            switch dataModel.first?.content.previewType {
            case let .addNetwork(network):
                onCompletion(.onNetworkAdded(network))
            case let .metadata(network, version):
                onCompletion(.onNetworkMetadataAdded(network: network, metadataVersion: version))
            default:
                onCompletion(.onDone)
            }
            dismissRequest.send()
        }

        func onImportKeysTap() {
            let importableKeys = dataModel.map(\.content).importableSeedKeysPreviews

            importKeysService.importDerivedKeys(importableKeys) { result in
                let derivedKeysCount = self.dataModel.map(\.content).importableKeysCount
                switch result {
                case .success:
                    self.onCompletion(.onDerivedKeysImport(count: derivedKeysCount))
                case .failure:
                    self.onCompletion(.onImportKeysFailure)
                }
                self.dismissRequest.send()
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
                content: [.stub],
                signature: .stub,
                onCompletion: { _ in }
            ))
            .preferredColorScheme(.dark)
            // Multi transaction (i.e. different QR code design)
            TransactionPreview(viewModel: .init(
                content: [.stub, .stub],
                signature: .stub,
                onCompletion: { _ in }
            ))
        }
    }
#endif
