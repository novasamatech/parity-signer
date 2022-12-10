//
//  TransactionPreview.swift
//  NativeSigner
//
//  Created by Alexander Slesarev on 9.8.2021.
//

import SwiftUI

struct TransactionWrapper: Identifiable {
    let id = UUID()
    let content: MTransaction
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
            ScrollView {
                ForEach(viewModel.dataModel, id: \.id) { singleTransaction(content: $0.content) }
                qrCodeComponent(viewModel.dataModel.count)
                logNote(viewModel.dataModel.first?.content.ttype)
                actions(viewModel.dataModel.first?.content.ttype)
            }
        }
        .onAppear {
            viewModel.use(navigation: navigation)
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
            TransactionErrorsView(content: content)
                .padding(.horizontal, Spacing.medium)
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
            case .stub:
                VStack {
                    ForEach(content.sortedValueCards(), id: \.index) { card in
                        TransactionCardView(card: card)
                    }
                }
                .padding(Spacing.medium)
            case .done,
                 .importDerivations:
                VStack {
                    ForEach(content.sortedValueCards(), id: \.index) { card in
                        TransactionCardView(card: card)
                    }
                }
                .padding(Spacing.medium)
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
            case .stub,
                 .importDerivations:
                PrimaryButton(
                    action: viewModel.onApproveTap,
                    text: transactionType == .stub ? Localizable.TransactionPreview.Action.approve.key : Localizable
                        .TransactionPreview.Action.selectSeed.key,
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
        default:
            return transactionsCount == 1 ?
                Localizable.TransactionSign.Label.Header.single.string :
                Localizable.TransactionSign.Label.Header.multiple(viewModel.dataModel.count)
        }
    }
}

extension TransactionPreview {
    final class ViewModel: ObservableObject {
        @Binding var isPresented: Bool
        @Published var isDetailsPresented: Bool = false
        @Published var selectedDetails: MTransaction!
        private weak var navigation: NavigationCoordinator!
        private weak var data: SignerDataModel!
        private let seedsMediator: SeedsMediating
        private let snackbarPresentation: BottomSnackbarPresentation

        let dataModel: [TransactionWrapper]
        let signature: MSignatureReady?

        init(
            isPresented: Binding<Bool>,
            content: [MTransaction],
            signature: MSignatureReady?,
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
            snackbarPresentation: BottomSnackbarPresentation = ServiceLocator.bottomSnackbarPresentation
        ) {
            _isPresented = isPresented
            dataModel = content.map { TransactionWrapper(content: $0) }
            self.signature = signature
            self.seedsMediator = seedsMediator
            self.snackbarPresentation = snackbarPresentation
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func use(data: SignerDataModel) {
            self.data = data
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

        func signTransaction() {
            let seedName = dataModel.compactMap { $0.content.authorInfo?.address.seedName }.first
            let seedPhrase = seedsMediator.getSeed(seedName: seedName ?? "")
            let actionResult = navigation.performFake(
                navigation:
                .init(
                    action: .goForward,
                    details: "",
                    seedPhrase: seedPhrase
                )
            )
            print(actionResult)
        }

        func presentDetails(for content: MTransaction) {
            selectedDetails = content
            isDetailsPresented = true
        }
    }
}

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
