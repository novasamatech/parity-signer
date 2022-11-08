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
                viewModel: .init(title: Localizable.TransactionPreview.Label.title.string, leftButton: .xmark),
                actionModel: .init(
                    leftBarMenuAction: { viewModel.onBackButtonTap() },
                    rightBarMenuAction: {}
                )
            )
            ScrollView {
                ForEach(
                    viewModel.dataModel,
                    id: \.id
                ) { singleTransaction(content: $0.content)
                    .padding(.horizontal, Spacing.medium)
                    .frame(width: UIScreen.main.bounds.width)
                }
            }
            VStack {
                // CTAs
                VStack {
                    switch viewModel.dataModel.first?.content.ttype {
                    case .sign:
                        BigButton(
                            text: Localizable.TransactionPreview.unlockSign.key,
                            isShaded: false,
                            isCrypto: true,
                            action: {
                                focus = false
                                viewModel.sign(with: comment)
                            }
                        )
                    case .stub:
                        BigButton(
                            text: Localizable.TransactionPreview.approve.key,
                            action: {
                                navigation.perform(navigation: .init(action: .goForward))
                            }
                        )
                    case .importDerivations:
                        BigButton(
                            text: Localizable.TransactionPreview.selectSeed.key,
                            isCrypto: true,
                            action: {
                                navigation.perform(navigation: .init(action: .goForward))
                            }
                        )
                    case .read,
                         .done,
                         .none:
                        EmptyView()
                    }
                    if viewModel.dataModel.first?.content.ttype != .done {
                        BigButton(
                            text: Localizable.TransactionPreview.decline.key,
                            isShaded: true,
                            isDangerous: true,
                            action: {
                                focus = false
                                navigation.perform(navigation: .init(action: .goBack))
                            }
                        )
                    }
                }
            }
            .padding(.top, -10)
            .padding(.horizontal, 16)
        }
        .onAppear {
            viewModel.use(navigation: navigation)
        }
        .frame(width: UIScreen.main.bounds.width)
    }

    @ViewBuilder
    func singleTransaction(content: MTransaction) -> some View {
        VStack {
            TransactionBlock(cards: content.content.assemble())
                .padding(.bottom, Spacing.medium)
            if let authorInfo = content.authorInfo {
                AddressCard(card: authorInfo)
            }
            if let network = content.networkInfo {
                NetworkCard(title: network.networkTitle, logo: network.networkLogo)
            }
            if content.ttype == .sign {
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
                    .padding(.horizontal, 8)
                }
                HStack {
                    Localizable.visibleOnlyOnThisDevice.text
                        .font(Fontstyle.subtitle1.base)
                        .padding(.bottom)
                    Spacer()
                }
            }
        }
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

// struct TransactionPreview_Previews: PreviewProvider {
// static var previews: some View {
// TransactionPreview()
// }
// }
