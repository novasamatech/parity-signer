//
//  ManageKeySetsView.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 08/09/2023.
//

import SwiftUI

struct ManageKeySetsView: View {
    private enum Constants {
        static let maxItems = 5
    }

    @StateObject var viewModel: ViewModel

    var body: some View {
        FullScreenRoundedModal(
            backgroundTapAction: viewModel.onClose,
            animateBackground: $viewModel.animateBackground,
            ignoredEdges: .bottom,
            safeAreaInsetsMode: .full,
            content: {
                VStack(spacing: 0) {
                    // Header
                    HStack {
                        Localizable.ManageKeySets.Label.header.text
                            .foregroundColor(.textAndIconsPrimary)
                            .font(PrimaryFont.titleS.font)
                        Spacer()
                        CircleButton(action: viewModel.onClose)
                    }
                    .padding(.bottom, Spacing.small)
                    // Spacer
                    Divider()
                        .padding(.vertical, Spacing.extraSmall)
                    // List
                    if viewModel.keySets.count > Constants.maxItems {
                        ScrollView {
                            itemsList()
                        }
                    } else {
                        itemsList()
                    }
                    // Spacer
                    Divider()
                        .padding(.vertical, Spacing.extraSmall)
                    // Actions
                    VStack(alignment: .leading, spacing: 0) {
                        ActionSheetCircleButton(
                            action: viewModel.onAddKeySet,
                            icon: Image(.addSmall),
                            text: Localizable.ManageKeySets.Action.add.key
                        )
                        ActionSheetCircleButton(
                            action: viewModel.onRecoverKeySet,
                            icon: Image(.recover),
                            text: Localizable.ManageKeySets.Action.recover.key
                        )
                    }
                }
                .padding(.leading, Spacing.large)
                .padding(.trailing, Spacing.medium)
                .padding(.bottom, Spacing.medium)
                .onAppear {
                    viewModel.onAppear()
                }
            }
        )
    }

    @ViewBuilder
    func item(for keySet: SeedNameCard) -> some View {
        HStack(alignment: .center, spacing: Spacing.small) {
            IdenticonView(
                identicon: keySet.identicon,
                rowHeight: Heights.identiconInManageKeySet
            )
            .padding(.vertical, Spacing.extraSmall)
            VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                Text(keySet.seedName)
                    .foregroundColor(.textAndIconsPrimary)
                    .font(PrimaryFont.titleS.font)
                if let countLabel = viewModel.countLabel(for: keySet) {
                    Text(countLabel)
                        .font(PrimaryFont.captionM.font)
                        .foregroundColor(.textAndIconsTertiary)
                        .padding(.bottom, Spacing.extraExtraSmall)
                }
            }
            Spacer()
            VStack(alignment: .center) {
                if viewModel.isSelected(keySet) {
                    Image(.checkmarkList)
                        .foregroundColor(.textAndIconsPrimary)
                        .frame(width: Heights.manageKeySetSelectionIcon, height: Heights.manageKeySetSelectionIcon)
                        .background(Circle().foregroundColor(.fill6))
                }
            }
        }
        .fixedSize(horizontal: false, vertical: true)
        .contentShape(Rectangle())
        .onTapGesture {
            viewModel.selectKeySet(keySet)
        }
    }

    @ViewBuilder
    func itemsList() -> some View {
        LazyVStack(spacing: 0) {
            ForEach(
                viewModel.keySets,
                id: \.seedName
            ) {
                item(for: $0)
            }
        }
    }
}

extension ManageKeySetsView {
    enum OnCompletionAction: Equatable {
        case onClose
        case addKeySet
        case recoverKeySet
        case viewKeySet(SeedNameCard)
    }

    final class ViewModel: ObservableObject {
        private let onCompletion: (OnCompletionAction) -> Void
        private let keyListService: KeyListService
        private let currentKeySet: String
        @Published var animateBackground: Bool = false
        @Published var keySets: [SeedNameCard] = []
        @Binding var isPresented: Bool

        init(
            isPresented: Binding<Bool>,
            currentKeySet: String,
            keyListService: KeyListService = KeyListService(),
            onCompletion: @escaping (OnCompletionAction) -> Void
        ) {
            _isPresented = isPresented
            self.currentKeySet = currentKeySet
            self.keyListService = keyListService
            self.onCompletion = onCompletion
        }

        func onAppear() {
            keyListService.getKeyList { result in
                switch result {
                case let .success(seeds):
                    self.keySets = seeds.seedNameCards
                case .failure:
                    self.keySets = []
                }
            }
        }

        func onClose() {
            dismiss(.onClose)
        }

        func onAddKeySet() {
            dismiss(.addKeySet)
        }

        func onRecoverKeySet() {
            dismiss(.recoverKeySet)
        }

        func selectKeySet(_ keySet: SeedNameCard) {
            dismiss(.viewKeySet(keySet))
        }

        func isSelected(_ keySet: SeedNameCard) -> Bool {
            currentKeySet == keySet.seedName
        }

        private func dismiss(_ action: OnCompletionAction) {
            Animations.chainAnimation(
                animateBackground.toggle(),
                delayedAnimationClosure: {
                    self.isPresented = false
                    self.onCompletion(action)
                }()
            )
        }

        func countLabel(for keySet: SeedNameCard) -> String? {
            switch keySet.derivedKeysCount {
            case 0:
                nil
            case 1:
                Localizable.ManageKeySets.Label.DerivedKeys.single(1)
            default:
                Localizable.ManageKeySets.Label.DerivedKeys.plural(keySet.derivedKeysCount)
            }
        }
    }
}

#if DEBUG
    struct ManageKeySetsView_Previews: PreviewProvider {
        static var previews: some View {
            ManageKeySetsView(
                viewModel: .init(
                    isPresented: Binding<Bool>.constant(true),
                    currentKeySet: "",
                    onCompletion: { _ in }
                )
            )
        }
    }
#endif
