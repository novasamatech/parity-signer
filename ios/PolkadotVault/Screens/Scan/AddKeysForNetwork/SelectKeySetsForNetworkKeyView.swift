//
//  SelectKeySetsForNetworkKeyView.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 19/06/2023.
//

import SwiftUI

struct SelectKeySetsForNetworkKeyView: View {
    @StateObject var viewModel: ViewModel
    @State var animateBackground: Bool = false
    @Environment(\.presentationMode) var mode: Binding<PresentationMode>
    @Environment(\.safeAreaInsets) private var safeAreaInsets

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: {
                viewModel.onCancelTap()
            },
            animateBackground: $animateBackground,
            safeAreaInsetsMode: .full,
            content: {
                VStack(alignment: .leading, spacing: 0) {
                    ScrollView(showsIndicators: false) {
                        VStack(alignment: .leading, spacing: 0) {
                            mainContent()
                            seedsSelection()
                            Spacer()
                        }
                    }
                    HStack(spacing: Spacing.extraSmall) {
                        SecondaryButton(
                            action: viewModel.onCancelTap(),
                            text: Localizable.SelectKeySetsForNetworkKey.Action.cancel.key
                        )
                        PrimaryButton(
                            action: viewModel.onDoneTap,
                            text: Localizable.SelectKeySetsForNetworkKey.Action.create.key,
                            style: .primary(isDisabled: .constant(viewModel.selectedSeedNames.isEmpty))
                        )
                    }
                    .padding(.horizontal, Spacing.large)
                    .padding(.bottom, Spacing.small)
                    .padding(.top, Spacing.medium)
                }
            }
        )
        .fullScreenModal(
            isPresented: $viewModel.isPresentingError,
            onDismiss: { viewModel.onErrorDismiss?() }
        ) {
            ErrorBottomModal(
                viewModel: viewModel.errorViewModel,
                isShowingBottomAlert: $viewModel.isPresentingError
            )
            .clearModalBackground()
        }
    }

    @ViewBuilder
    func mainContent() -> some View {
        VStack(alignment: .leading, spacing: Spacing.medium) {
            Text(Localizable.SelectKeySetsForNetworkKey.Label.title(viewModel.networkName))
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.titleL.font)
            Localizable.SelectKeySetsForNetworkKey.Label.content.text
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.bodyL.font)
        }
        .padding(.horizontal, Spacing.large)
        .padding(.vertical, Spacing.medium)
    }

    @ViewBuilder
    func seedsSelection() -> some View {
        LazyVStack(spacing: 0) {
            ForEach(
                viewModel.seedNames,
                id: \.self
            ) {
                item(for: $0)
                Divider()
                    .padding(.horizontal, Spacing.medium)
            }
            selectAllSeeds()
        }
        .containerBackground()
        .padding(.horizontal, Spacing.extraSmall)
        .padding(.bottom, Spacing.medium)
    }

    @ViewBuilder
    func item(for seedName: String) -> some View {
        HStack(alignment: .center, spacing: 0) {
            Text(seedName.capitalized)
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.titleS.font)
            Spacer()
            if viewModel.isSelected(seedName) {
                Image(.checkmarkChecked)
                    .foregroundColor(.accentPink300)
            } else {
                Image(.checkmarkUnchecked)
            }
        }
        .contentShape(Rectangle())
        .padding(.horizontal, Spacing.medium)
        .frame(height: Heights.createKeysForNetworkItemHeight)
        .onTapGesture {
            viewModel.toggleSelection(seedName)
        }
    }

    @ViewBuilder
    func selectAllSeeds() -> some View {
        HStack(alignment: .center, spacing: 0) {
            Localizable.SelectKeySetsForNetworkKey.Action.selectAll.text
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.titleS.font)
            Spacer()
        }
        .contentShape(Rectangle())
        .padding(.horizontal, Spacing.medium)
        .frame(height: Heights.selectKeySetsForNetworkKeyItemHeight)
        .onTapGesture {
            viewModel.selectAllSeedNames()
        }
    }
}

extension SelectKeySetsForNetworkKeyView {
    enum OnCompletionAction: Equatable {
        case onCancel
        case onDerivedKeysCreated
    }

    final class ViewModel: ObservableObject {
        private let cancelBag = CancelBag()
        private let createKeyService: CreateDerivedKeyService
        private let seedsMediator: SeedsMediating
        let networkName: String
        var onErrorDismiss: (() -> Void)?
        private let onCompletion: (OnCompletionAction) -> Void

        @Binding var isPresented: Bool
        @Published var isPresentingDerivationPath: Bool = false
        @Published var seedNames: [String] = []
        @Published var selectedSeedNames: [String] = []
        // Error presentatation
        @Published var isPresentingError: Bool = false
        @Published var errorViewModel: ErrorBottomModalViewModel!

        init(
            networkName: String,
            createKeyService: CreateDerivedKeyService = CreateDerivedKeyService(),
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
            isPresented: Binding<Bool>,
            onCompletion: @escaping (OnCompletionAction) -> Void
        ) {
            self.networkName = networkName
            self.createKeyService = createKeyService
            self.seedsMediator = seedsMediator
            self.onCompletion = onCompletion
            _isPresented = isPresented
            updateSeedNames()
        }

        func selectAllSeedNames() {
            selectedSeedNames = seedNames
        }

        func isSelected(_ seedName: String) -> Bool {
            selectedSeedNames.contains(seedName)
        }

        func toggleSelection(_ seedName: String) {
            if selectedSeedNames.contains(seedName) {
                selectedSeedNames.removeAll { $0 == seedName }
            } else {
                selectedSeedNames.append(seedName)
            }
        }

        func onCancelTap() {
            onCompletion(.onCancel)
            isPresented = false
        }

        func onDoneTap() {
            createKeyService.createDerivedKeyForKeySets(
                selectedSeedNames,
                networkName
            ) { result in
                switch result {
                case .success:
                    self.isPresented = false
                    self.onCompletion(.onDerivedKeysCreated)
                case let .failure(error):
                    self.onErrorDismiss = { self.isPresented = false }
                    self.errorViewModel = .alertError(message: error.localizedDescription)
                    self.isPresentingError = true
                }
            }
        }
    }
}

private extension SelectKeySetsForNetworkKeyView.ViewModel {
    func updateSeedNames() {
        seedNames = seedsMediator.seedNames
    }
}

#if DEBUG
    struct SelectKeySetsForNetworkKeyView_Previews: PreviewProvider {
        static var previews: some View {
            SelectKeySetsForNetworkKeyView(
                viewModel: .init(
                    networkName: "networkName",
                    isPresented: .constant(true),
                    onCompletion: { _ in }
                )
            )
        }
    }
#endif
