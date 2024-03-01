//
//  BananaSplitModal.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 21/02/2024.
//

import SwiftUI

struct BananaSplitModalView: View {
    @StateObject var viewModel: ViewModel
    @FocusState private var textFieldFocused: Bool

    var body: some View {
        NavigationView {
            GeometryReader { geo in
                VStack(spacing: 0) {
                    NavigationBarView(
                        viewModel: .init(
                            leftButtons: [.init(
                                type: .xmark,
                                action: viewModel.onBackTap
                            )],
                            rightButtons: [.init(
                                type: .activeAction(
                                    Localizable.BananaSplitBackup.Action.create.key,
                                    .constant(!viewModel.isActionAvailable())
                                ),
                                action: {
                                    textFieldFocused = false
                                    viewModel.onCreateTap()
                                }
                            )]
                        )
                    )
                    ScrollView(showsIndicators: false) {
                        VStack(alignment: .leading, spacing: 0) {
                            mainContent()
                            passphraseView()
                            infoView()
                            Spacer()
                        }
                    }
                }
                .frame(
                    minWidth: geo.size.width,
                    minHeight: geo.size.height
                )
                .background(.backgroundPrimary)
                NavigationLink(
                    destination:
                    BananaSplitQRCodeModalView(
                        viewModel: .init(
                            seedName: viewModel.seedName,
                            bananaSplitBackup: viewModel.bananaSplitBackup,
                            onCompletion: viewModel.onQRCodeCompletion
                        )
                    )
                    .navigationBarHidden(true),
                    isActive: $viewModel.isPresentingQRCode
                ) { EmptyView() }
            }
            .navigationBarHidden(true)
            .navigationViewStyle(.stack)
            .fullScreenModal(
                isPresented: $viewModel.isPresentingError
            ) {
                ErrorBottomModal(
                    viewModel: viewModel.presentableError,
                    isShowingBottomAlert: $viewModel.isPresentingError
                )
                .clearModalBackground()
            }
        }
    }

    @ViewBuilder
    func mainContent() -> some View {
        VStack(alignment: .leading, spacing: 0) {
            Localizable.BananaSplitBackup.Label.title.text
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.titleL.font)
                .padding(.top, Spacing.extraSmall)
                .padding(.horizontal, Spacing.extraSmall)
            Localizable.BananaSplitBackup.Label.header.text
                .foregroundColor(.textAndIconsTertiary)
                .font(PrimaryFont.bodyM.font)
                .padding(.vertical, Spacing.extraSmall)
                .padding(.horizontal, Spacing.extraSmall)
            Localizable.BananaSplitBackup.Label.Shards.header.text
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.bodyL.font)
                .padding(.vertical, Spacing.extraSmall)
                .padding(.horizontal, Spacing.extraSmall)
            TextField("", text: $viewModel.totalShards)
                .submitLabel(.done)
                .primaryTextFieldStyle(
                    Localizable.NewSeed.Name.Label.placeholder.string,
                    keyboardType: .asciiCapableNumberPad,
                    text: $viewModel.totalShards
                )
                .focused($textFieldFocused)
                .onSubmit {
                    textFieldFocused = false
                    viewModel.onSubmitTap()
                }
                .padding(.vertical, Spacing.medium)
            Text(Localizable.BananaSplitBackup.Label.Shards.footer(
                viewModel.requiredShardsCount,
                viewModel.totalShards
            ))
            .foregroundColor(.textAndIconsTertiary)
            .font(PrimaryFont.captionM.font)
            .padding(.horizontal, Spacing.extraSmall)
        }
        .padding(.horizontal, Spacing.medium)
        .padding(.bottom, Spacing.medium)
    }

    @ViewBuilder
    func passphraseView() -> some View {
        VStack(alignment: .leading, spacing: Spacing.medium) {
            HStack(alignment: .center, spacing: 0) {
                VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                    Localizable.BananaSplitBackup.Label.Passphrase.header.text
                        .foregroundColor(.textAndIconsTertiary)
                        .font(PrimaryFont.bodyM.font)
                    Text(viewModel.recoveryPassphrase)
                        .foregroundColor(.textAndIconsPrimary)
                        .font(PrimaryFont.bodyL.font)
                }
                Spacer()
                IconButton(
                    action: viewModel.refreshPassphrase,
                    icon: .refreshPassphrase
                )
            }
            .padding(.vertical, Spacing.medium)
            .padding(.leading, Spacing.medium)
            .padding(.trailing, Spacing.extraSmall)
            .overlay(
                RoundedRectangle(cornerRadius: Spacing.medium)
                    .stroke(.fill12, lineWidth: 1)
            )
            Localizable.BananaSplitBackup.Label.Passphrase.footer.text
                .foregroundColor(.textAndIconsTertiary)
                .font(PrimaryFont.captionM.font)
                .padding(.horizontal, Spacing.extraSmall)
        }
        .padding(.horizontal, Spacing.medium)
    }

    @ViewBuilder
    func infoView() -> some View {
        HStack(alignment: .center, spacing: Spacing.medium) {
            Localizable.BananaSplitBackup.Label.Passphrase.info.text
                .frame(maxWidth: .infinity, alignment: .leading)
                .fixedSize(horizontal: false, vertical: true)
                .foregroundColor(.accentPink300)
                .font(PrimaryFont.captionM.font)
            Image(.infoIconBold)
                .foregroundColor(.accentPink300)
        }
        .padding(Spacing.medium)
        .background(
            RoundedRectangle(cornerRadius: CornerRadius.medium)
                .foregroundColor(.accentPink300Fill8)
        )
        .padding(.horizontal, Spacing.medium)
        .padding(.top, Spacing.extraExtraLarge)
        .padding(.bottom, Spacing.medium)
    }
}

extension BananaSplitModalView {
    private enum Constants {
        static let passphraseWords: UInt32 = 4
        static let defaultTotalShards: UInt32 = 3
    }

    enum OnCompletionAction: Equatable {
        case create([QrData])
        case cancel
        case close
    }

    final class ViewModel: ObservableObject {
        @Published var totalShards: String = .init(Constants.defaultTotalShards) {
            didSet {
                updateRequiredShards()
            }
        }

        @Published var requiredShardsCount: UInt32 = 2
        @Published var recoveryPassphrase: String = ""
        @Published var isPresentingError: Bool = false
        @Published var isPresentingQRCode: Bool = false
        @Published var bananaSplitBackup: BananaSplitBackup = .init(qrCodes: [])
        @Published var presentableError: ErrorBottomModalViewModel!
        @Binding var isPresented: Bool
        let onCompletion: (BananaSplitModalView.OnCompletionAction) -> Void

        let seedName: String
        private let bananaSplitMediator: KeychainBananaSplitAccessAdapting
        private let seedsMediator: SeedsMediating
        private let service: BananaSplitServicing
        private var totalShardsCount: UInt32 {
            UInt32(totalShards) ?? Constants.defaultTotalShards
        }

        init(
            seedName: String,
            bananaSplitMediator: KeychainBananaSplitAccessAdapting = KeychainBananaSplitAccessAdapter(),
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
            service: BananaSplitServicing = BananaSplitService(),
            isPresented: Binding<Bool>,
            onCompletion: @escaping (BananaSplitModalView.OnCompletionAction) -> Void
        ) {
            self.seedName = seedName
            self.bananaSplitMediator = bananaSplitMediator
            self.seedsMediator = seedsMediator
            self.service = service
            self.onCompletion = onCompletion
            _isPresented = isPresented
            refreshPassphrase()
        }

        func onBackTap() {
            isPresented = false
        }

        func onCreateTap() {
            service.encrypt(
                secret: seedsMediator.getSeed(seedName: seedName),
                title: seedName,
                passphrase: recoveryPassphrase,
                totalShards: totalShardsCount,
                requiredShards: requiredShardsCount
            ) { result in
                switch result {
                case let .success(bananaSplitBackup):
                    self.bananaSplitBackup = bananaSplitBackup
                    let result = self.bananaSplitMediator.saveBananaSplit(
                        with: self.seedName,
                        bananaSplitBackup: bananaSplitBackup,
                        passphrase: .init(passphrase: self.recoveryPassphrase)
                    )
                    switch result {
                    case .success:
                        self.isPresentingQRCode = true
                    case let .failure(error):
                        self.presentableError = .alertError(message: error.localizedDescription)
                        self.isPresentingError = true
                    }
                case let .failure(error):
                    self.presentableError = .alertError(message: error.backendDisplayError)
                    self.isPresentingError = true
                }
            }
        }

        func isActionAvailable() -> Bool {
            !totalShards.isEmpty
        }

        func refreshPassphrase() {
            service.generatePassphrase(with: Constants.passphraseWords) { result in
                switch result {
                case let .success(newPassphrase):
                    self.recoveryPassphrase = newPassphrase
                case let .failure(error):
                    self.presentableError = .alertError(message: error.localizedDescription)
                    self.isPresentingError = true
                }
            }
        }

        func onQRCodeCompletion(_: BananaSplitQRCodeModalView.OnCompletionAction) {
            isPresented = false
            onCompletion(.close)
        }
    }
}

private extension BananaSplitModalView.ViewModel {
    func onSubmitTap() {
        guard isActionAvailable() else { return }
        onCreateTap()
    }

    func updateRequiredShards() {
        requiredShardsCount = totalShardsCount / 2 + 1
    }
}

#if DEBUG
    struct BananaSplitModalView_Previews: PreviewProvider {
        static var previews: some View {
            BananaSplitModalView(
                viewModel: .init(
                    seedName: "Key Set",
                    isPresented: .constant(true),
                    onCompletion: { _ in }
                )
            )
            .previewLayout(.sizeThatFits)
        }
    }
#endif
