//
//  AddDerivedKeysView.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 24/06/2023.
//

import Combine
import SwiftUI

struct AddDerivedKeyDerivedKeyData: Equatable {
    let base58: String
    let identicon: SignerImage
    let network: String
}

struct AddDerivedKeyKeySetData: Equatable {
    let keySetName: String
    let derivedKeys: [AddDerivedKeyDerivedKeyData]
}

struct AddDerivedKeysData: Equatable {
    let keySets: [AddDerivedKeyKeySetData]
    let qrPayload: [[UInt8]]
}

struct AddDerivedKeysView: View {
    @StateObject var viewModel: ViewModel
    @Environment(\.safeAreaInsets) private var safeAreaInsets

    var body: some View {
        VStack(alignment: .leading, spacing: 0) {
            // Navigation Bar
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    leftButtons: [.init(type: .arrow, action: viewModel.onBackTap)],
                    backgroundColor: Asset.backgroundPrimary.swiftUIColor
                )
            )
            GeometryReader { geo in
                ScrollView(showsIndicators: false) {
                    VStack(alignment: .leading, spacing: 0) {
                        mainContent()
                        keySets()
                        qrCodeFooter()
                        Spacer()
                        SecondaryButton(
                            action: viewModel.onDoneTap(),
                            text: Localizable.AddDerivedKeys.Action.done.key,
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
            .background(Asset.backgroundPrimary.swiftUIColor)
        }
    }

    @ViewBuilder
    func mainContent() -> some View {
        VStack(alignment: .leading, spacing: 0) {
            Localizable.AddDerivedKeys.Label.title.text
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.titleL.font)
                .padding(.top, Spacing.extraSmall)
            Localizable.AddDerivedKeys.Label.header.text
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
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
                        .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
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
                background: Asset.fill6.swiftUIColor,
                size: Heights.identiconInAddDerivedKey
            )
            .padding(.vertical, Spacing.medium)
            .padding(.trailing, Spacing.extraSmall)
            Text(key.base58.truncateMiddle())
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                .font(PrimaryFont.bodyL.font)
            Spacer()
        }
        .padding(.horizontal, Spacing.medium)
    }

    @ViewBuilder
    func qrCodeFooter() -> some View {
        VStack(alignment: .leading, spacing: Spacing.medium) {
            // Header
            Localizable.AddDerivedKeys.Label.footer.text
                .font(PrimaryFont.bodyL.font)
                .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
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
}

extension AddDerivedKeysView {
    enum OnCompletionAction: Equatable {}

    final class ViewModel: ObservableObject {
        var dataModel: AddDerivedKeysData! = .stub
        private let onCompletion: (OnCompletionAction) -> Void
        var onErrorDismiss: (() -> Void)?

        @Binding var isPresented: Bool
        @Published var isPresentingDerivationPath: Bool = false
        @Published var keySets: [MmNetwork] = []

        init(
            isPresented: Binding<Bool>,
            onCompletion: @escaping (OnCompletionAction) -> Void
        ) {
            _isPresented = isPresented
            self.onCompletion = onCompletion
        }

        func onDoneTap() {}

        func onBackTap() {
            isPresented = false
        }
    }
}

#if DEBUG
    struct AddDerivedKeysView_Previews: PreviewProvider {
        static var previews: some View {
            AddDerivedKeysView(
                viewModel: .init(
                    isPresented: .constant(true),
                    onCompletion: { _ in }
                )
            )
        }
    }
#endif

extension AddDerivedKeysData {
    static let stub: AddDerivedKeysData = .init(
        keySets: [
            .init(
                keySetName: "My Key Set",
                derivedKeys: [
                    .init(
                        base58: "1B2lb765432457SkT",
                        identicon: .stubIdenticon,
                        network: "polkadot"
                    ),
                    .init(
                        base58: "1iKLh365474566754ZTDE",
                        identicon: .stubIdenticon,
                        network: "polkadot"
                    ),
                    .init(
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
