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
            }
            actions(transactionType: viewModel.dataModel.first?.content.ttype)
        }
        .onAppear {
            viewModel.use(navigation: navigation)
        }
        .background(Asset.backgroundPrimary.swiftUIColor)
    }

    @ViewBuilder
    func singleTransaction(content: MTransaction) -> some View {
        VStack {
            TransactionSummaryView(renderable: .init(content), onTransactionDetailsTap: {})
            VStack {
                ForEach(content.content.asSortedCards(), id: \.index) { card in
                    TransactionCardView(card: card)
                }
            }
            if let authorInfo = content.authorInfo {
                AddressCard(card: authorInfo)
            }
            if let network = content.networkInfo {
                NetworkCard(title: network.networkTitle, logo: network.networkLogo)
            }
            if content.ttype == .sign {
                signContent()
            }
        }
        .padding(.horizontal, Spacing.extraSmall)
    }

    @ViewBuilder
    func signContent() -> some View {
        HStack {
            Localizable.logNote.text.font(Fontstyle.overline.base)
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
                        viewModel.sign(with: comment)
                    },
                    text: Localizable.TransactionPreview.Action.unlockSign.key,
                    style: .primary()
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
            if transactionType != .done {
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
        private weak var navigation: NavigationCoordinator!
        private weak var data: SignerDataModel!
        private let seedsMediator: SeedsMediating

        let dataModel: [TransactionWrapper]

        init(
            isPresented: Binding<Bool>,
            content: [MTransaction],
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
        ) {
            _isPresented = isPresented
            dataModel = content.map { TransactionWrapper(content: $0) }
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
    }
}

struct TransactionPreview_Previews: PreviewProvider {
    static var previews: some View {
        TransactionPreview(viewModel: .init(
            isPresented: Binding<Bool>.constant(true),
            content: [PreviewData.signTransaction]
        ))
        .environmentObject(NavigationCoordinator())
        .environmentObject(SignerDataModel())
        .preferredColorScheme(.dark)
    }
}
