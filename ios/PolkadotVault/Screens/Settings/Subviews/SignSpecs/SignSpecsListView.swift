//
//  SignSpecsListView.swift
//  PolkadotVault
//
//  Created by Krzysztof Rodak on 22/03/2023.
//

import SwiftUI

struct SignSpecsListView: View {
    @StateObject var viewModel: ViewModel
    @EnvironmentObject var navigation: NavigationCoordinator

    var body: some View {
        VStack {
            NavigationBarView(
                viewModel: NavigationBarViewModel(
                    title: Localizable.SignSpecsList.Label.title.string,
                    leftButtons: [.init(type: .arrow, action: viewModel.onBackTap)],
                    backgroundColor: Asset.backgroundPrimary.swiftUIColor
                )
            )
            ScrollView {
                LazyVStack {
                    ForEach(viewModel.content.identities, id: \.addressKey) { keyRecord in
                        rawKeyRow(keyRecord)
                            .contentShape(Rectangle())
                            .onTapGesture {
                                viewModel.onRecordTap(keyRecord)
                            }
                    }
                }
            }
            .background(Asset.backgroundPrimary.swiftUIColor)
        }
        .onAppear {
            viewModel.use(navigation: navigation)
        }
    }

    @ViewBuilder
    func rawKeyRow(_ rawKey: MRawKey) -> some View {
        HStack(alignment: .center, spacing: Spacing.small) {
            NetworkIdenticon(
                identicon: rawKey.address.identicon,
                network: rawKey.networkLogo,
                background: Asset.backgroundPrimary.swiftUIColor,
                size: Sizes.signSpecsIdenticonSize
            )
            VStack(alignment: .leading, spacing: Spacing.extraExtraSmall) {
                Text(rawKey.address.displayablePath)
                    .font(PrimaryFont.captionM.font)
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                Text(rawKey.publicKey.truncateMiddle())
                    .font(PrimaryFont.bodyL.font)
                    .foregroundColor(Asset.textAndIconsPrimary.swiftUIColor)
                Text(rawKey.address.seedName)
                    .font(PrimaryFont.bodyM.font)
                    .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
            }
            Spacer()
            Asset.chevronRight.swiftUIImage
                .foregroundColor(Asset.textAndIconsTertiary.swiftUIColor)
                .padding(Spacing.small)
        }
        .frame(height: Heights.signSpecsListRowHeight)
        .padding(.horizontal, Spacing.medium)
    }
}

extension SignSpecsListView {
    final class ViewModel: ObservableObject {
        let content: MSignSufficientCrypto
        private let seedsMediator: SeedsMediating
        private weak var navigation: NavigationCoordinator!

        init(
            content: MSignSufficientCrypto,
            seedsMediator: SeedsMediating = ServiceLocator.seedsMediator
        ) {
            self.content = content
            self.seedsMediator = seedsMediator
        }

        func use(navigation: NavigationCoordinator) {
            self.navigation = navigation
        }

        func onBackTap() {
            navigation.perform(navigation: .init(action: .goBack))
        }

        func onRecordTap(_ keyRecord: MRawKey) {
            let seedPhrase = seedsMediator.getSeed(seedName: keyRecord.address.seedName)
            guard !seedPhrase.isEmpty else { return }
            navigation.perform(
                navigation: .init(
                    action: .goForward,
                    details: keyRecord.addressKey,
                    seedPhrase: seedPhrase
                )
            )
        }
    }
}
