//
//  AddDerivedKeysView.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 24/06/2023.
//

import Combine
import SwiftUI

struct AddDerivedKeysView: View {
    @StateObject var viewModel: ViewModel
    @Environment(\.safeAreaInsets) private var safeAreaInsets

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            // Navigation Bar
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    leftButtons: [.init(type: .xmark, action: viewModel.onBackTap)],
                    backgroundColor: .backgroundPrimary
                )
            )
            GeometryReader { geo in
                ScrollView(showsIndicators: false) {
                    VStack(alignment: .leading, spacing: 0) {
                        mainContent()
                        errorsSection()
                        keySets()
                        qrCodeSection()
                        infoBoxSection()
                        Spacer()
                        ActionButton(
                            action: viewModel.onMainActionTap(),
                            text: Localizable.AddDerivedKeys.Action.main.key,
                            style: .secondary()
                        )
                        .padding(Spacing.large)
                    }
                    .frame(
                        minWidth: geo.size.width,
                        minHeight: geo.size.height
                    )
                }
            }
            .background(.backgroundPrimary)
        }
        .fullScreenModal(
            isPresented: $viewModel.isPresentingError
        ) {
            ErrorBottomModal(
                viewModel: viewModel.presentableError,
                isShowingBottomAlert: $viewModel.isPresentingError
            )
            .clearModalBackground()
        }
        .fullScreenModal(
            isPresented: $viewModel.isPresentingAddKeysCancelation
        ) {
            HorizontalActionsBottomModal(
                viewModel: .cancelAddingDerivedKeys,
                mainAction: viewModel.onAddCancelationTap(),
                isShowingBottomAlert: $viewModel.isPresentingAddKeysCancelation
            )
            .clearModalBackground()
        }
    }

    @ViewBuilder
    func errorsSection() -> some View {
        LazyVStack(spacing: Spacing.extraSmall) {
            ForEach(
                viewModel.dataModel.errors,
                id: \.id
            ) {
                ActionableInfoBoxView(renderable: .init(text: $0.errorMessage), action: nil)
                    .padding(.bottom, $0 == viewModel.dataModel.errors.last ? Spacing.extraSmall : 0)
            }
        }
        .padding(.horizontal, Spacing.medium)
    }

    @ViewBuilder
    func mainContent() -> some View {
        VStack(alignment: .leading, spacing: 0) {
            Localizable.AddDerivedKeys.Label.title.text
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.titleL.font)
                .padding(.top, Spacing.extraSmall)
            Localizable.AddDerivedKeys.Label.header.text
                .foregroundColor(.textAndIconsPrimary)
                .font(PrimaryFont.bodyL.font)
                .padding(.vertical, Spacing.extraSmall)
        }
        .padding(.horizontal, Spacing.large)
        .padding(.bottom, Spacing.medium)
    }

    @ViewBuilder
    func keySets() -> some View {
        LazyVStack(spacing: 0) {
            ForEach(
                viewModel.dataModel.keySets,
                id: \.keySetName
            ) { keySet in
                LazyVStack(alignment: .leading, spacing: 0) {
                    Text(keySet.keySetName)
                        .font(PrimaryFont.titleS.font)
                        .foregroundColor(.textAndIconsPrimary)
                        .multilineTextAlignment(.leading)
                        .padding(Spacing.medium)
                    Divider()
                        .padding(.horizontal, Spacing.medium)
                    ForEach(
                        keySet.derivedKeys,
                        id: \.base58
                    ) { key in
                        derivedKey(for: key)
                        if key != keySet.derivedKeys.last {
                            Divider()
                                .padding(.horizontal, Spacing.medium)
                        }
                    }
                }
                .containerBackground()
                .padding(.bottom, Spacing.extraSmall)
            }
        }
        .padding(.horizontal, Spacing.medium)
    }

    @ViewBuilder
    func derivedKey(for key: AddDerivedKeyDerivedKeyData) -> some View {
        HStack(alignment: .center, spacing: 0) {
            NetworkIdenticon(
                identicon: key.identicon,
                network: key.network,
                background: .fill6,
                size: Heights.identiconInAddDerivedKey
            )
            .padding(.vertical, Spacing.medium)
            .padding(.trailing, Spacing.extraSmall)
            VStack(alignment: .leading, spacing: 0) {
                Text(key.path)
                    .foregroundColor(.textAndIconsTertiary)
                    .font(PrimaryFont.captionM.font)
                Spacer().frame(height: Spacing.extraExtraSmall)
                HStack(spacing: Spacing.extraExtraSmall) {
                    Text(key.base58.truncateMiddle())
                        .foregroundColor(.textAndIconsPrimary)
                        .font(PrimaryFont.bodyL.font)
                        .lineLimit(1)
                }
            }
            Spacer()
        }
        .padding(.horizontal, Spacing.medium)
    }

    @ViewBuilder
    func qrCodeSection() -> some View {
        VStack(alignment: .leading, spacing: Spacing.medium) {
            // Header
            Localizable.AddDerivedKeys.Label.qrCodeHeader.text
                .font(PrimaryFont.bodyL.font)
                .foregroundColor(.textAndIconsPrimary)
            // QR Code container
            VStack(alignment: .leading, spacing: 0) {
                AnimatedQRCodeView(
                    viewModel: Binding<AnimatedQRCodeViewModel>.constant(
                        .init(
                            qrCodes: viewModel.dataModel.qrPayload
                        )
                    )
                )
            }
            .containerBackground()
        }
        .padding(.horizontal, Spacing.large)
        .padding(.top, Spacing.large)
    }

    @ViewBuilder
    func infoBoxSection() -> some View {
        VStack(alignment: .leading, spacing: Spacing.medium) {
            // Header
            Localizable.AddDerivedKeys.Label.infoBoxHeader.text
                .font(PrimaryFont.bodyL.font)
                .foregroundColor(.textAndIconsPrimary)
            // Info Box
            ActionableInfoBoxView(renderable: .init(text: Localizable.AddDerivedKeys.Label.infobox.string), action: nil)
        }
        .padding(.horizontal, Spacing.large)
        .padding(.top, Spacing.large)
    }
}

extension AddDerivedKeysView {
    enum OnCompletionAction: Equatable {
        case onCancel
        case onDone
    }

    final class ViewModel: ObservableObject {
        private let onCompletion: (OnCompletionAction) -> Void
        private let dynamicDerivationsPreview: DdPreview
        private let derivedKeysService: CreateDerivedKeyService
        private let seedsMediator: SeedsMediating
        let dataModel: AddDerivedKeysData
        @Binding var isPresented: Bool
        @Published var isPresentingAddKeysCancelation: Bool = false
        @Published var isPresentingDerivationPath: Bool = false
        @Published var isPresentingError: Bool = false
        @Published var presentableError: ErrorBottomModalViewModel = .importDynamicDerivedKeys(content: "")

        init(
            dataModel: AddDerivedKeysData,
            dynamicDerivationsPreview: DdPreview,
            derivedKeysService: CreateDerivedKeyService = CreateDerivedKeyService(),
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator,
            isPresented: Binding<Bool>,
            onCompletion: @escaping (OnCompletionAction) -> Void
        ) {
            self.dataModel = dataModel
            self.dynamicDerivationsPreview = dynamicDerivationsPreview
            self.derivedKeysService = derivedKeysService
            self.seedsMediator = seedsMediator
            _isPresented = isPresented
            self.onCompletion = onCompletion
        }

        func onMainActionTap() {
            if dynamicDerivationsPreview.keySet.derivations.isEmpty {
                onSuccess()
            } else {
                continueCreation()
            }
        }

        func onAddCancelationTap() {
            isPresented = false
            onCompletion(.onCancel)
        }

        func onBackTap() {
            isPresentingAddKeysCancelation = true
        }

        private func onSuccess() {
            isPresented = false
            onCompletion(.onDone)
        }

        private func continueCreation() {
            derivedKeysService.createDerivedKeys(
                dynamicDerivationsPreview.keySet.seedName,
                seedsMediator.getSeed(seedName: dynamicDerivationsPreview.keySet.seedName),
                keysToImport: dynamicDerivationsPreview.keySet.derivations
            ) { [weak self] result in
                switch result {
                case .success:
                    self?.onSuccess()
                case let .failure(error):
                    self?.presentableError = .importDynamicDerivedKeys(content: error.localizedDescription)
                    self?.isPresentingError = true
                }
            }
        }
    }
}

#if DEBUG
    struct AddDerivedKeysView_Previews: PreviewProvider {
        static var previews: some View {
            AddDerivedKeysView(
                viewModel: .init(
                    dataModel: .stub,
                    dynamicDerivationsPreview: .stub,
                    isPresented: .constant(true),
                    onCompletion: { _ in }
                )
            )
            AddDerivedKeysView(
                viewModel: .init(
                    dataModel: .stubWithErrors,
                    dynamicDerivationsPreview: .stub,
                    isPresented: .constant(true),
                    onCompletion: { _ in }
                )
            )
        }
    }
#endif

extension AddDerivedKeysData {
    static let stub: AddDerivedKeysData = .init(
        errors: [],
        keySets: [
            .init(
                keySetName: "My Key Set",
                derivedKeys: [
                    .init(
                        path: "//polkadot//1",
                        base58: "1B2lb765432457SkT",
                        identicon: .stubIdenticon,
                        network: "polkadot"
                    ),
                    .init(
                        path: "//polkadot//2",
                        base58: "1iKLh365474566754ZTDE",
                        identicon: .stubIdenticon,
                        network: "polkadot"
                    ),
                    .init(
                        path: "//polkadot//3",
                        base58: "1jkCfy543654765675DOKg",
                        identicon: .stubIdenticon,
                        network: "polkadot"
                    )
                ]
            ),
            .init(
                keySetName: "Other Key Set",
                derivedKeys: [
                    .init(
                        path: "//polkadot//1",
                        base58: "1B2lb7653464235453SkT",
                        identicon: .stubIdenticon,
                        network: "polkadot"
                    )
                ]
            )
        ],
        qrPayload:
        [
            Stubs.stubQRCode
        ]
    )

    static let stubWithErrors: AddDerivedKeysData = .init(
        errors: [
            .init(
                errorMessage: """
                Some keys can not be imported until their networks are added. \
                Please add missing networks and their metadata.
                """
            ),
            .init(
                errorMessage: """
                Some are hidden from the list because they have already been imported.
                """
            )
        ],
        keySets: [
            .init(
                keySetName: "My Key Set",
                derivedKeys: [
                    .init(
                        path: "//polkadot//1",
                        base58: "1B2lb765432457SkT",
                        identicon: .stubIdenticon,
                        network: "polkadot"
                    ),
                    .init(
                        path: "//polkadot//2",
                        base58: "1iKLh365474566754ZTDE",
                        identicon: .stubIdenticon,
                        network: "polkadot"
                    ),
                    .init(
                        path: "//polkadot//3",
                        base58: "1jkCfy543654765675DOKg",
                        identicon: .stubIdenticon,
                        network: "polkadot"
                    )
                ]
            ),
            .init(
                keySetName: "Other Key Set",
                derivedKeys: [
                    .init(
                        path: "//polkadot//1",
                        base58: "1B2lb7653464235453SkT",
                        identicon: .stubIdenticon,
                        network: "polkadot"
                    )
                ]
            )
        ],
        qrPayload:
        [
            Stubs.stubQRCode
        ]
    )
}
