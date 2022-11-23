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
                viewModel: .init(title: title(), leftButton: .xmark),
                actionModel: .init(
                    leftBarMenuAction: { viewModel.onBackButtonTap() },
                    rightBarMenuAction: {}
                )
            )
            ScrollView {
                ForEach(viewModel.dataModel, id: \.id) { singleTransaction(content: $0.content) }
                // QR Code
                if let signature = viewModel.signature {
                    VStack(alignment: .leading, spacing: Spacing.small) {
                        Localizable.TransactionSign.Label.signCode.text
                            .font(Fontstyle.bodyL.base)
                            .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        AnimatedQRCodeView(
                            viewModel: Binding<AnimatedQRCodeViewModel>
                                .constant(.init(qrCodes: [signature.signature]))
                        )
                    }
                    .padding(.horizontal, Spacing.large)
                    .padding(.top, Spacing.large)
                }
                logNote()
                actions(transactionType: viewModel.dataModel.first?.content.ttype)
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
        VStack(alignment: .leading, spacing: 0) {
            switch content.ttype {
            case .sign:
                // Rounded corner summary card
                TransactionSummaryView(
                    renderable: .init(content),
                    onTransactionDetailsTap: {
                        viewModel.presentDetails(for: content)
                    }
                )
                .padding(.horizontal, Spacing.medium)
            default:
                VStack {
                    ForEach(content.content.asSortedCards(), id: \.index) { card in
                        TransactionCardView(card: card)
                    }
                }
                .padding(.horizontal, Spacing.medium)
            }
        }
    }

    @ViewBuilder
    func logNote() -> some View {
        // Log Note
        VStack(alignment: .leading) {
            if isLogNoteVisible {
                VStack(alignment: .leading) {
                    Localizable.TransactionSign.Action.note.text
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                        .font(Fontstyle.bodyL.base)
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
    }

    @ViewBuilder
    /// To be deleted
    func signContent() -> some View {
        HStack {
            Localizable.logNote.text
                .font(Fontstyle.overline.base)
                .foregroundColor(Asset.text400.swiftUIColor)
            Spacer()
        }
        ZStack {
            RoundedRectangle(cornerRadius: 8).stroke(Asset.border400.swiftUIColor).frame(height: 39)
            TextField(
                Localizable.comment.string,
                text: $comment,
                prompt: Localizable.commentNotPublished.text
            )
            .foregroundColor(Asset.text400.swiftUIColor)
            .background(Asset.bg100.swiftUIColor)
            .font(Fontstyle.body2.base)
            .focused($focus)
            .onDisappear {
                focus = false
            }
            .padding(.horizontal, Spacing.extraSmall)
        }
        HStack {
            Localizable.visibleOnlyOnThisDevice.text
                .font(Fontstyle.subtitle1.base)
                .padding(.bottom)
            Spacer()
        }
    }

    @ViewBuilder
    func actions(transactionType: TransactionType?) -> some View {
        VStack {
            switch transactionType {
            case .sign:
                PrimaryButton(
                    action: {
                        focus = false
                        viewModel.onBackButtonTap()
                    },
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
            case .read,
                 .done,
                 .none:
                EmptyView()
            }
            if ![.done, .sign].contains(transactionType) {
                EmptyButton(
                    action: viewModel.onCancelTap,
                    text: Localizable.TransactionPreview.Action.cancel.key
                )
            }
        }
        .padding(.horizontal, Spacing.large)
        .padding(.bottom, Spacing.medium)
        .padding(.top, Spacing.extraSmall)
    }

    func title() -> String {
        viewModel.dataModel.count == 1 ?
            Localizable.TransactionSign.Label.Header.single.string :
            Localizable.TransactionSign.Label.Header.multiple(viewModel.dataModel.count)
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

        let dataModel: [TransactionWrapper]
        let signature: MSignatureReady?

        init(
            isPresented: Binding<Bool>,
            content: [MTransaction],
            signature: MSignatureReady?,
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
        ) {
            _isPresented = isPresented
            dataModel = content.map { TransactionWrapper(content: $0) }
            self.signature = signature
            self.seedsMediator = seedsMediator
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

        func onCancelTap() {
            navigation.performFake(navigation: .init(action: .goBack))
            isPresented.toggle()
        }

        func onApproveTap() {
            navigation.perform(navigation: .init(action: .goForward))
        }

        func sign(with comment: String) {
            let seedName = dataModel.compactMap { $0.content.authorInfo?.address.seedName }.first
            let seedPhrase = seedsMediator.getSeed(seedName: seedName ?? "")
            navigation.perform(
                navigation:
                .init(
                    action: .goForward,
                    details: comment,
                    seedPhrase: seedPhrase
                )
            )
        }

        func presentDetails(for content: MTransaction) {
            selectedDetails = content
            isDetailsPresented = true
        }
    }
}

struct TransactionPreview_Previews: PreviewProvider {
    static var previews: some View {
        TransactionPreview(viewModel: .init(
            isPresented: Binding<Bool>.constant(true),
            content: [PreviewData.signTransaction],
            signature: MSignatureReady(signature: PreviewData.exampleQRCode)
        ))
        .environmentObject(NavigationCoordinator())
        .environmentObject(SignerDataModel())
        .preferredColorScheme(.dark)
    }
}
